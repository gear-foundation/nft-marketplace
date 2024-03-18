#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
pub type NftId = u64;
pub type TimeSec = u32;

pub const FEE_PER_UPLOADED_FILE: u128 = 282_857_142_900;
pub const MAX_CREATOR_ROYALTY: u16 = 1_000; // 10%

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<NftInit>;
    type Handle = InOut<NftAction, Result<NftEvent, NftError>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<StateQuery, StateReply>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct NftInit {
    pub collection_owner: ActorId,
    pub config: Config,
    pub img_links_and_data: Vec<(String, ImageData)>,
    pub permission_to_mint: Option<Vec<ActorId>>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct ImageData {
    pub limit_copies: Option<u32>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub collection_tags: Vec<String>,
    pub collection_banner: String,
    pub collection_logo: String,
    pub user_mint_limit: Option<u32>,
    pub additional_links: Option<AdditionalLinks>,
    pub royalty: u16,
    pub payment_for_mint: u128,
    pub transferable: Option<u64>,
    pub sellable: Option<u64>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct AdditionalLinks {
    pub external_url: Option<String>,
    pub telegram: Option<String>,
    pub xcom: Option<String>,
    pub medium: Option<String>,
    pub discord: Option<String>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum NftAction {
    Transfer {
        to: ActorId,
        token_id: NftId,
    },
    TransferFrom {
        from: ActorId,
        to: ActorId,
        token_id: NftId,
    },
    GetTokenInfo {
        token_id: NftId,
    },
    CanDelete,
    GetPaymentForMint,
    Mint {
        minter: ActorId,
    },
    Approve {
        to: ActorId,
        token_id: NftId,
    },
    RevokeApproval {
        token_id: NftId,
    },
    Expand {
        additional_links: Vec<(String, ImageData)>,
    },
    ChangeConfig {
        config: Config,
    },
    AddUsersForMint {
        users: Vec<ActorId>,
    },
    DeleteUserForMint {
        user: ActorId,
    },
    LiftRestrictionMint,
    AddAdmin {
        admin: ActorId,
    },
    RemoveAdmin {
        admin: ActorId,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum NftEvent {
    Transferred {
        owner: ActorId,
        recipient: ActorId,
        token_id: NftId,
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
    Minted {
        token_id: NftId,
        nft_data: Nft,
    },
    Initialized {
        config: Config,
        total_number_of_tokens: Option<u64>,
        permission_to_mint: Option<Vec<ActorId>>,
    },
    Approved {
        to: ActorId,
        token_id: NftId,
    },
    ApprovalRevoked {
        token_id: NftId,
    },
    Expanded {
        additional_links: Vec<(String, ImageData)>,
        total_number_of_tokens: Option<u64>,
    },
    ConfigChanged {
        config: Config,
    },
    UsersForMintAdded {
        users: Vec<ActorId>,
    },
    UserForMintDeleted {
        user: ActorId,
    },
    LiftRestrictionMint,
    AdminAdded {
        admin: ActorId,
    },
    AdminRemoved {
        admin: ActorId,
    }
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
    OnlyOneAdminLeft
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct NftState {
    pub tokens: Vec<(NftId, Nft)>,
    pub owners: Vec<(ActorId, Vec<NftId>)>,
    pub token_approvals: Vec<(NftId, ActorId)>,
    pub config: Config,
    pub nonce: NftId,
    pub img_links_and_data: Vec<(String, ImageData)>,
    pub collection_owner: ActorId,
    pub total_number_of_tokens: Option<u64>,
    pub permission_to_mint: Option<Vec<ActorId>>,
    pub marketplace_address: ActorId,
    pub admins: Vec<ActorId>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Nft {
    pub owner: ActorId,
    pub name: String,
    pub description: String,
    pub metadata: Vec<String>,
    pub media_url: String,
    pub mint_time: u64,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateQuery {
    Name,
    Description,
    Config,
    All,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    Name(String),
    Description(String),
    Config(Config),
    All(NftState),
}
