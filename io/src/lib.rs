#![no_std]
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, CodeId};

pub struct NftMarketplaceMetadata;
impl Metadata for NftMarketplaceMetadata {
    type Init = In<NftMarketplaceInit>;
    type Handle = InOut<NftMarketplaceAction, Result<NftMarketplaceEvent, NftMarketplaceError>>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = InOut<StateQuery, StateReply>;
}

/// * gas_for_creation - gas needed to create the collection
/// * gas_for_transfer_token - gas that is needed to transfer nft tokens (in case of sale, auction or offer)
/// * gas_for_close_auction - gas which is needed to send a delayed message to close the auction
/// (this action includes a transfer, so gas_for_close_auction must be greater than gas_for_transfer_token)
/// * gas_for_delete_collection - gas that is needed to delete a collection 
/// (a message is sent to the collection contract to see if the collection can be deleted)
/// * gas_for_get_token_info - gas which is needed to get information from the collection about the token
/// (used for sale, auction and offers)
/// * time_between_create_collections - time between collection creation 
/// (to avoid regular users from creating collections too often)
/// * minimum_transfer_value - minimum allowable transfer value
/// * ms_in_block - number of milliseconds in one block
/// (this variable is needed to correctly calculate the time for the delayed message in the auction)
#[derive(Encode, Decode, TypeInfo)]
pub struct NftMarketplaceInit {
    pub gas_for_creation: u64,
    pub gas_for_transfer_token: u64, 
    pub gas_for_close_auction: u64,
    pub gas_for_delete_collection: u64,
    pub gas_for_get_token_info: u64, 
    pub time_between_create_collections: u64,
    pub minimum_transfer_value: u128,
    pub ms_in_block: u32,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum NftMarketplaceAction {
    AddNewCollection {
        code_id: CodeId,
        meta_link: String,
        type_name: String,
        type_description: String,
    },
    CreateCollection {
        type_name: String,
        payload: Vec<u8>,
    },
    SaleNft {
        collection_address: ActorId,
        token_id: u64,
        price: u128,
    },
    CancelSaleNft {
        collection_address: ActorId,
        token_id: u64,
    },
    BuyNft {
        collection_address: ActorId,
        token_id: u64,
    },
    CreateAuction {
        collection_address: ActorId,
        token_id: u64,
        min_price: u128,
        duration_ms: u32,
    },
    AddBid {
        collection_address: ActorId,
        token_id: u64,
    },
    CloseAuction {
        collection_address: ActorId,
        token_id: u64,
    },
    CancelAuction {
        collection_address: ActorId,
        token_id: u64,
    },
    CreateOffer {
        collection_address: ActorId,
        token_id: u64,
    },
    CancelOffer {
        collection_address: ActorId,
        token_id: u64,
    },
    AcceptOffer {
        offer: Offer,
    },
    DeleteCollection {
        collection_address: ActorId,
    },
    AddAdmins {
        users: Vec<ActorId>,
    },
    DeleteAdmin {
        user: ActorId,
    },
    UpdateConfig {
        gas_for_creation: Option<u64>,
        gas_for_transfer_token: Option<u64>,
        gas_for_close_auction: Option<u64>,
        gas_for_delete_collection: Option<u64>,
        gas_for_get_token_info: Option<u64>,
        time_between_create_collections: Option<u64>,
        minimum_transfer_value: Option<u128>,
        ms_in_block: Option<u32>,
    },
}

#[derive(Encode, Decode, Debug, TypeInfo)]
pub enum NftMarketplaceEvent {
    NewCollectionAdded {
        code_id: CodeId,
        meta_link: String,
        type_name: String,
        type_description: String,
    },
    CollectionCreated {
        type_name: String,
        collection_address: ActorId,
    },
    SaleNft {
        collection_address: ActorId,
        token_id: u64,
        price: u128,
        owner: ActorId,
    },
    SaleNftCanceled {
        collection_address: ActorId,
        token_id: u64,
    },
    NftSold {
        collection_address: ActorId,
        token_id: u64,
        price: u128,
        current_owner: ActorId,
    },
    AuctionCreated {
        collection_address: ActorId,
        token_id: u64,
        min_price: u128,
        duration_ms: u32,
    },
    AuctionClosed {
        collection_address: ActorId,
        token_id: u64,
        price: u128,
        current_owner: ActorId,
    },
    BidAdded {
        collection_address: ActorId,
        token_id: u64,
    },
    AuctionCanceled {
        collection_address: ActorId,
        token_id: u64,
    },
    OfferCreated {
        collection_address: ActorId,
        token_id: u64,
        price: u128,
    },
    OfferCanceled {
        collection_address: ActorId,
        token_id: u64,
    },
    OfferAccepted {
        offer: Offer,
    },
    CollectionDeleted {
        collection_address: ActorId,
    },
    AdminsAdded {
        users: Vec<ActorId>,
    },
    AdminDeleted {
        user: ActorId,
    },
    ConfigUpdated {
        gas_for_creation: Option<u64>,
        gas_for_transfer_token: Option<u64>,
        gas_for_close_auction: Option<u64>,
        gas_for_delete_collection: Option<u64>,
        gas_for_get_token_info: Option<u64>,
        time_between_create_collections: Option<u64>,
        minimum_transfer_value: Option<u128>,
        ms_in_block: Option<u32>,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct NftMarketplaceError(pub String);

#[derive(Encode, Decode, TypeInfo)]
pub enum StateQuery {
    All,
    Admins,
    CollectionsInfo,
    Config,
    AllCollections,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    All(State),
    Admins(Vec<ActorId>),
    CollectionsInfo(Vec<(String, CollectionInfo)>),
    Config(Config),
    AllCollections(Vec<(ActorId, ActorId)>),
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct State {
    pub admins: Vec<ActorId>,
    pub collection_to_owner: Vec<(ActorId, ActorId)>,
    pub time_creation: Vec<(ActorId, u64)>,
    pub type_collections: Vec<(String, CollectionInfo)>,
    pub sales: Vec<((ActorId, u64), NftInfoForSale)>,
    pub auctions: Vec<((ActorId, u64), Auction)>,
    pub offers: Vec<(Offer, u128)>,
    pub config: Config,
}

/// * code_id - code_id is used to create a collection by that CodeId, 
/// the admin should preload the NFT contract and specify in the marketplace so that regular users can use it
/// * meta_link -  it is necessary to set a reference where the meta of this collection type will be stored for further interaction with the contract
/// * type_description - description of this type of collection
#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct CollectionInfo {
    pub code_id: CodeId,
    pub meta_link: String,
    pub type_description: String,
}

#[derive(Default, Debug, Encode, Decode, TypeInfo, Clone)]
pub struct Config {
    pub gas_for_creation: u64,
    pub gas_for_transfer_token: u64,
    pub gas_for_close_auction: u64,
    pub gas_for_delete_collection: u64,
    pub gas_for_get_token_info: u64,
    pub time_between_create_collections: u64,
    pub minimum_transfer_value: u128,
    pub ms_in_block: u32,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct NftInfoForSale {
    pub price: u128,
    pub token_owner: ActorId,
    pub collection_owner: ActorId,
    pub royalty: u16,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct Auction {
    pub owner: ActorId,
    pub started_at: u64,
    pub ended_at: u64,
    pub current_price: u128,
    pub current_winner: ActorId,
    pub collection_owner: ActorId,
    pub royalty: u16,
}
#[derive(Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Hash)]
pub struct Offer {
    pub collection_address: ActorId,
    pub token_id: u64,
    pub creator: ActorId,
}
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum NftAction {
    Transfer {
        to: ActorId,
        token_id: u64,
    },
    TransferFrom {
        from: ActorId,
        to: ActorId,
        token_id: u64,
    },
    GetTokenInfo {
        token_id: u64,
    },
    CanDelete,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum NftEvent {
    Transferred {
        owner: ActorId,
        recipient: ActorId,
        token_id: u64,
    },
    TokenInfoReceived {
        token_owner: ActorId,
        approval: Option<ActorId>,
        sellable: bool,
        collection_owner: ActorId,
        royalty: u16,
    },
    CanDelete(bool),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct NftError(pub String);
