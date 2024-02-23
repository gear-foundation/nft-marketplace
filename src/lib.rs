#![no_std]

use crate::nft_messages::*;
use gstd::{
    collections::HashMap, debug, exec, msg, prelude::*, prog::ProgramGenerator, ActorId, CodeId,
};
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
        fee_per_uploaded_file,
        royalty_to_marketplace_for_trade,
        royalty_to_marketplace_for_mint,
        minimum_transfer_value,
        ms_in_block,
    } = msg::load().expect("Unable to decode `NftMarketplaceInit`");

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
            fee_per_uploaded_file,
            royalty_to_marketplace_for_trade,
            royalty_to_marketplace_for_mint,
            minimum_transfer_value,
            ms_in_block,
        },
        ..Default::default()
    };
    unsafe { NFT_MARKETPLACE = Some(nft_marketplace) };
    msg::reply(
        Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::Initialized {
            time_between_create_collections,
            fee_per_uploaded_file,
            royalty_to_marketplace_for_trade,
            royalty_to_marketplace_for_mint,
            minimum_transfer_value,
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
        } => nft_marketplace.add_new_collection(code_id, meta_link, type_name, type_description),
        NftMarketplaceAction::CreateCollection { type_name, payload } => {
            nft_marketplace.create_collection(type_name, payload).await
        }
        NftMarketplaceAction::Mint { collection_address } => {
            nft_marketplace.mint(collection_address).await
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
                msg::send_with_gas(msg_source, "", 0, msg_value).expect("Error in sending value");
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
        } => nft_marketplace.add_bid(collection_address, token_id),
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
                msg::send_with_gas(msg_source, "", 0, msg_value).expect("Error in sending value");
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
            fee_per_uploaded_file,
            minimum_transfer_value,
            ms_in_block,
        } => nft_marketplace.update_config(
            gas_for_creation,
            gas_for_mint,
            gas_for_transfer_token,
            gas_for_close_auction,
            gas_for_delete_collection,
            gas_for_get_info,
            time_between_create_collections,
            fee_per_uploaded_file,
            royalty_to_marketplace_for_trade,
            royalty_to_marketplace_for_mint,
            minimum_transfer_value,
            ms_in_block,
        ),
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
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;

        let collection_info = TypeCollectionInfo {
            code_id,
            meta_link: meta_link.clone(),
            type_description: type_description.clone(),
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
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        let msg_value = msg::value();
        self.check_time_creation(&msg_src)?;

        let collection_info = self.get_collection_info(&type_name)?;

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
                    msg::send_with_gas(msg_src, "", 0, msg_value).expect("Error in sending value");
                    return Err(NftMarketplaceError::CreationError);
                }
            },
            Err(_) => {
                msg::send_with_gas(msg_src, "", 0, msg_value).expect("Error in sending value");
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
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        let msg_value = msg::value();

        let reply = mint(
            collection_address,
            msg_src,
            self.config.gas_for_mint,
            self.config.gas_for_get_info,
            self.config.royalty_to_marketplace_for_mint,
        )
        .await;
        if reply.is_err() {
            msg::send_with_gas(msg_src, "", 0, msg_value).expect("Error in sending value");
        }
        reply
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
        self.check_admin(msg_src)?;
        self.admins.extend(users.clone());
        Ok(NftMarketplaceEvent::AdminsAdded { users })
    }
    pub fn delete_admin(
        &mut self,
        user: ActorId,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;
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
        fee_per_uploaded_file: Option<u128>,
        royalty_to_marketplace_for_trade: Option<u16>,
        royalty_to_marketplace_for_mint: Option<u16>,
        minimum_transfer_value: Option<u128>,
        ms_in_block: Option<u32>,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;
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
        if let Some(fee) = fee_per_uploaded_file {
            self.config.fee_per_uploaded_file = fee;
        }
        if let Some(royalty) = royalty_to_marketplace_for_trade {
            self.config.royalty_to_marketplace_for_trade = royalty;
        }
        if let Some(royalty) = royalty_to_marketplace_for_mint {
            self.config.royalty_to_marketplace_for_mint = royalty;
        }
        if let Some(min_value) = minimum_transfer_value {
            self.config.minimum_transfer_value = min_value;
        }
        if let Some(time_block) = ms_in_block {
            self.config.ms_in_block = time_block;
        }

        Ok(NftMarketplaceEvent::ConfigUpdated {
            gas_for_creation,
            gas_for_mint,
            gas_for_transfer_token,
            gas_for_close_auction,
            gas_for_delete_collection,
            gas_for_get_info,
            time_between_create_collections,
            fee_per_uploaded_file,
            royalty_to_marketplace_for_trade,
            royalty_to_marketplace_for_mint,
            minimum_transfer_value,
            ms_in_block,
        })
    }

    pub fn balance_out(&mut self, value: u128) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();
        self.check_admin(msg_src)?;
        if value < EXISTENTIAL_DEPOSIT {
            return Err(NftMarketplaceError::LessThanExistentialDeposit);
        }
        msg::send_with_gas(msg_src, "", 0, value).expect("Error in sending value");
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

    fn check_admin(&self, msg_src: ActorId) -> Result<(), NftMarketplaceError> {
        if !self.admins.contains(&msg_src) {
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
            let type_collections = nft_marketplace
                .type_collections
                .into_iter()
                .map(|(id, collection_info)| (id, collection_info))
                .collect();
            StateReply::CollectionsInfo(type_collections)
        }
        StateQuery::Config => StateReply::Config(nft_marketplace.config),
        StateQuery::AllCollections => {
            let collection_to_owner = nft_marketplace
                .collection_to_owner
                .into_iter()
                .map(|(owner, collection_info)| (owner, collection_info))
                .collect();
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
        } = value;

        let collection_to_owner = collection_to_owner
            .into_iter()
            .map(|(owner, collection_info)| (owner, collection_info))
            .collect();

        let time_creation = time_creation
            .into_iter()
            .map(|(id, time)| (id, time))
            .collect();

        let type_collections = type_collections
            .into_iter()
            .map(|(id, collection_info)| (id, collection_info))
            .collect();

        let sales = sales.into_iter().map(|(id, info)| (id, info)).collect();
        let auctions = auctions
            .into_iter()
            .map(|(id, auction)| (id, auction))
            .collect();

        let offers = offers
            .into_iter()
            .map(|(offer, price)| (offer, price))
            .collect();

        Self {
            admins,
            collection_to_owner,
            time_creation,
            type_collections,
            sales,
            auctions,
            offers,
            config,
        }
    }
}
