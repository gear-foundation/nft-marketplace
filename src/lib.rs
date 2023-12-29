#![no_std]

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

#[derive(Default)]
pub struct NftMarketplace {
    pub admins: Vec<ActorId>,
    pub collection_to_owner: HashMap<CollectionId, ActorId>,
    pub time_creation: HashMap<ActorId, u64>,
    pub type_collections: HashMap<String, CollectionInfo>,
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
        gas_for_close_auction,
        gas_for_delete_collection,
        gas_for_get_token_info,
        time_between_create_collections,
        minimum_transfer_value,
        ms_in_block,
    } = msg::load().expect("Unable to decode `NftMarketplaceInit`");

    let nft_marketplace = NftMarketplace {
        admins: vec![msg::source()],
        config: Config {
            gas_for_creation,
            gas_for_transfer_token,
            gas_for_close_auction,
            gas_for_delete_collection,
            gas_for_get_token_info,
            time_between_create_collections,
            minimum_transfer_value,
            ms_in_block,
        },
        ..Default::default()
    };
    unsafe { NFT_MARKETPLACE = Some(nft_marketplace) };
}

#[gstd::async_main]
async fn main() {
    let action: NftMarketplaceAction =
        msg::load().expect("Unable to decode `NftMarketplaceAction`");
    let nft_marketplace = unsafe {
        NFT_MARKETPLACE
            .as_mut()
            .expect("`Collection Factory` is not initialized.")
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
        } => nft_marketplace.buy(collection_address, token_id).await,
        NftMarketplaceAction::CreateAuction {
            collection_address,
            token_id,
            min_price,
            duration_ms,
        } => {
            nft_marketplace
                .create_auction(collection_address, token_id, min_price, duration_ms)
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
            nft_marketplace
                .create_offer(collection_address, token_id)
                .await
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
            gas_for_transfer_token,
            gas_for_close_auction,
            gas_for_delete_collection,
            gas_for_get_token_info,
            time_between_create_collections,
            minimum_transfer_value,
            ms_in_block,
        } => nft_marketplace.update_config(
            gas_for_creation,
            gas_for_transfer_token,
            gas_for_close_auction,
            gas_for_delete_collection,
            gas_for_get_token_info,
            time_between_create_collections,
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
        self.check_admin()?;

        let collection_info = CollectionInfo {
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
        self.check_time_creation(&msg_src)?;

        let collection_info = self.get_collection_info(&type_name)?;
        debug!("PAYLOAD: {:?}", payload);

        let (address, _) = ProgramGenerator::create_program_bytes_with_gas_for_reply(
            collection_info.code_id,
            payload,
            self.config.gas_for_creation,
            0,
            0,
        )
        .expect("Error during Collection program initialization")
        .await
        .expect("Program was not initialized");

        self.collection_to_owner.insert(address, msg_src);

        self.time_creation
            .entry(msg_src)
            .insert(exec::block_timestamp());

        Ok(NftMarketplaceEvent::CollectionCreated {
            type_name,
            collection_address: address,
        })
    }

    pub async fn delete_collection(
        &mut self,
        collection_address: ActorId,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let msg_src = msg::source();

        let collection_owner =
            if let Some(owner) = self.collection_to_owner.get(&collection_address) {
                *owner
            } else {
                return Err(NftMarketplaceError(
                    "There is no collection with that address".to_owned(),
                ));
            };

        if self.admins.contains(&msg_src) {
            self.collection_to_owner.remove(&collection_address);
        } else if collection_owner == msg_src {
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
                    return Err(NftMarketplaceError("Removal denied".to_owned()));
                }
            } else {
                return Err(NftMarketplaceError("Wrong received reply".to_owned()));
            }
        } else {
            return Err(NftMarketplaceError(
                "Only the owner of the collection can send this message".to_owned(),
            ));
        }

        Ok(NftMarketplaceEvent::CollectionDeleted { collection_address })
    }

    pub fn add_admins(
        &mut self,
        users: Vec<ActorId>,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        self.check_admin()?;
        self.admins.extend(users.clone());
        Ok(NftMarketplaceEvent::AdminsAdded { users })
    }
    pub fn delete_admin(
        &mut self,
        user: ActorId,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        self.check_admin()?;
        self.admins.retain(|&admin| admin != user);
        Ok(NftMarketplaceEvent::AdminDeleted { user })
    }
    pub fn update_config(
        &mut self,
        gas_for_creation: Option<u64>,
        gas_for_transfer_token: Option<u64>,
        gas_for_close_auction: Option<u64>,
        gas_for_delete_collection: Option<u64>,
        gas_for_get_token_info: Option<u64>,
        time_between_create_collections: Option<u64>,
        minimum_transfer_value: Option<u128>,
        ms_in_block: Option<u32>,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        self.check_admin()?;
        if let Some(gas) = gas_for_creation {
            self.config.gas_for_creation = gas;
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
        if let Some(gas) = gas_for_get_token_info {
            self.config.gas_for_get_token_info = gas;
        }
        if let Some(time) = time_between_create_collections {
            self.config.time_between_create_collections = time;
        }
        if let Some(min_value) = minimum_transfer_value {
            self.config.minimum_transfer_value = min_value;
        }
        if let Some(time_block) = ms_in_block {
            self.config.ms_in_block = time_block;
        }

        Ok(NftMarketplaceEvent::ConfigUpdated {
            gas_for_creation,
            gas_for_transfer_token,
            gas_for_close_auction,
            gas_for_delete_collection,
            gas_for_get_token_info,
            time_between_create_collections,
            minimum_transfer_value,
            ms_in_block,
        })
    }

    fn check_time_creation(&self, user: &ActorId) -> Result<(), NftMarketplaceError> {
        if let Some(time) = self.time_creation.get(user) {
            if exec::block_timestamp() - time < self.config.time_between_create_collections
                && !self.admins.contains(user)
            {
                return Err(NftMarketplaceError(
                    "The time limit for creating a collection has not yet expired.".to_owned(),
                ));
            }
        }
        Ok(())
    }

    fn check_admin(&self) -> Result<(), NftMarketplaceError> {
        if !self.admins.contains(&msg::source()) {
            return Err(NftMarketplaceError(
                "Only admin can send this message".to_owned(),
            ));
        }
        Ok(())
    }

    fn check_reply(
        &self,
        reply: Result<NftEvent, NftError>,
    ) -> Result<NftEvent, NftMarketplaceError> {
        match reply {
            Ok(result) => Ok(result),
            Err(NftError(error_string)) => Err(NftMarketplaceError(error_string.clone())),
        }
    }

    fn get_collection_info(&self, type_name: &str) -> Result<&CollectionInfo, NftMarketplaceError> {
        if let Some(collection_info) = self.type_collections.get(type_name) {
            Ok(collection_info)
        } else {
            Err(NftMarketplaceError(
                "There is no collection with this name yet.".to_owned(),
            ))
        }
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
    match query {
        StateQuery::All => {
            msg::reply(StateReply::All(nft_marketplace.into()), 0)
                .expect("Unable to share the state");
        }

        StateQuery::Admins => {
            msg::reply(StateReply::Admins(nft_marketplace.admins), 0)
                .expect("Unable to share the state");
        }
        StateQuery::CollectionsInfo => {
            let type_collections = nft_marketplace
                .type_collections
                .into_iter()
                .map(|(id, collection_info)| (id, collection_info))
                .collect();
            msg::reply(StateReply::CollectionsInfo(type_collections), 0)
                .expect("Unable to share the state");
        }
        StateQuery::Config => {
            msg::reply(StateReply::Config(nft_marketplace.config), 0)
                .expect("Unable to share the state");
        }
        StateQuery::AllCollections => {
            let collection_to_owner = nft_marketplace
                .collection_to_owner
                .into_iter()
                .map(|(owner, collections)| (owner, collections))
                .collect();
            msg::reply(StateReply::AllCollections(collection_to_owner), 0)
                .expect("Unable to share the state");
        }
    }
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
            .map(|(owner, collections)| (owner, collections))
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
