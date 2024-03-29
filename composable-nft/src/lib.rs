#![no_std]
use composable_nft_io::*;
use gstd::{
    collections::{HashMap, HashSet},
    debug, exec, msg,
    prelude::*,
    ActorId,
};

#[derive(Debug)]
struct ComposableNftContract {
    pub tokens: HashMap<NftId, Nft>,
    pub owners: HashMap<ActorId, HashSet<NftId>>,
    pub restriction_mint: HashMap<ActorId, u32>,
    pub token_approvals: HashMap<NftId, ActorId>,
    pub config: Config,
    pub nonce: NftId,
    pub img_links: Vec<Vec<String>>,
    pub combinations: HashSet<Vec<u8>>,
    pub collection_owner: ActorId,
    pub number_combination: u64,
}
static mut NFT_CONTRACT: Option<ComposableNftContract> = None;

impl ComposableNftContract {
    fn mint(&mut self, combination: Vec<u8>) -> Result<ComposableNftEvent, ComposableNftError> {
        let msg_src = msg::source();
        self.check_combination(&combination)?;
        self.check_limit()?;

        let Some(next_nft_nonce) = self.nonce.checked_add(1) else {
            return Err(ComposableNftError("Math overflow.".to_owned()));
        };
        self.payment_for_mint()?;

        let token_id = self.nonce;
        self.nonce = next_nft_nonce;

        self.owners
            .entry(msg_src)
            .and_modify(|ids| {
                ids.insert(token_id);
            })
            .or_insert_with(|| HashSet::from([token_id]));

        let name = format!("{} - {}", self.config.name, token_id);

        let media_url: Vec<String> = combination
            .iter()
            .enumerate()
            .map(|(index, &comb_value)| self.img_links[index][comb_value as usize].clone())
            .collect();

        let nft_data = Nft {
            owner: msg_src,
            name,
            description: self.config.description.clone(),
            metadata: vec![],
            media_url: media_url.clone(),
            mint_time: exec::block_timestamp(),
        };
        self.tokens.insert(token_id, nft_data.clone());
        self.restriction_mint
            .entry(msg_src)
            .and_modify(|ids| {
                *ids += 1;
            })
            .or_insert(1);

        let mut current_combination = Vec::with_capacity(combination.len());
        current_combination.extend_from_slice(&combination);
        self.combinations.insert(current_combination);

        Ok(ComposableNftEvent::Minted { token_id, nft_data })
    }
    fn transfer_from(
        &mut self,
        from: &ActorId,
        to: &ActorId,
        token_id: NftId,
    ) -> Result<ComposableNftEvent, ComposableNftError> {
        self.can_transfer(from, to, &token_id)?;

        let nft = self
            .tokens
            .get_mut(&token_id)
            .expect("ComposableNftToken: token does not exist");

        nft.owner = *to;

        if let Some(tokens) = self.owners.get_mut(from) {
            tokens.retain(|&token| token != token_id);
            if tokens.is_empty() {
                self.owners.remove(from);
            }
        } else {
            return Err(ComposableNftError(
                "Fatal: owner does not contain nft id.".to_owned(),
            ));
        }

        self.owners
            .entry(*to)
            .and_modify(|ids| {
                ids.insert(token_id);
            })
            .or_insert_with(|| HashSet::from([token_id]));

        self.token_approvals.remove(&token_id);

        Ok(ComposableNftEvent::Transferred {
            owner: *from,
            recipient: *to,
            token_id,
        })
    }

    fn approve(
        &mut self,
        to: &ActorId,
        token_id: NftId,
    ) -> Result<ComposableNftEvent, ComposableNftError> {
        self.can_approve(&token_id)?;
        self.token_approvals.insert(token_id, *to);

        Ok(ComposableNftEvent::Approved {
            account: *to,
            token_id,
        })
    }

    fn revoke_approve(
        &mut self,
        token_id: NftId,
    ) -> Result<ComposableNftEvent, ComposableNftError> {
        if let Some(nft_info) = self.tokens.get(&token_id) {
            if nft_info.owner != msg::source() {
                return Err(ComposableNftError(
                    "Only nft owner can send this message".to_owned(),
                ));
            }
        } else {
            return Err(ComposableNftError(
                "This nft id hasn't come out yet".to_owned(),
            ));
        }

        let res = self.token_approvals.remove(&token_id);
        if res.is_none() {
            return Err(ComposableNftError(
                "No approve has been issued to this token".to_owned(),
            ));
        }
        Ok(ComposableNftEvent::ApprovalRevoked { token_id })
    }

    fn change_config(&mut self, config: Config) -> Result<ComposableNftEvent, ComposableNftError> {
        self.check_collection_owner()?;

        if !self.tokens.is_empty() {
            return Err(ComposableNftError(
                "The collection configuration can no more be changed".to_owned(),
            ));
        }
        // made 10_000 so you can enter hundredths of a percent.
        if config.royalty > 10_000 {
            return Err(ComposableNftError(
                "Royalty percent must be less than 100%".to_owned(),
            ));
        }
        if config.transferable.is_none() && config.sellable.is_some() {
            return Err(ComposableNftError("Tokens must be transferable".to_owned()));
        }
        if let Some(limit) = config.tokens_limit {
            if limit > self.number_combination {
                return Err(ComposableNftError(
                    "Exceeds the number of possible combinations".to_owned(),
                ));
            }
        }

        self.config = config.clone();

        Ok(ComposableNftEvent::ConfigChanged { config })
    }
    fn get_token_info(&self, token_id: NftId) -> Result<ComposableNftEvent, ComposableNftError> {
        let nft = self.tokens.get(&token_id);
        let (token_owner, can_sell) = if let Some(nft) = nft {
            let can_sell = if let Some(time) = self.config.sellable {
                exec::block_timestamp() >= nft.mint_time + time
            } else {
                false
            };
            (nft.owner, can_sell)
        } else {
            return Err(ComposableNftError(
                "ComposableNft: token does not exist".to_owned(),
            ));
        };
        let approval = self.token_approvals.get(&token_id).copied();

        Ok(ComposableNftEvent::TokenInfoReceived {
            token_owner,
            approval,
            sellable: can_sell,
            collection_owner: self.collection_owner,
            royalty: self.config.royalty,
        })
    }

    fn payment_for_mint(&self) -> Result<(), ComposableNftError> {
        if self.config.payment_for_mint != 0 {
            if msg::value() != self.config.payment_for_mint {
                return Err(ComposableNftError(
                    "Incorrectly entered mint fee .".to_owned(),
                ));
            }
            // use send_with_gas to transfer the value directly to the balance, not to the mailbox.
            msg::send_with_gas(self.collection_owner, "", 0, self.config.payment_for_mint)
                .expect("Error in sending value");
        }

        Ok(())
    }

    fn can_delete(&self) -> Result<ComposableNftEvent, ComposableNftError> {
        Ok(ComposableNftEvent::CanDelete(self.tokens.is_empty()))
    }

    fn can_approve(&self, token_id: &NftId) -> Result<(), ComposableNftError> {
        if self.token_approvals.contains_key(token_id) {
            return Err(ComposableNftError(
                "Approve has already been issued".to_owned(),
            ));
        }
        if let Some(nft_info) = self.tokens.get(token_id) {
            if nft_info.owner != msg::source() {
                return Err(ComposableNftError(
                    "Only nft owner can send this message".to_owned(),
                ));
            }
        } else {
            return Err(ComposableNftError(
                "This nft id hasn't come out yet".to_owned(),
            ));
        }

        Ok(())
    }
    fn check_collection_owner(&self) -> Result<(), ComposableNftError> {
        if self.collection_owner != msg::source() {
            return Err(ComposableNftError(
                "Only collection owner can send this message".to_owned(),
            ));
        }
        Ok(())
    }
    fn check_combination(&self, combination: &Vec<u8>) -> Result<(), ComposableNftError> {
        if self.combinations.contains(combination) {
            return Err(ComposableNftError(
                "This combination already exists".to_owned(),
            ));
        }
        if combination.len() != self.img_links.len() {
            return Err(ComposableNftError(
                "Incorrectly entered combination: wrong combination length".to_owned(),
            ));
        }
        if combination
            .iter()
            .zip(self.img_links.iter())
            .any(|(comb_value, inner_vec)| *comb_value >= inner_vec.len() as u8)
        {
            return Err(ComposableNftError(
                "Incorrectly entered combination: out of bounds".to_owned(),
            ));
        }
        Ok(())
    }
    fn check_limit(&self) -> Result<(), ComposableNftError> {
        let msg_src = msg::source();
        if let Some(limit) = self.config.tokens_limit {
            if self.nonce >= limit {
                return Err(ComposableNftError("All tokens are minted.".to_owned()));
            }
        } else if self.nonce >= self.number_combination {
            return Err(ComposableNftError("All tokens are minted.".to_owned()));
        }

        if let Some(limit) = self.config.user_mint_limit {
            if let Some(number_tokens) = self.restriction_mint.get(&msg_src) {
                if number_tokens >= &limit && self.collection_owner != msg::source() {
                    return Err(ComposableNftError(
                        "You've exhausted your limit.".to_owned(),
                    ));
                }
            }
        }
        Ok(())
    }
    fn check_approve(&self, user: &ActorId, token_id: &NftId) -> Result<(), ComposableNftError> {
        if let Some(approved_account) = self.token_approvals.get(token_id) {
            if approved_account != user {
                return Err(ComposableNftError(
                    "Caller is not approved to perform transfer.".to_owned(),
                ));
            }
        } else {
            return Err(ComposableNftError(
                "Target token_approvals is empty.".to_owned(),
            ));
        }
        Ok(())
    }

    fn can_transfer(
        &self,
        from: &ActorId,
        to: &ActorId,
        token_id: &NftId,
    ) -> Result<(), ComposableNftError> {
        if let Some(nft) = self.tokens.get(token_id) {
            let owner = nft.owner;
            if owner != *from {
                return Err(ComposableNftError(
                    "ComposableNftToken: access denied".to_owned(),
                ));
            }
            let msg_src = msg::source();
            if owner != msg_src {
                self.check_approve(&msg_src, token_id)?;
            }
            if let Some(time) = self.config.transferable {
                if exec::block_timestamp() < nft.mint_time + time {
                    return Err(ComposableNftError(
                        "ComposableNft: transfer will be available after the deadline".to_owned(),
                    ));
                }
            } else {
                return Err(ComposableNftError(
                    "ComposableNft: token is not transferable".to_owned(),
                ));
            }
        } else {
            return Err(ComposableNftError(
                "ComposableNftToken: token does not exist".to_owned(),
            ));
        }

        if from == to {
            return Err(ComposableNftError(
                "Self transfer is not allowed.".to_owned(),
            ));
        }
        Ok(())
    }
}

#[no_mangle]
extern "C" fn init() {
    let ComposableNftInit {
        owner,
        config,
        img_links,
    } = msg::load().expect("Unable to decode `ComposableNftInit`.");

    debug!("INIT COMPOSABLE NFT");
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

    // made 10_000 so you can enter hundredths of a percent.
    if config.royalty > 10_000 {
        panic!("Royalty percent must be less than 100%");
    }
    assert!(
        config.tokens_limit.map(|limit| limit > 0).unwrap_or(true),
        "The tokens limit must be greater than zero"
    );

    let number_combination: u64 = img_links
        .iter()
        .map(|inner_vec| inner_vec.len() as u64)
        .product();

    assert!(
        config
            .tokens_limit
            .map(|limit| limit <= number_combination)
            .unwrap_or(true),
        "The tokens limit must be greater than zero"
    );

    unsafe {
        NFT_CONTRACT = Some(ComposableNftContract {
            tokens: HashMap::new(),
            owners: HashMap::new(),
            token_approvals: HashMap::new(),
            restriction_mint: HashMap::new(),
            config: config.clone(),
            nonce: 0,
            img_links,
            combinations: HashSet::new(),
            collection_owner: owner,
            number_combination,
        })
    };
    msg::send(
        owner,
        Ok::<ComposableNftEvent, ComposableNftError>(ComposableNftEvent::Initialized {
            config: config.clone(),
        }),
        0,
    )
    .expect("Error during send to owner `ComposableNftEvent::Initialized`");

    msg::reply(ComposableNftEvent::Initialized { config }, 0)
        .expect("Error during replying with `ComposableNftEvent::Initialized`");
}

#[no_mangle]
extern "C" fn handle() {
    let action: ComposableNftAction = msg::load().expect("Could not load `ComposableNftAction`.");
    let nft_contract: &mut ComposableNftContract = unsafe {
        NFT_CONTRACT
            .as_mut()
            .expect("Unexpected uninitialized `ComposableNftContract`.")
    };

    let result = match action {
        ComposableNftAction::Mint { combination } => nft_contract.mint(combination),
        ComposableNftAction::TransferFrom { from, to, token_id } => {
            nft_contract.transfer_from(&from, &to, token_id)
        }
        ComposableNftAction::Transfer { to, token_id } => {
            nft_contract.transfer_from(&msg::source(), &to, token_id)
        }
        ComposableNftAction::Approve { to, token_id } => nft_contract.approve(&to, token_id),
        ComposableNftAction::RevokeApproval { token_id } => nft_contract.revoke_approve(token_id),
        ComposableNftAction::ChangeConfig { config } => nft_contract.change_config(config),
        ComposableNftAction::GetTokenInfo { token_id } => nft_contract.get_token_info(token_id),
        ComposableNftAction::CanDelete => nft_contract.can_delete(),
    };

    msg::reply(result, 0).expect("Failed to encode or reply with `StudentNftEvent`.");
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
            let nft_state: ComposableNftState = nft.into();
            StateReply::All(nft_state)
        }
    };
    msg::reply(reply, 0).expect("Unable to share state");
}

impl From<ComposableNftContract> for ComposableNftState {
    fn from(value: ComposableNftContract) -> Self {
        let ComposableNftContract {
            tokens,
            owners,
            token_approvals,
            config,
            nonce,
            collection_owner,
            img_links,
            restriction_mint,
            number_combination,
            ..
            // combinations,
            // number_combination
        } = value;

        let tokens = tokens
            .iter()
            .map(|(nft_id, nft)| (*nft_id, nft.clone()))
            .collect();
        let owners = owners
            .iter()
            .map(|(k, v)| (*k, v.iter().copied().collect()))
            .collect();
        let token_approvals = token_approvals
            .iter()
            .map(|(nft_id, actor_id)| (*nft_id, *actor_id))
            .collect();
        let restriction_mint = restriction_mint
            .iter()
            .map(|(id, number)| (*id, *number))
            .collect();

        Self {
            tokens,
            owners,
            token_approvals,
            config,
            nonce,
            img_links,
            collection_owner,
            restriction_mint,
            number_combination,
        }
    }
}
