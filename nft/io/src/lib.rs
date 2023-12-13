#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
pub type NftId = u64;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<NftInit>;
    type Handle = InOut<NftAction, NftEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<StateQuery, StateReply>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct NftInit {
    pub owner: ActorId,
    pub config: Config,
    pub img_links: Vec<(String, u32)>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub collection_tags: Vec<String>,
    pub collection_img: String,
    // pub tokens_limit: Option<u64>,
    pub user_mint_limit: Option<u32>,
    pub transferable: bool,
    pub approvable: bool,
    pub burnable: bool,
    pub sellable: bool,
    pub attendable: bool,
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
    Mint,
    Approve {
        to: ActorId,
        token_id: NftId,
    },
    RevokeApproval {
        token_id: NftId,
    },
    Burn {
        token_id: NftId,
    },
    Expand {
        additional_links: Vec<(String, u32)>,
    },
    ChangeConfig {
        config: Config,
    },
    AddAdmin {
        new_admin: ActorId,
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
        owner: ActorId,
        approval: Option<ActorId>,
        sellable: bool,
    },
    CanDelete(bool),
    Initialized {
        config: Config,
    },
    AdminAdded,
    Minted {
        owner: ActorId,
        token_id: NftId,
        media_url: String,
    },
    Burnt {
        token_id: NftId,
    },
    Approved {
        to: ActorId,
        token_id: NftId,
    },
    ApprovalRevoked {
        token_id: NftId,
    },
    Expanded {
        additional_links: Vec<(String, u32)>,
    },
    ConfigChanged {
        config: Config,
    },
}
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct NftError(pub String);

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct NftState {
    pub tokens: Vec<(NftId, Nft)>,
    pub owners: Vec<(ActorId, Vec<NftId>)>,
    pub token_approvals: Vec<(NftId, ActorId)>,
    pub config: Config,
    pub nonce: NftId,
    pub img_links: Vec<(String, u32)>,
    pub admins: Vec<ActorId>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Nft {
    pub owner: ActorId,
    pub name: String,
    pub description: String,
    pub media_url: String,
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
