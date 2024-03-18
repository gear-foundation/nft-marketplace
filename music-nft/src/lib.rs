#![no_std]
use gstd::{
    collections::{HashMap, HashSet},
    debug, exec, msg,
    prelude::*,
    ActorId,
};
use music_nft_io::*;

#[derive(Debug)]
struct NftContract {
    pub tokens: HashMap<NftId, Nft>,
    pub owners: HashMap<ActorId, HashSet<NftId>>,
    pub restriction_mint: HashMap<ActorId, u32>, // made in order to track the number of mint nft
    pub token_approvals: HashMap<NftId, ActorId>,
    pub config: Config,
    pub nonce: NftId,
    pub links_and_data: Vec<(Links, ImageData)>,
    pub collection_owner: ActorId,
    pub permission_to_mint: Option<Vec<ActorId>>,
    pub total_number_of_tokens: Option<u64>,
    pub marketplace_address: ActorId,
    pub admins: HashSet<ActorId>,
}
static mut NFT_CONTRACT: Option<NftContract> = None;

impl NftContract {
    fn mint(&mut self, minter: ActorId, msg_value: u128) -> Result<MusicNftEvent, MusicNftError> {
        // check if there are tokens for mint
        self.check_available_amount_of_tokens()?;

        // check if a user can make a mint:
        // - quantity limit
        // - user-specific limit
        self.check_mint(&minter)?;
        let Some(next_nft_nonce) = self.nonce.checked_add(1) else {
            return Err(MusicNftError::MathOverflow);
        };
        // value check on mint
        self.payment_for_mint(msg_value)?;

        let rand_index = get_random_value(self.links_and_data.len() as u64);
        let token_id = self.nonce;

        let (links, img_info) = self
            .links_and_data
            .get_mut(rand_index as usize)
            .ok_or(MusicNftError::ErrorGettingRandom)?;

        if let Some(limit) = img_info.limit_copies.as_mut() {
            *limit -= 1;
        }

        let img_link = if let Some(img_link) = &links.img_link {
            img_link.clone()
        } else {
            self.config.collection_logo.clone()
        };

        let music_link = links.music_link.clone();
        let token_description = if let Some(token_description) = &img_info.description {
            token_description.clone()
        } else {
            self.config.description.clone()
        };

        // if there are 0 copies of this token left, then delete this token
        if let Some(0) = img_info.limit_copies {
            self.links_and_data.remove(rand_index as usize);
        }

        self.owners
            .entry(minter)
            .and_modify(|ids| {
                ids.insert(token_id);
            })
            .or_insert_with(|| HashSet::from([token_id]));

        let name = format!("{} - {}", self.config.name, token_id);
        let links_and_data = Nft {
            owner: minter,
            name,
            description: token_description,
            metadata: vec![],
            img_link: img_link.clone(),
            music_link: music_link.clone(),
            mint_time: exec::block_timestamp(),
        };

        self.nonce = next_nft_nonce;
        self.tokens.insert(token_id, links_and_data.clone());
        self.restriction_mint
            .entry(minter)
            .and_modify(|ids| {
                *ids += 1;
            })
            .or_insert(1);

        msg::send_with_gas(
            minter,
            Ok::<MusicNftEvent, MusicNftError>(MusicNftEvent::Minted { token_id, links_and_data }),
            0,
            0,
        )
        .expect("Error in sending message");
        Ok(MusicNftEvent::SuccessfullyMinted)
    }
    fn transfer_from(
        &mut self,
        from: &ActorId,
        to: &ActorId,
        token_id: NftId,
    ) -> Result<MusicNftEvent, MusicNftError> {
        // checking for the possibility to transfer
        self.can_transfer(from, to, &token_id)?;

        let nft = self
            .tokens
            .get_mut(&token_id)
            .ok_or(MusicNftError::TokenDoesNotExist)?;
        let tokens = self
            .owners
            .get_mut(from)
            .ok_or(MusicNftError::OwnerDoesNotHaveNft)?;
        tokens.retain(|&token| token != token_id);

        if tokens.is_empty() {
            self.owners.remove(from);
        }

        nft.owner = *to;

        self.owners
            .entry(*to)
            .and_modify(|ids| {
                ids.insert(token_id);
            })
            .or_insert_with(|| HashSet::from([token_id]));

        self.token_approvals.remove(&token_id);

        Ok(MusicNftEvent::Transferred {
            owner: *from,
            recipient: *to,
            token_id,
        })
    }
    fn approve(&mut self, to: &ActorId, token_id: NftId) -> Result<MusicNftEvent, MusicNftError> {
        // checking for the possibility to give an approve
        self.can_approve(&token_id)?;
        self.token_approvals.insert(token_id, *to);

        Ok(MusicNftEvent::Approved { to: *to, token_id })
    }

    fn revoke_approve(&mut self, token_id: NftId) -> Result<MusicNftEvent, MusicNftError> {
        let nft_info = self
            .tokens
            .get(&token_id)
            .ok_or(MusicNftError::TokenDoesNotExist)?;
        if nft_info.owner != msg::source() {
            return Err(MusicNftError::AccessDenied);
        }
        let _ = self
            .token_approvals
            .remove(&token_id)
            .ok_or(MusicNftError::NoApproval);
        Ok(MusicNftEvent::ApprovalRevoked { token_id })
    }

    fn get_token_info(&self, token_id: NftId) -> Result<MusicNftEvent, MusicNftError> {
        let nft = self
            .tokens
            .get(&token_id)
            .ok_or(MusicNftError::TokenDoesNotExist)?;

        let can_sell = self
            .config
            .sellable
            .map(|time| exec::block_timestamp() >= nft.mint_time + time)
            .unwrap_or_default();

        let approval = self.token_approvals.get(&token_id).copied();

        Ok(MusicNftEvent::TokenInfoReceived {
            token_owner: nft.owner,
            approval,
            sellable: can_sell,
            collection_owner: self.collection_owner,
            royalty: self.config.royalty,
        })
    }
    fn get_payment_for_mint(&self) -> Result<MusicNftEvent, MusicNftError> {
        Ok(MusicNftEvent::PaymentForMintReceived {
            payment_for_mint: self.config.payment_for_mint,
        })
    }
    fn expand(&mut self, additional_links: Vec<(Links, ImageData)>) -> Result<MusicNftEvent, MusicNftError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;
        if additional_links
            .iter()
            .any(|(_, img_data)| img_data.limit_copies.map_or(false, |limit| limit == 0))
        {
            return Err(MusicNftError::LimitIsZero);
        }

        self.links_and_data.extend(additional_links.clone());
        let mut number_tokens = sum_limit_copies(&self.links_and_data);
        // The tokens that have already been minted should be added as well
        if let Some(all_copies) = number_tokens.as_mut() {
            *all_copies += self.tokens.len() as u64;
        }
        self.total_number_of_tokens = number_tokens;

        Ok(MusicNftEvent::Expanded {
            additional_links,
            total_number_of_tokens: number_tokens,
        })
    }

    fn change_config(&mut self, config: Config) -> Result<MusicNftEvent, MusicNftError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;

        if !self.tokens.is_empty() {
            return Err(MusicNftError::ConfigCannotBeChanged);
        }

        // made 10_000 so you can enter hundredths of a percent.
        if config.royalty > 10_000 {
            return Err(MusicNftError::WrongRoyalty);
        }
        if config.transferable.is_none() && config.sellable.is_some() {
            return Err(MusicNftError::NotTransferable);
        }

        if let Some(limit) = config.user_mint_limit {
            if limit == 0 {
                return Err(MusicNftError::LimitIsZero);
            }
        }
        self.config = config.clone();
        Ok(MusicNftEvent::ConfigChanged { config })
    }

    fn add_users_for_mint(&mut self, users: Vec<ActorId>) -> Result<MusicNftEvent, MusicNftError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;
        if let Some(allowed_users) = &mut self.permission_to_mint {
            allowed_users.extend(users.clone());
            allowed_users.sort();
            allowed_users.dedup();
        } else if self.tokens.is_empty() {
            let mut new_users = users.clone();
            new_users.push(msg_src);
            self.permission_to_mint = Some(new_users);
        } else {
            return Err(MusicNftError::UserRestrictionCannotBeChanged);
        }
        
        Ok(MusicNftEvent::UsersForMintAdded { users })
    }
    fn delete_user_for_mint(&mut self, user: ActorId) -> Result<MusicNftEvent, MusicNftError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;
        let allowed_users = self
            .permission_to_mint
            .as_mut()
            .ok_or(MusicNftError::NoListOfRestriction)?;
        if let Some(pos) = allowed_users
            .iter()
            .position(|allowed_user| *allowed_user == user)
        {
            allowed_users.remove(pos);
        } else {
            return Err(MusicNftError::ThereIsNoSuchUser);
        }

        Ok(MusicNftEvent::UserForMintDeleted { user })
    }

    fn lift_restrictions_mint(&mut self) -> Result<MusicNftEvent, MusicNftError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;

        if !self.tokens.is_empty() {
            return Err(MusicNftError::UserRestrictionCannotBeChanged);
        }
        self.permission_to_mint = None;

        Ok(MusicNftEvent::LiftRestrictionMint)
    }

    fn add_admin(&mut self, admin: ActorId) -> Result<MusicNftEvent, MusicNftError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;
        self.admins.insert(admin);
        Ok(MusicNftEvent::AdminAdded { admin})
    }

    fn remove_admin(&mut self, admin: ActorId) -> Result<MusicNftEvent, MusicNftError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;
        if self.admins.len() == 1 {
            return Err(MusicNftError::OnlyOneAdminLeft);
        }
        self.admins.remove(&admin);
        Ok(MusicNftEvent::AdminRemoved { admin})
    }

    fn can_delete(&self) -> Result<MusicNftEvent, MusicNftError> {
        Ok(MusicNftEvent::CanDelete(self.tokens.is_empty()))
    }

    fn check_admin(&self, msg_src: ActorId) -> Result<(), MusicNftError> {
        if !self.admins.contains(&msg_src) {
            return Err(MusicNftError::AccessDenied);
        }
        Ok(())
    }

    fn check_available_amount_of_tokens(&self) -> Result<(), MusicNftError> {
        if self.links_and_data.is_empty() {
            return Err(MusicNftError::AllTokensMinted);
        }
        Ok(())
    }
    fn check_mint(&self, user: &ActorId) -> Result<(), MusicNftError> {
        if let Some(permission_to_mint) = &self.permission_to_mint {
            if !permission_to_mint.contains(user) {
                return Err(MusicNftError::AccessDenied);
            }
        }

        if let Some(limit) = self.config.user_mint_limit {
            if let Some(number_tokens) = self.restriction_mint.get(user) {
                if number_tokens >= &limit && self.collection_owner != *user {
                    return Err(MusicNftError::ExhaustedLimit);
                }
            }
        }
        Ok(())
    }
    
    // Checking for sufficient mint value and sending the value to the creator
    fn payment_for_mint(&self, msg_value: u128) -> Result<(), MusicNftError> {
        if self.config.payment_for_mint != 0 {
            if msg_value != self.config.payment_for_mint {
                return Err(MusicNftError::WrongValue);
            }
            // use send_with_gas to transfer the value directly to the balance, not to the mailbox.
            msg::send_with_gas(self.collection_owner, "", 0, self.config.payment_for_mint)
                .expect("Error in sending value");
        }

        Ok(())
    }

    // verification of approval
    fn can_approve(&self, token_id: &NftId) -> Result<(), MusicNftError> {
        if self.token_approvals.contains_key(token_id) {
            return Err(MusicNftError::ThereIsApproval);
        }
        let nft_info = self
            .tokens
            .get(token_id)
            .ok_or(MusicNftError::TokenDoesNotExist)?;
        if nft_info.owner != msg::source() {
            return Err(MusicNftError::AccessDenied);
        }
        Ok(())
    }
    // approval check
    fn check_approve(&self, user: &ActorId, token_id: &NftId) -> Result<(), MusicNftError> {
        let approved_account = self
            .token_approvals
            .get(token_id)
            .ok_or(MusicNftError::AccessDenied)?;
        if approved_account != user {
            return Err(MusicNftError::AccessDenied);
        }

        Ok(())
    }

    // doing all the checks to verify that the transfer can be made
    fn can_transfer(&self, from: &ActorId, to: &ActorId, token_id: &NftId) -> Result<(), MusicNftError> {
        let nft = self
            .tokens
            .get(token_id)
            .ok_or(MusicNftError::TokenDoesNotExist)?;

        let owner = nft.owner;
        if owner != *from {
            return Err(MusicNftError::AccessDenied);
        }
        let msg_src = msg::source();
        // if the owner of the token does not match the sender of the message, then check the approval
        if owner != msg_src {
            self.check_approve(&msg_src, token_id)?;
        }
        let time = self.config.transferable.ok_or(MusicNftError::NotTransferable)?;
        if exec::block_timestamp() < nft.mint_time + time {
            return Err(MusicNftError::NotTransferable);
        }

        if from == to {
            return Err(MusicNftError::AccessDenied);
        }
        Ok(())
    }
}

#[no_mangle]
extern "C" fn init() {
    let MusicNftInit {
        collection_owner,
        config,
        links_and_data,
        mut permission_to_mint
    } = msg::load().expect("Unable to decode `MusicNftInit`.");

    let msg_value = msg::value();
    let picture_fee = FEE_PER_UPLOADED_FILE * links_and_data.len() as u128;

    let existential_deposit = exec::env_vars().existential_deposit;

    if picture_fee < existential_deposit && msg_value != existential_deposit
        || picture_fee >= existential_deposit && msg_value != picture_fee
    {
        panic!("Wrong value for picture fee");
    }
    assert!(
        config
            .user_mint_limit
            .map(|limit| limit > 0)
            .unwrap_or(true),
        "The mint limit must be greater than zero"
    );
    if config.payment_for_mint > 0 && config.payment_for_mint < existential_deposit {
        panic!(
            "{}",
            format!(
                "The payment for mint must be greater than existential deposit ({})",
                existential_deposit
            )
        );
    }

    // can't take more than MAX_CREATOR_ROYALTY
    if config.royalty > MAX_CREATOR_ROYALTY {
        panic!("Royalty percent must be less");
    }
    // can't be made sellable but not transferable.
    if config.transferable.is_none() && config.sellable.is_some() {
        panic!("Tokens must be transferable");
    }
    // at least one image in the collection is a must have.
    assert!(
        !links_and_data.is_empty(),
        "There must be at least one link to create a collection"
    );
    // сan't have zero copies of the nft.
    if links_and_data
        .iter()
        .any(|(_, img_data)| img_data.limit_copies.map_or(false, |limit| limit == 0))
    {
        panic!("Limit of copies value is equal to 0");
    }
    let total_number_of_tokens = sum_limit_copies(&links_and_data);
    if let Some(users) = permission_to_mint.as_mut() {
        if !users.contains(&collection_owner) {
            users.push(collection_owner);
        }
    }
    let mut admins = HashSet::new();
    admins.insert(collection_owner);
    unsafe {
        NFT_CONTRACT = Some(NftContract {
            tokens: HashMap::new(),
            owners: HashMap::new(),
            token_approvals: HashMap::new(),
            restriction_mint: HashMap::new(),
            config: config.clone(),
            nonce: 0,
            links_and_data,
            collection_owner,
            total_number_of_tokens,
            permission_to_mint: permission_to_mint.clone(),
            marketplace_address: msg::source(),
            admins,
        })
    };

    msg::send(
        collection_owner,
        Ok::<MusicNftEvent, MusicNftError>(MusicNftEvent::Initialized {
            config: config.clone(),
            total_number_of_tokens,
            permission_to_mint: permission_to_mint.clone(),
        }),
        0,
    )
    .expect("Error during send to owner `MusicNftEvent::Initialized`");

    msg::reply(
        MusicNftEvent::Initialized {
            config: config.clone(),
            total_number_of_tokens,
            permission_to_mint,
        },
        0,
    )
    .expect("Error during a reply `MusicNftEvent::Initialized`");
}

#[no_mangle]
extern "C" fn handle() {
    let action: MusicNftAction = msg::load().expect("Could not load `MusicNftAction`.");
    let nft_contract: &mut NftContract = unsafe {
        NFT_CONTRACT
            .as_mut()
            .expect("Unexpected uninitialized `MusicNftContract`.")
    };

    let result = match action {
        MusicNftAction::Mint { minter } => {
            let msg_source = msg::source();
            let msg_value = msg::value();
            let result = if msg_source != nft_contract.marketplace_address {
                Err(MusicNftError::AccessDenied)
            } else {
                nft_contract.mint(minter, msg_value)
            };
            
            if result.is_err() {
                msg::send_with_gas(msg_source, "", 0, msg_value).expect("Error in sending value");
            }
            result
        }
        MusicNftAction::TransferFrom { from, to, token_id } => {
            nft_contract.transfer_from(&from, &to, token_id)
        }
        MusicNftAction::Transfer { to, token_id } => {
            nft_contract.transfer_from(&msg::source(), &to, token_id)
        }
        MusicNftAction::Approve { to, token_id } => nft_contract.approve(&to, token_id),
        MusicNftAction::RevokeApproval { token_id } => nft_contract.revoke_approve(token_id),
        MusicNftAction::Expand { additional_links } => nft_contract.expand(additional_links),
        MusicNftAction::ChangeConfig { config } => nft_contract.change_config(config),
        MusicNftAction::GetTokenInfo { token_id } => nft_contract.get_token_info(token_id),
        MusicNftAction::GetPaymentForMint => nft_contract.get_payment_for_mint(),
        MusicNftAction::CanDelete => nft_contract.can_delete(),
        MusicNftAction::AddUsersForMint { users } => nft_contract.add_users_for_mint(users),
        MusicNftAction::DeleteUserForMint { user } => nft_contract.delete_user_for_mint(user),
        MusicNftAction::LiftRestrictionMint => nft_contract.lift_restrictions_mint(),
        MusicNftAction::AddAdmin { admin } => nft_contract.add_admin(admin),
        MusicNftAction::RemoveAdmin { admin } => nft_contract.remove_admin(admin),
    };

    msg::reply(result, 0)
        .expect("Failed to encode or reply with `Result<MusicNftEvent, MusicNftError>`.");
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
            links_and_data,
            collection_owner,
            total_number_of_tokens,
            permission_to_mint,
            marketplace_address,
            admins,
            ..
        } = value;

        let tokens = tokens
            .into_iter()
            .map(|(nft_id, nft)| (nft_id, nft))
            .collect();
        let owners = owners
            .into_iter()
            .map(|(actor_id, token_set)| (actor_id, token_set.into_iter().collect()))
            .collect();
        let token_approvals = token_approvals
            .into_iter()
            .map(|(nft_id, actor_id)| (nft_id, actor_id))
            .collect();

        let admins = admins
            .into_iter()
            .collect();

        Self {
            tokens,
            owners,
            token_approvals,
            config,
            nonce,
            links_and_data,
            collection_owner,
            total_number_of_tokens,
            permission_to_mint,
            marketplace_address,
            admins
        }
    }
}

fn sum_limit_copies(links_and_data: &[(Links, ImageData)]) -> Option<u64> {
    let sum = links_and_data.iter().try_fold(0, |acc, (_, img_data)| {
        img_data.limit_copies.map(|y| acc + y as u64)
    });
    sum
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
