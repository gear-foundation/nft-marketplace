#![no_std]
use gstd::{
    collections::{HashMap, HashSet},
    debug, exec, msg,
    prelude::*,
    ActorId,
};
use nft_io::{
    Action, Config, ImageData, Nft, NftAction, NftError, NftEvent, NftId, NftInit, NftState,
    StateQuery, StateReply, BLOCK_DURATION_IN_SECS, EXISTENTIAL_DEPOSIT, GAS_AUTO_CHANGING,
};

#[derive(Debug)]
struct NftContract {
    pub tokens: HashMap<NftId, Nft>,
    pub owners: HashMap<ActorId, HashSet<NftId>>,
    pub restriction_mint: HashMap<ActorId, u32>, // made in order to track the number of mint!(not transfer) nft
    pub token_approvals: HashMap<NftId, ActorId>,
    pub config: Config,
    pub nonce: NftId,
    pub img_links_and_data: Vec<(String, ImageData)>,
    pub collection_owner: ActorId,
}
static mut NFT_CONTRACT: Option<NftContract> = None;

impl NftContract {
    fn mint(&mut self) -> Result<NftEvent, NftError> {
        let msg_src = msg::source();
        self.check_available_amount_of_tokens()?;
        self.check_mint_limit(&msg_src)?;
        let Some(next_nft_nonce) = self.nonce.checked_add(1) else {
            return Err(NftError("Math overflow.".to_owned()));
        };
        self.payment_for_mint()?;

        let rand_index = get_random_value(self.img_links_and_data.len() as u64);
        let img_link: String;
        let token_id = self.nonce;

        if let Some((link, img_info)) = self.img_links_and_data.get_mut(rand_index as usize) {
            if let Some(limit) = img_info.limit_copies.as_mut() {
                *limit -= 1;
            }
            img_link = link.clone();

            if let Some(auto_changing_rules) = &img_info.auto_changing_rules {
                auto_changing_rules
                    .iter()
                    .for_each(|(trigger_time, auto_action)| {
                        let action = match &auto_action {
                            Action::ChangeImg(img_link) => NftAction::ChangeImg {
                                token_id,
                                img_link: img_link.to_string(),
                            },
                            Action::AddMeta(metadata) => NftAction::AddMetadata {
                                token_id,
                                metadata: metadata.to_string(),
                            },
                        };
                        msg::send_with_gas_delayed(
                            exec::program_id(),
                            action,
                            GAS_AUTO_CHANGING,
                            0,
                            *trigger_time / BLOCK_DURATION_IN_SECS,
                        )
                        .expect("Error in sending a delayed message `NftAction`");
                    });
            }

            if let Some(0) = img_info.limit_copies {
                self.img_links_and_data.remove(rand_index as usize);
            }
        } else {
            return Err(NftError("Error with getting a random nft".to_owned()));
        }

        self.owners
            .entry(msg_src)
            .and_modify(|ids| {
                ids.insert(token_id);
            })
            .or_insert_with(|| HashSet::from([token_id]));

        let name = format!("{} - {}", self.config.name, token_id);
        let nft_data = Nft {
            owner: msg_src,
            name,
            description: self.config.description.clone(),
            metadata: vec![],
            media_url: img_link.clone(),
            mint_time: exec::block_timestamp(),
        };
        self.nonce = next_nft_nonce;
        self.tokens.insert(token_id, nft_data.clone());
        self.restriction_mint
            .entry(msg_src)
            .and_modify(|ids| {
                *ids += 1;
            })
            .or_insert(1);

        Ok(NftEvent::Minted { token_id, nft_data })
    }
    fn transfer_from(
        &mut self,
        from: &ActorId,
        to: &ActorId,
        token_id: NftId,
    ) -> Result<NftEvent, NftError> {
        self.can_transfer(from, to, &token_id)?;

        let nft = self
            .tokens
            .get_mut(&token_id)
            .expect("NonFungibleToken: token does not exist");

        if let Some(tokens) = self.owners.get_mut(from) {
            tokens.retain(|&token| token != token_id);
            if tokens.is_empty() {
                self.owners.remove(from);
            }
        } else {
            return Err(NftError("Fatal: owner does not contain nft id.".to_owned()));
        }

        nft.owner = *to;

        self.owners
            .entry(*to)
            .and_modify(|ids| {
                ids.insert(token_id);
            })
            .or_insert_with(|| HashSet::from([token_id]));

        self.token_approvals.remove(&token_id);

        Ok(NftEvent::Transferred {
            owner: *from,
            recipient: *to,
            token_id,
        })
    }
    fn approve(&mut self, to: &ActorId, token_id: NftId) -> Result<NftEvent, NftError> {
        self.can_approve(&token_id)?;
        self.token_approvals.insert(token_id, *to);

        Ok(NftEvent::Approved { to: *to, token_id })
    }

    fn revoke_approve(&mut self, token_id: NftId) -> Result<NftEvent, NftError> {
        if let Some(nft_info) = self.tokens.get(&token_id) {
            if nft_info.owner != msg::source() {
                return Err(NftError("Only nft owner can send this message".to_owned()));
            }
        } else {
            return Err(NftError("This nft id hasn't come out yet".to_owned()));
        }

        let res = self.token_approvals.remove(&token_id);
        if res.is_none() {
            return Err(NftError(
                "No approve has been issued to this token".to_owned(),
            ));
        }
        Ok(NftEvent::ApprovalRevoked { token_id })
    }

    fn get_token_info(&self, token_id: NftId) -> Result<NftEvent, NftError> {
        let nft = self.tokens.get(&token_id);
        let (token_owner, can_sell) = if let Some(nft) = nft {
            let can_sell = if let Some(time) = self.config.sellable {
                exec::block_timestamp() >= nft.mint_time + time
            } else {
                false
            };
            (nft.owner, can_sell)
        } else {
            return Err(NftError(
                "NonFungibleToken: token does not exist".to_owned(),
            ));
        };
        let approval = self.token_approvals.get(&token_id).copied();

        Ok(NftEvent::TokenInfoReceived {
            token_owner,
            approval,
            sellable: can_sell,
            collection_owner: self.collection_owner,
            royalty: self.config.royalty,
        })
    }
    fn expand(&mut self, additional_links: Vec<(String, ImageData)>) -> Result<NftEvent, NftError> {
        self.check_collection_owner()?;
        if additional_links
            .iter()
            .any(|(_, img_data)| img_data.limit_copies.map_or(false, |limit| limit == 0))
        {
            return Err(NftError("Limit of copies value is equal to 0".to_owned()));
        }

        self.img_links_and_data.extend(additional_links.clone());

        Ok(NftEvent::Expanded { additional_links })
    }

    fn change_config(&mut self, config: Config) -> Result<NftEvent, NftError> {
        self.check_collection_owner()?;

        if !self.tokens.is_empty() {
            return Err(NftError(
                "The collection configuration can no more be changed".to_owned(),
            ));
        }

        // made 10_000 so you can enter hundredths of a percent.
        if config.royalty > 10_000 {
            return Err(NftError(
                "Royalty percent must be less than 100%".to_owned(),
            ));
        }
        if config.transferable.is_none() && config.sellable.is_some() {
            return Err(NftError("Tokens must be transferable".to_owned()));
        }

        if let Some(limit) = config.user_mint_limit {
            if limit == 0 {
                return Err(NftError(
                    "The mint limit must be greater than zero".to_owned(),
                ));
            }
        }
        self.config = config.clone();
        Ok(NftEvent::ConfigChanged { config })
    }

    fn change_image(&mut self, token_id: NftId, img_link: String) -> Result<NftEvent, NftError> {
        if exec::program_id() != msg::source() {
            return Err(NftError("Only program can send this message".to_owned()));
        }
        let nft = self
            .tokens
            .get_mut(&token_id)
            .expect("Token does not exist");

        nft.media_url = img_link.clone();

        msg::send(
            nft.owner,
            NftEvent::ImageChanged {
                token_id,
                img_link: img_link.clone(),
            },
            0,
        )
        .expect("Error in sending NftEvent::ImageChanged");
        Ok(NftEvent::ImageChanged { token_id, img_link })
    }

    fn add_metadata(&mut self, token_id: NftId, metadata: String) -> Result<NftEvent, NftError> {
        if exec::program_id() != msg::source() {
            return Err(NftError("Only program can send this message".to_owned()));
        }
        let nft = self
            .tokens
            .get_mut(&token_id)
            .expect("Token does not exist");

        nft.metadata.push(metadata.clone());
        msg::send(
            nft.owner,
            NftEvent::MetadataAdded {
                token_id,
                metadata: metadata.clone(),
            },
            0,
        )
        .expect("Error in sending NftEvent::MetadataAdded");
        Ok(NftEvent::MetadataAdded { token_id, metadata })
    }

    fn can_delete(&self) -> Result<NftEvent, NftError> {
        Ok(NftEvent::CanDelete(self.tokens.is_empty()))
    }

    fn check_collection_owner(&self) -> Result<(), NftError> {
        if self.collection_owner != msg::source() {
            return Err(NftError(
                "Only collection owner can send this message".to_owned(),
            ));
        }
        Ok(())
    }

    fn check_available_amount_of_tokens(&self) -> Result<(), NftError> {
        if self.img_links_and_data.is_empty() {
            return Err(NftError("All tokens are minted.".to_owned()));
        }
        Ok(())
    }
    fn check_mint_limit(&self, user: &ActorId) -> Result<(), NftError> {
        if let Some(limit) = self.config.user_mint_limit {
            if let Some(number_tokens) = self.restriction_mint.get(user) {
                if number_tokens >= &limit && self.collection_owner != *user {
                    return Err(NftError("You've exhausted your limit.".to_owned()));
                }
            }
        }
        Ok(())
    }

    // Checking for sufficient mint value and sending the value to the creator
    fn payment_for_mint(&self) -> Result<(), NftError> {
        if self.config.payment_for_mint != 0 {
            if msg::value() != self.config.payment_for_mint {
                return Err(NftError("Incorrectly entered mint fee.".to_owned()));
            }
            // use send_with_gas to transfer the value directly to the balance, not to the mailbox.
            msg::send_with_gas(self.collection_owner, "", 0, self.config.payment_for_mint)
                .expect("Error in sending value");
        }

        Ok(())
    }
    // verification of approval
    fn can_approve(&self, token_id: &NftId) -> Result<(), NftError> {
        if self.token_approvals.contains_key(token_id) {
            return Err(NftError("Approve has already been issued".to_owned()));
        }
        if let Some(nft_info) = self.tokens.get(token_id) {
            if nft_info.owner != msg::source() {
                return Err(NftError("Only nft owner can send this message".to_owned()));
            }
        } else {
            return Err(NftError("This nft id hasn't come out yet".to_owned()));
        }

        Ok(())
    }

    // approval check
    fn check_approve(&self, user: &ActorId, token_id: &NftId) -> Result<(), NftError> {
        if let Some(approved_account) = self.token_approvals.get(token_id) {
            if approved_account != user {
                return Err(NftError(
                    "Caller is not approved to perform transfer.".to_owned(),
                ));
            }
        } else {
            return Err(NftError("Caller is not approved to perform transfer.".to_owned()));
        }
        Ok(())
    }

    // doing all the checks to verify that the transfer can be made
    fn can_transfer(&self, from: &ActorId, to: &ActorId, token_id: &NftId) -> Result<(), NftError> {
        if let Some(nft) = self.tokens.get(token_id) {
            let owner = nft.owner;
            if owner != *from {
                return Err(NftError("NonFungibleToken: access denied".to_owned()));
            }
            let msg_src = msg::source();
            // if the owner of the token does not match the sender of the message, then check the approval
            if owner != msg_src {
                self.check_approve(&msg_src, token_id)?;
            }
            if let Some(time) = self.config.transferable {
                if exec::block_timestamp() < nft.mint_time + time {
                    return Err(NftError(
                        "NonFungibleToken: transfer will be available after the deadline"
                            .to_owned(),
                    ));
                }
            } else {
                return Err(NftError(
                    "NonFungibleToken: token is not transferable".to_owned(),
                ));
            }
        } else {
            return Err(NftError(
                "NonFungibleToken: token does not exist".to_owned(),
            ));
        }

        if from == to {
            return Err(NftError("Self transfer is not allowed.".to_owned()));
        }
        Ok(())
    }
}

#[no_mangle]
extern "C" fn init() {
    let NftInit {
        collection_owner,
        config,
        img_links_and_data,
    } = msg::load().expect("Unable to decode `NftInit`.");
    debug!("INIT NFT");

    assert!(
        config
            .user_mint_limit
            .map(|limit| limit > 0)
            .unwrap_or(true),
        "The mint limit must be greater than zero"
    );

    if config.payment_for_mint > 0 && config.payment_for_mint < EXISTENTIAL_DEPOSIT {
        panic!(
            "{}",
            format!(
                "The payment for mint must be greater than existential deposit ({})",
                EXISTENTIAL_DEPOSIT
            )
        );
    }

    // made 10_000 so hundredths of a percent could be entered.
    if config.royalty > 10_000 {
        panic!("Royalty percent must be less than 100%");
    }
    // can't be made sellable but not transferable.
    if config.transferable.is_none() && config.sellable.is_some() {
        panic!("Tokens must be transferable");
    }
    // at least one image in the collection is a must have.
    assert!(
        !img_links_and_data.is_empty(),
        "There must be at least one link to create a collection"
    );
    // сan't have zero copies of the nft.
    if img_links_and_data
        .iter()
        .any(|(_, img_data)| img_data.limit_copies.map_or(false, |limit| limit == 0))
    {
        panic!("Limit of copies value is equal to 0");
    }

    unsafe {
        NFT_CONTRACT = Some(NftContract {
            tokens: HashMap::new(),
            owners: HashMap::new(),
            token_approvals: HashMap::new(),
            restriction_mint: HashMap::new(),
            config: config.clone(),
            nonce: 0,
            img_links_and_data,
            collection_owner,
        })
    };
    msg::send(
        collection_owner,
        NftEvent::Initialized {
            config: config.clone(),
        },
        0,
    )
    .expect("Error during send to owner `NftEvent::Initialized`");
    msg::reply(NftEvent::Initialized { config }, 0)
        .expect("Error during send reply `NftEvent::Initialized`");
}

#[no_mangle]
extern "C" fn handle() {
    let action: NftAction = msg::load().expect("Could not load `NftAction`.");
    let nft_contract: &mut NftContract = unsafe {
        NFT_CONTRACT
            .as_mut()
            .expect("Unexpected uninitialized `NftContract`.")
    };

    let result = match action {
        NftAction::Mint => nft_contract.mint(),
        NftAction::TransferFrom { from, to, token_id } => {
            nft_contract.transfer_from(&from, &to, token_id)
        }
        NftAction::Transfer { to, token_id } => {
            nft_contract.transfer_from(&msg::source(), &to, token_id)
        }
        NftAction::Approve { to, token_id } => nft_contract.approve(&to, token_id),
        NftAction::RevokeApproval { token_id } => nft_contract.revoke_approve(token_id),
        NftAction::Expand { additional_links } => nft_contract.expand(additional_links),
        NftAction::ChangeConfig { config } => nft_contract.change_config(config),
        NftAction::GetTokenInfo { token_id } => nft_contract.get_token_info(token_id),
        NftAction::CanDelete => nft_contract.can_delete(),
        NftAction::ChangeImg { token_id, img_link } => {
            nft_contract.change_image(token_id, img_link)
        }
        NftAction::AddMetadata { token_id, metadata } => {
            nft_contract.add_metadata(token_id, metadata)
        }
    };

    msg::reply(result, 0).expect("Failed to encode or reply with `Result<NftEvent, NftError>`.");
}

#[no_mangle]
extern "C" fn state() {
    let nft = unsafe {
        NFT_CONTRACT
            .take()
            .expect("Unexpected: The contract is not initialized")
    };
    let query: StateQuery = msg::load().expect("Unable to load the state query");
    let reply = match query {
        StateQuery::Name => StateReply::Name(nft.config.name),
        StateQuery::Description => StateReply::Description(nft.config.description),
        StateQuery::Config => StateReply::Config(nft.config),
        StateQuery::All => {
            let nft_state: NftState = nft.into();
            StateReply::All(nft_state)
        }
    };
    msg::reply(reply, 0).expect("Unable to share state");
}

impl From<NftContract> for NftState {
    fn from(value: NftContract) -> Self {
        let NftContract {
            tokens,
            owners,
            token_approvals,
            config,
            nonce,
            img_links_and_data,
            collection_owner,
            ..
        } = value;

        let tokens = tokens
            .iter()
            .map(|(nft_id, nft)| (*nft_id, nft.clone()))
            .collect();
        let owners = owners
            .iter()
            .map(|(actor_id, token_set)| (*actor_id, token_set.iter().copied().collect()))
            .collect();
        let token_approvals = token_approvals
            .iter()
            .map(|(nft_id, actor_id)| (*nft_id, *actor_id))
            .collect();

        Self {
            tokens,
            owners,
            token_approvals,
            config,
            nonce,
            img_links_and_data,
            collection_owner,
        }
    }
}

static mut SEED: u8 = 0;

pub fn get_random_value(range: u64) -> u64 {
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let mut random_input: [u8; 32] = exec::program_id().into();
    let block_time = exec::block_timestamp() as u8;
    random_input[0] = random_input[0].wrapping_add(seed).wrapping_add(block_time);
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    let mut result: u64 = 0;

    for &byte in &random {
        result = (result << 8) | u64::from(byte);
    }

    result % range
}
