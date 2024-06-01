#![no_std]

use crate::nft_messages::*;
use gstd::{collections::HashMap, exec, msg, prelude::*, prog::ProgramGenerator, ActorId, CodeId};
use nft_marketplace_io::*;

mod auction;
mod nft_messages;
mod offer;
mod payment;
mod sale;

type CollectionId = ActorId;
type TokenId = u64;
type Price = u128;
type TypeName = String;

#[derive(Default)]
pub struct NftMarketplace {
    pub admins: Vec<ActorId>,
    pub collection_to_owner: HashMap<CollectionId, (TypeName, ActorId)>,
    pub time_creation: HashMap<ActorId, u64>,
    pub type_collections: HashMap<String, TypeCollectionInfo>,
    pub sales: HashMap<(CollectionId, TokenId), NftInfoForSale>,
    pub auctions: HashMap<(CollectionId, TokenId), Auction>,
    pub offers: HashMap<Offer, Price>,
    pub config: Config,
    pub minimum_value_for_trade: u128,
    pub allow_message: bool,
    pub allow_create_collection: bool,
}

static mut NFT_MARKETPLACE: Option<NftMarketplace> = None;

#[no_mangle]
extern "C" fn init() {
    let NftMarketplaceInit {
        gas_for_creation,
        gas_for_transfer_token,
        gas_for_mint,
        gas_for_close_auction,
        gas_for_delete_collection,
        gas_for_get_info,
        time_between_create_collections,
        royalty_to_marketplace_for_trade,
        royalty_to_marketplace_for_mint,
        ms_in_block,
        fee_per_uploaded_file,
        max_creator_royalty,
        max_number_of_images,
    } = msg::load().expect("Unable to decode `NftMarketplaceInit`");
    let existential_deposit = exec::env_vars().existential_deposit;
    let minimum_value_for_trade =
        existential_deposit * 10_000 / (10_000 - royalty_to_marketplace_for_trade) as u128;
    let nft_marketplace = NftMarketplace {
        admins: vec![msg::source()],
        config: Config {
            gas_for_creation,
            gas_for_transfer_token,
            gas_for_mint,
            gas_for_close_auction,
            gas_for_delete_collection,
            gas_for_get_info,
            time_between_create_collections,
            royalty_to_marketplace_for_trade,
            royalty_to_marketplace_for_mint,
            ms_in_block,
            fee_per_uploaded_file,
            max_creator_royalty,
            max_number_of_images,
        },
        minimum_value_for_trade,
        ..Default::default()
    };
    unsafe { NFT_MARKETPLACE = Some(nft_marketplace) };
    msg::reply(
        Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::Initialized {
            time_between_create_collections,
            royalty_to_marketplace_for_trade,
            royalty_to_marketplace_for_mint,
            minimum_value_for_trade,
            fee_per_uploaded_file,
            max_creator_royalty,
            max_number_of_images,
        }),
        0,
    )
    .expect("Failed to encode or reply with `Result<NftMarketplaceEvent, NftMarketplaceError>`.");
}

#[gstd::async_main]
async fn main() {
    let action: NftMarketplaceAction =
        msg::load().expect("Unable to decode `NftMarketplaceAction`");
    let nft_marketplace = unsafe {
        NFT_MARKETPLACE
            .as_mut()
            .expect("`Nft Marketplace` is not initialized.")
    };

    let result = match action {
        NftMarketplaceAction::AddNewCollection {
            code_id,
            meta_link,
            type_name,
            type_description,
            allow_create,
        } => nft_marketplace.add_new_collection(
            code_id,
            meta_link,
            type_name,
            type_description,
            allow_create,
        ),
        NftMarketplaceAction::CreateCollection { type_name, payload } => {
            let msg_source = msg::source();
            let msg_value = msg::value();
            let reply = nft_marketplace
                .create_collection(type_name, payload, msg_source, msg_value)
                .await;
            if reply.is_err() {
                msg::send_with_gas(
                    msg_source,
                    Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::ValueSent),
                    0,
                    msg_value,
                )
                .expect("Error in sending value");
            }
            reply
        }
        NftMarketplaceAction::Mint { collection_address } => {
            let msg_source = msg::source();
            let msg_value = msg::value();
            let reply = nft_marketplace
                .mint(collection_address, msg_source, msg_value)
                .await;
            if reply.is_err() {
                msg::send_with_gas(
                    msg_source,
                    Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::ValueSent),
                    0,
                    msg_value,
                )
                .expect("Error in sending value");
            }
            reply
        }
        NftMarketplaceAction::SaleNft {
            collection_address,
            token_id,
            price,
        } => {
            nft_marketplace
                .sell(collection_address, token_id, price)
                .await
        }
        NftMarketplaceAction::CancelSaleNft {
            collection_address,
            token_id,
        } => {
            nft_marketplace
                .cancel_sale(collection_address, token_id)
                .await
        }
        NftMarketplaceAction::BuyNft {
            collection_address,
            token_id,
        } => {
            let msg_source = msg::source();
            let msg_value = msg::value();
            let reply = nft_marketplace
                .buy(collection_address, token_id, msg_source, msg_value)
                .await;
            if reply.is_err() {
                msg::send_with_gas(
                    msg_source,
                    Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::ValueSent),
                    0,
                    msg_value,
                )
                .expect("Error in sending value");
            }
            reply
        }
        NftMarketplaceAction::CreateAuction {
            collection_address,
            token_id,
            min_price,
            duration,
        } => {
            nft_marketplace
                .create_auction(collection_address, token_id, min_price, duration)
                .await
        }
        NftMarketplaceAction::AddBid {
            collection_address,
            token_id,
        } => {
            let msg_source = msg::source();
            let msg_value = msg::value();
            let reply =
                nft_marketplace.add_bid(collection_address, token_id, msg_source, msg_value);
            if reply.is_err() {
                msg::send_with_gas(
                    msg_source,
                    Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::ValueSent),
                    0,
                    msg_value,
                )
                .expect("Error in sending value");
            }
            reply
        }
        NftMarketplaceAction::CloseAuction {
            collection_address,
            token_id,
        } => {
            nft_marketplace
                .close_auction(collection_address, token_id)
                .await
        }
        NftMarketplaceAction::CancelAuction {
            collection_address,
            token_id,
        } => {
            nft_marketplace
                .cancel_auction(collection_address, token_id)
                .await
        }
        NftMarketplaceAction::CreateOffer {
            collection_address,
            token_id,
        } => {
            let msg_source = msg::source();
            let msg_value = msg::value();

            let reply = nft_marketplace
                .create_offer(collection_address, token_id, msg_source, msg_value)
                .await;
            if reply.is_err() {
                msg::send_with_gas(
                    msg_source,
                    Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::ValueSent),
                    0,
                    msg_value,
                )
                .expect("Error in sending value");
            }
            reply
        }
        NftMarketplaceAction::CancelOffer {
            collection_address,
            token_id,
        } => nft_marketplace.cancel_offer(collection_address, token_id),
        NftMarketplaceAction::AcceptOffer { offer } => nft_marketplace.accept_offer(offer).await,
        NftMarketplaceAction::DeleteCollection { collection_address } => {
            nft_marketplace.delete_collection(collection_address).await
        }
        NftMarketplaceAction::AddAdmins { users } => nft_marketplace.add_admins(users),
        NftMarketplaceAction::DeleteAdmin { user } => nft_marketplace.delete_admin(user),
        NftMarketplaceAction::UpdateConfig {
            gas_for_creation,
            gas_for_mint,
            gas_for_transfer_token,
            gas_for_close_auction,
            gas_for_delete_collection,
            gas_for_get_info,
            time_between_create_collections,
            royalty_to_marketplace_for_trade,
            royalty_to_marketplace_for_mint,
            ms_in_block,
            fee_per_uploaded_file,
            max_creator_royalty,
            max_number_of_images,
        } => nft_marketplace.update_config(
            gas_for_creation,
            gas_for_mint,
            gas_for_transfer_token,
            gas_for_close_auction,
            gas_for_delete_collection,
            gas_for_get_info,
            time_between_create_collections,
            royalty_to_marketplace_for_trade,
            royalty_to_marketplace_for_mint,
            ms_in_block,
            fee_per_uploaded_file,
            max_creator_royalty,
            max_number_of_images,
        ),
        NftMarketplaceAction::AllowMessage(allow) => nft_marketplace.allow_message(allow),
        NftMarketplaceAction::AllowCreateCollection(allow) => {
            nft_marketplace.allow_create_collection(allow)
        }
        NftMarketplaceAction::AddExternalCollection {
            collection_address,
            type_name,
        } => nft_marketplace.add_external_collection(collection_address, type_name),
    };

    msg::reply(result, 0).expect(
        "Failed to encode or reply with `Result<NftMarketplaceEvent, NftMarketplaceError>`.",
    );
}

impl NftMarketplace {
    pub fn add_new_collection(
        &mut self,
        code_id: CodeId,
        meta_link: String,
        type_name: String,
        type_description: String,
        allow_create: bool,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(&msg_src)?;

        let collection_info = TypeCollectionInfo {
            code_id,
            meta_link: meta_link.clone(),
            type_description: type_description.clone(),
            allow_create,
        };
        self.type_collections
            .insert(type_name.clone(), collection_info.clone());

        Ok(NftMarketplaceEvent::NewCollectionAdded {
            code_id,
            meta_link,
            type_name,
            type_description,
        })
    }

    pub async fn create_collection(
        &mut self,
        type_name: String,
        payload: Vec<u8>,
        msg_src: ActorId,
        msg_value: u128,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        self.check_time_creation(&msg_src)?;
        self.check_allow_create_collection(&msg_src)?;
        self.check_allow_message(&msg_src)?;

        let collection_info = self.get_collection_info(&type_name)?;
        if !collection_info.allow_create {
            return Err(NftMarketplaceError::AccessDenied);
        }

        let address = match ProgramGenerator::create_program_bytes_with_gas_for_reply(
            collection_info.code_id,
            payload,
            self.config.gas_for_creation,
            msg_value,
            0,
        ) {
            Ok(future) => match future.await {
                Ok((address, _)) => address,
                Err(_) => {
                    return Err(NftMarketplaceError::CreationError);
                }
            },
            Err(_) => {
                return Err(NftMarketplaceError::CreationError);
            }
        };

        self.collection_to_owner
            .insert(address, (type_name.clone(), msg_src));

        self.time_creation
            .entry(msg_src)
            .insert(exec::block_timestamp());

        Ok(NftMarketplaceEvent::CollectionCreated {
            type_name,
            collection_address: address,
        })
    }

    pub async fn mint(
        &mut self,
        collection_address: ActorId,
        msg_src: ActorId,
        msg_value: u128,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        self.check_allow_message(&msg_src)?;
        mint(
            collection_address,
            msg_src,
            msg_value,
            self.config.gas_for_mint,
            self.config.gas_for_get_info,
            self.config.royalty_to_marketplace_for_mint,
        )
        .await
    }

    pub async fn delete_collection(
        &mut self,
        collection_address: ActorId,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();

        let (.., collection_owner) = self
            .collection_to_owner
            .get(&collection_address)
            .ok_or(NftMarketplaceError::WrongCollectionAddress)?;

        if self.admins.contains(&msg_src) {
            self.collection_to_owner.remove(&collection_address);
        } else if *collection_owner == msg_src {
            let reply = msg::send_with_gas_for_reply_as::<NftAction, Result<NftEvent, NftError>>(
                collection_address,
                NftAction::CanDelete,
                self.config.gas_for_delete_collection,
                0,
                0,
            )
            .expect("Error during get info about deleting")
            .await
            .expect("The program had a problem getting information about the deletion");

            if let NftEvent::CanDelete(answer) = self.check_reply(reply)? {
                if answer {
                    self.collection_to_owner.remove(&collection_address);
                } else {
                    return Err(NftMarketplaceError::AccessDenied);
                }
            } else {
                return Err(NftMarketplaceError::WrongReply);
            }
        } else {
            return Err(NftMarketplaceError::AccessDenied);
        }

        Ok(NftMarketplaceEvent::CollectionDeleted { collection_address })
    }

    pub fn add_admins(
        &mut self,
        users: Vec<ActorId>,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(&msg_src)?;
        self.admins.extend(users.clone());
        Ok(NftMarketplaceEvent::AdminsAdded { users })
    }
    pub fn delete_admin(
        &mut self,
        user: ActorId,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(&msg_src)?;
        self.admins.retain(|&admin| admin != user);
        Ok(NftMarketplaceEvent::AdminDeleted { user })
    }
    pub fn update_config(
        &mut self,
        gas_for_creation: Option<u64>,
        gas_for_mint: Option<u64>,
        gas_for_transfer_token: Option<u64>,
        gas_for_close_auction: Option<u64>,
        gas_for_delete_collection: Option<u64>,
        gas_for_get_info: Option<u64>,
        time_between_create_collections: Option<u64>,
        royalty_to_marketplace_for_trade: Option<u16>,
        royalty_to_marketplace_for_mint: Option<u16>,
        ms_in_block: Option<u32>,
        fee_per_uploaded_file: Option<u128>,
        max_creator_royalty: Option<u16>,
        max_number_of_images: Option<u64>,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(&msg_src)?;
        if let Some(gas) = gas_for_creation {
            self.config.gas_for_creation = gas;
        }
        if let Some(gas) = gas_for_mint {
            self.config.gas_for_mint = gas;
        }
        if let Some(gas) = gas_for_transfer_token {
            self.config.gas_for_transfer_token = gas;
        }
        if let Some(gas) = gas_for_close_auction {
            self.config.gas_for_close_auction = gas;
        }
        if let Some(gas) = gas_for_delete_collection {
            self.config.gas_for_delete_collection = gas;
        }
        if let Some(gas) = gas_for_get_info {
            self.config.gas_for_get_info = gas;
        }
        if let Some(time) = time_between_create_collections {
            self.config.time_between_create_collections = time;
        }
        let minimum_value_for_trade = if let Some(royalty) = royalty_to_marketplace_for_trade {
            self.config.royalty_to_marketplace_for_trade = royalty;
            let existential_deposit = exec::env_vars().existential_deposit;
            self.minimum_value_for_trade =
                existential_deposit * 10_000 / (10_000 - royalty) as u128;
            Some(self.minimum_value_for_trade)
        } else {
            None
        };
        if let Some(royalty) = royalty_to_marketplace_for_mint {
            self.config.royalty_to_marketplace_for_mint = royalty;
        }
        if let Some(ms_in_block) = ms_in_block {
            self.config.ms_in_block = ms_in_block;
        }
        if let Some(fee) = fee_per_uploaded_file {
            self.config.fee_per_uploaded_file = fee;
        }
        if let Some(royalty) = max_creator_royalty {
            self.config.max_creator_royalty = royalty;
        }
        if let Some(max_number_of_images) = max_number_of_images {
            self.config.max_number_of_images = max_number_of_images;
        }

        Ok(NftMarketplaceEvent::ConfigUpdated {
            gas_for_creation,
            gas_for_mint,
            gas_for_transfer_token,
            gas_for_close_auction,
            gas_for_delete_collection,
            gas_for_get_info,
            time_between_create_collections,
            royalty_to_marketplace_for_trade,
            royalty_to_marketplace_for_mint,
            ms_in_block,
            minimum_value_for_trade,
            fee_per_uploaded_file,
            max_creator_royalty,
            max_number_of_images,
        })
    }
    pub fn allow_message(
        &mut self,
        allow: bool,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(&msg_src)?;
        self.allow_message = allow;
        Ok(NftMarketplaceEvent::AllowMessageChanged)
    }
    pub fn allow_create_collection(
        &mut self,
        allow: bool,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(&msg_src)?;
        self.allow_create_collection = allow;
        Ok(NftMarketplaceEvent::AllowCreateCollectionChanged)
    }

    pub fn add_external_collection(
        &mut self,
        collection_address: ActorId,
        type_name: String,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(&msg_src)?;
        if !self.type_collections.contains_key(&type_name) {
            return Err(NftMarketplaceError::WrongCollectionName);
        }
        self.collection_to_owner
            .insert(collection_address, (type_name.clone(), msg_src));
        Ok(NftMarketplaceEvent::CollectionCreated {
            type_name,
            collection_address,
        })
    }

    pub fn balance_out(&mut self) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(&msg_src)?;
        let balance = exec::value_available();
        let existential_deposit = exec::env_vars().existential_deposit;
        if balance < 2 * existential_deposit {
            return Err(NftMarketplaceError::InsufficientBalance);
        }
        let value = balance - existential_deposit;
        msg::send_with_gas(
            msg_src,
            Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::ValueSent),
            0,
            value,
        )
        .expect("Error in sending value");
        Ok(NftMarketplaceEvent::BalanceHasBeenWithdrawn { value })
    }

    fn check_time_creation(&self, user: &ActorId) -> Result<(), NftMarketplaceError> {
        if let Some(time) = self.time_creation.get(user) {
            if exec::block_timestamp() - time < self.config.time_between_create_collections
                && !self.admins.contains(user)
            {
                return Err(NftMarketplaceError::DeadlineError);
            }
        }
        Ok(())
    }

    fn check_admin(&self, msg_src: &ActorId) -> Result<(), NftMarketplaceError> {
        if !self.admins.contains(msg_src) {
            return Err(NftMarketplaceError::AccessDenied);
        }
        Ok(())
    }
    fn check_allow_message(&self, msg_src: &ActorId) -> Result<(), NftMarketplaceError> {
        if !self.allow_message && !self.admins.contains(msg_src) {
            return Err(NftMarketplaceError::AccessDenied);
        }

        Ok(())
    }
    fn check_allow_create_collection(&self, msg_src: &ActorId) -> Result<(), NftMarketplaceError> {
        if !self.allow_create_collection && !self.admins.contains(msg_src) {
            return Err(NftMarketplaceError::AccessDenied);
        }
        Ok(())
    }

    fn check_reply(
        &self,
        reply: Result<NftEvent, NftError>,
    ) -> Result<NftEvent, NftMarketplaceError> {
        match reply {
            Ok(result) => Ok(result),
            Err(_) => Err(NftMarketplaceError::ErrorFromCollection),
        }
    }

    fn get_collection_info(
        &self,
        type_name: &str,
    ) -> Result<&TypeCollectionInfo, NftMarketplaceError> {
        self.type_collections
            .get(type_name)
            .ok_or(NftMarketplaceError::WrongCollectionName)
    }
}

#[no_mangle]
extern "C" fn state() {
    let nft_marketplace = unsafe {
        NFT_MARKETPLACE
            .take()
            .expect("Contract state is uninitialized")
    };
    let query: StateQuery = msg::load().expect("Unable to load the state query");
    let reply = match query {
        StateQuery::All => StateReply::All(nft_marketplace.into()),
        StateQuery::Admins => StateReply::Admins(nft_marketplace.admins),
        StateQuery::CollectionsInfo => {
            let type_collections = nft_marketplace.type_collections.into_iter().collect();
            StateReply::CollectionsInfo(type_collections)
        }
        StateQuery::Config => StateReply::Config(nft_marketplace.config),
        StateQuery::AllCollections => {
            let collection_to_owner = nft_marketplace.collection_to_owner.into_iter().collect();
            StateReply::AllCollections(collection_to_owner)
        }
        StateQuery::GetCollectionInfo(collection_address) => {
            let collection_to_owner = nft_marketplace.collection_to_owner.get(&collection_address);
            if let Some((type_name, owner)) = collection_to_owner {
                let meta_link = &nft_marketplace
                    .type_collections
                    .get(type_name)
                    .expect("This collection type name must exist")
                    .meta_link;
                let collection_info = CollectionInfo {
                    owner: *owner,
                    type_name: type_name.clone(),
                    meta_link: meta_link.clone(),
                };
                StateReply::CollectionInfo(Some(collection_info))
            } else {
                StateReply::CollectionInfo(None)
            }
        }
    };
    msg::reply(reply, 0).expect("Unable to share the state");
}

impl From<NftMarketplace> for State {
    fn from(value: NftMarketplace) -> Self {
        let NftMarketplace {
            admins,
            collection_to_owner,
            time_creation,
            type_collections,
            sales,
            auctions,
            offers,
            config,
            minimum_value_for_trade,
            allow_message,
            allow_create_collection,
        } = value;

        let collection_to_owner = collection_to_owner.into_iter().collect();

        let time_creation = time_creation.into_iter().collect();

        let type_collections = type_collections.into_iter().collect();

        let sales = sales.into_iter().collect();
        let auctions = auctions.into_iter().collect();

        let offers = offers.into_iter().collect();

        Self {
            admins,
            collection_to_owner,
            time_creation,
            type_collections,
            sales,
            auctions,
            offers,
            config,
            minimum_value_for_trade,
            allow_message,
            allow_create_collection,
        }
    }
}
