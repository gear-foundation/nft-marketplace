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
/// * gas_for_get_info - gas which is needed to get information from the collection
/// (used for sale, auction and offers)
/// * time_between_create_collections - time between collection creation
/// (to avoid regular users from creating collections too often)
/// * minimum_transfer_value - minimum allowable transfer value
/// (this variable is needed to correctly calculate the time for the delayed message in the auction)
#[derive(Encode, Decode, TypeInfo)]
pub struct NftMarketplaceInit {
    pub gas_for_creation: u64,
    pub gas_for_transfer_token: u64,
    pub gas_for_mint: u64,
    pub gas_for_close_auction: u64,
    pub gas_for_delete_collection: u64,
    pub gas_for_get_info: u64,
    pub time_between_create_collections: u64,
    pub royalty_to_marketplace_for_trade: u16,
    pub royalty_to_marketplace_for_mint: u16,
    pub ms_in_block: u32,
    pub fee_per_uploaded_file: u128,
    pub max_creator_royalty: u16,
    pub max_number_of_images: u64,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum NftMarketplaceAction {
    AddNewCollection {
        code_id: CodeId,
        meta_link: String,
        type_name: String,
        type_description: String,
        allow_create: bool,
    },
    CreateCollection {
        type_name: String,
        payload: Vec<u8>,
    },
    Mint {
        collection_address: ActorId,
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
        duration: u32,
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
    },
    AllowMessage(bool),
    AllowCreateCollection(bool),
    AddExternalCollection {
        collection_address: ActorId,
        type_name: String,
    },
}

#[derive(Encode, Decode, Debug, TypeInfo)]
pub enum NftMarketplaceEvent {
    Initialized {
        time_between_create_collections: u64,
        royalty_to_marketplace_for_trade: u16,
        royalty_to_marketplace_for_mint: u16,
        minimum_value_for_trade: u128,
        fee_per_uploaded_file: u128,
        max_creator_royalty: u16,
        max_number_of_images: u64,
    },
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
    Minted {
        collection_address: ActorId,
        minter: ActorId,
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
        current_price: u128,
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
        gas_for_mint: Option<u64>,
        gas_for_transfer_token: Option<u64>,
        gas_for_close_auction: Option<u64>,
        gas_for_delete_collection: Option<u64>,
        gas_for_get_info: Option<u64>,
        time_between_create_collections: Option<u64>,
        royalty_to_marketplace_for_trade: Option<u16>,
        royalty_to_marketplace_for_mint: Option<u16>,
        ms_in_block: Option<u32>,
        minimum_value_for_trade: Option<u128>,
        fee_per_uploaded_file: Option<u128>,
        max_creator_royalty: Option<u16>,
        max_number_of_images: Option<u64>,
    },
    BalanceHasBeenWithdrawn {
        value: u128,
    },
    ValueSent,
    AllowMessageChanged,
    AllowCreateCollectionChanged
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum NftMarketplaceError {
    WrongCollectionAddress,
    AlreadyOnAuction,
    AuctionClosed,
    AlreadyOnSale,
    WrongReply,
    WrongCollectionName,
    AccessDenied,
    DeadlineError,
    ThereIsNoSuchAuction,
    LessOrEqualThanBid,
    NotSellable,
    NoApproveToMarketplace,
    ErrorFromCollection,
    WrongDataOffer,
    OfferDoesNotExist,
    SaleDoesNotExist,
    ValueIsLessThanPrice,
    CreationError,
    ErrorGetInfo,
    WrongValue,
    MintError,
    InsufficientBalance,
    LessThanMinimumValueForTrade
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateQuery {
    All,
    Admins,
    CollectionsInfo,
    Config,
    AllCollections,
    GetCollectionInfo(ActorId),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    All(State),
    Admins(Vec<ActorId>),
    CollectionsInfo(Vec<(String, TypeCollectionInfo)>),
    Config(Config),
    AllCollections(Vec<(ActorId, (String, ActorId))>),
    CollectionInfo(Option<CollectionInfo>),
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct State {
    pub admins: Vec<ActorId>,
    pub collection_to_owner: Vec<(ActorId, (String, ActorId))>,
    pub time_creation: Vec<(ActorId, u64)>,
    pub type_collections: Vec<(String, TypeCollectionInfo)>,
    pub sales: Vec<((ActorId, u64), NftInfoForSale)>,
    pub auctions: Vec<((ActorId, u64), Auction)>,
    pub offers: Vec<(Offer, u128)>,
    pub config: Config,
    pub minimum_value_for_trade: u128,
    pub allow_message: bool,
    pub allow_create_collection: bool,
}

/// * code_id - code_id is used to create a collection by that CodeId,
/// the admin should preload the NFT contract and specify in the marketplace so that regular users can use it
/// * meta_link -  it is necessary to set a reference where the meta of this collection type will be stored for further interaction with the contract
/// * type_description - description of this type of collection
#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct TypeCollectionInfo {
    pub code_id: CodeId,
    pub meta_link: String,
    pub type_description: String,
    pub allow_create: bool,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct CollectionInfo {
    pub owner: ActorId,
    pub type_name: String,
    pub meta_link: String,
}

#[derive(Default, Debug, Encode, Decode, TypeInfo, Clone)]
pub struct Config {
    pub gas_for_creation: u64,
    pub gas_for_mint: u64,
    pub gas_for_transfer_token: u64,
    pub gas_for_close_auction: u64,
    pub gas_for_delete_collection: u64,
    pub gas_for_get_info: u64,
    pub time_between_create_collections: u64,
    pub royalty_to_marketplace_for_trade: u16,
    pub royalty_to_marketplace_for_mint: u16,
    pub ms_in_block: u32,
    pub fee_per_uploaded_file: u128,
    pub max_creator_royalty: u16,
    pub max_number_of_images: u64,
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
    GetPaymentForMint,
    Mint {
        minter: ActorId,
    },
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
    PaymentForMintReceived {
        payment_for_mint: u128,
    },
    SuccessfullyMinted,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum NftError {
    MathOverflow,
    ErrorGettingRandom,
    TokenDoesNotExist,
    AllTokensMinted,
    OwnerDoesNotHaveNft,
    AccessDenied,
    NoApproval,
    ThereIsApproval,
    LimitIsZero,
    ConfigCannotBeChanged,
    WrongRoyalty,
    NotTransferable,
    UserRestrictionCannotBeChanged,
    NoListOfRestriction,
    ThereIsNoSuchUser,
    ExhaustedLimit,
    WrongValue,
}
