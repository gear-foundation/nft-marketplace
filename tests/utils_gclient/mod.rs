use gclient::{GearApi, Result};
use gear_core::ids::{MessageId, ProgramId};
use gstd::{ActorId, Encode};
use nft_io::{Config, ImageData, NftInit};
use nft_io::{NftAction, NftState, StateQuery as StateQueryNft, StateReply as StateReplyNft};
use nft_marketplace_io::*;

pub const USERS: &[u64] = &[5, 6, 7, 8];
pub const USERS_STR: &[&str] = &["//John", "//Mike", "//Dan"];
pub const ALICE: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];

pub trait ApiUtils {
    fn get_actor_id(&self) -> ActorId;
    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId;
}

impl ApiUtils for GearApi {
    fn get_actor_id(&self) -> ActorId {
        ActorId::new(
            self.account_id()
                .encode()
                .try_into()
                .expect("Unexpected invalid account id length."),
        )
    }

    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId {
        let api_temp = self
            .clone()
            .with(value)
            .expect("Unable to build `GearApi` instance with provided signer.");
        api_temp.get_actor_id()
    }
}

pub async fn get_all_state_nft(api: &GearApi, program_id: &ProgramId) -> Option<NftState> {
    let reply = api
        .read_state(*program_id, StateQueryNft::All.encode())
        .await
        .expect("Unexpected invalid reply.");
    if let StateReplyNft::All(state) = reply {
        Some(state)
    } else {
        None
    }
}

pub async fn get_all_collection_state(
    api: &GearApi,
    program_id: &ProgramId,
) -> Option<Vec<(ActorId, (String, ActorId))>> {
    let reply = api
        .read_state(*program_id, StateQuery::AllCollections.encode())
        .await
        .expect("Unexpected invalid reply.");
    if let StateReply::AllCollections(state) = reply {
        Some(state)
    } else {
        None
    }
}

pub async fn get_marketplace_state(api: &GearApi, program_id: &ProgramId) -> Option<State> {
    let reply = api
        .read_state(*program_id, StateQuery::All.encode())
        .await
        .expect("Unexpected invalid reply.");
    if let StateReply::All(state) = reply {
        Some(state)
    } else {
        None
    }
}

pub async fn init_marketplace(api: &GearApi) -> Result<(MessageId, ProgramId), ()> {
    let royalty_to_marketplace = 200;

    let init_marketplace = NftMarketplaceInit {
        gas_for_creation: 200_000_000_000,
        gas_for_mint: 100_000_000_000,
        gas_for_transfer_token: 5_000_000_000,
        gas_for_close_auction: 10_000_000_000,
        gas_for_delete_collection: 5_000_000_000,
        gas_for_get_info: 5_000_000_000,
        time_between_create_collections: 3_600_000, // 1 hour in milliseconds
        fee_per_uploaded_file: 257_142_857_100,
        royalty_to_marketplace_for_trade: royalty_to_marketplace,
        royalty_to_marketplace_for_mint: royalty_to_marketplace,
        minimum_transfer_value: 10_300_000_000_000, // because roylty to marketplace
        ms_in_block: 3_000,
    }
    .encode();

    let path = "target/wasm32-unknown-unknown/debug/nft_marketplace.opt.wasm";

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(path).unwrap(),
            init_marketplace.clone(),
            0,
            true,
        )
        .await
        .expect("Error calculate upload gas");

    let (message_id, program_id, _) = api
        .upload_program_bytes(
            gclient::code_from_os(path).unwrap(),
            gclient::now_micros().to_le_bytes(),
            init_marketplace,
            gas_info.min_limit,
            0,
        )
        .await
        .expect("Error upload program bytes");
    Ok((message_id, program_id))
}

pub async fn add_new_collection(
    api: &GearApi,
    marketplace_program_id: ProgramId,
) -> Result<MessageId, ()> {
    let (nft_code_id, _) = api
        .upload_code_by_path("target/wasm32-unknown-unknown/debug/nft.opt.wasm")
        .await
        .expect("Error upload code");

    let nft_code_id: [u8; 32] = nft_code_id.into();

    let add_collection_payload = NftMarketplaceAction::AddNewCollection {
        code_id: nft_code_id.into(),
        meta_link: String::from("My Meta"),
        type_name: String::from("Simple NFT"),
        type_description: String::from("My Collection"),
    };

    let gas_info = api
        .calculate_handle_gas(
            None,
            marketplace_program_id,
            add_collection_payload.encode(),
            0,
            true,
        )
        .await
        .expect("Error calculate gas");

    let (message_id, _) = api
        .send_message(
            marketplace_program_id,
            add_collection_payload,
            gas_info.min_limit,
            0,
        )
        .await
        .expect("Error send message");

    Ok(message_id)
}

pub async fn create_collection(
    api: &GearApi,
    marketplace_program_id: ProgramId,
) -> Result<MessageId, ()> {
    let img_data = ImageData {
        limit_copies: Some(1),
        auto_changing_rules: None,
    };
    let img_links_and_data: Vec<(String, ImageData)> = (0..10)
        .map(|i| (format!("Img-{}", i), img_data.clone()))
        .collect();

    let init_nft_payload = NftInit {
        collection_owner: USERS[0].into(),
        config: Config {
            name: "User Collection".to_string(),
            description: "User Collection".to_string(),
            collection_banner: "Collection banner".to_string(),
            collection_logo: "Collection logo".to_string(),
            collection_tags: vec!["tag1".to_string()],
            additional_links: None,
            royalty: 0,
            user_mint_limit: Some(3),
            payment_for_mint: 10_000_000_000_000,
            transferable: Some(0),
            sellable: Some(0),
        },
        img_links_and_data,
        permission_to_mint: None,
        fee_per_uploaded_file: 257_142_857_100,
    }
    .encode();

    let create_collection_payload = NftMarketplaceAction::CreateCollection {
        type_name: String::from("Simple NFT"),
        payload: init_nft_payload,
    };
    let gas_info = api
        .calculate_handle_gas(
            None,
            marketplace_program_id,
            create_collection_payload.encode(),
            10_000_000_000_000,
            true,
        )
        .await
        .expect("Error calculate gas");

    let (message_id, _) = api
        .send_message(
            marketplace_program_id,
            create_collection_payload,
            gas_info.min_limit,
            10_000_000_000_000,
        )
        .await
        .expect("Error send message");

    Ok(message_id)
}

pub async fn mint(
    api: &GearApi,
    marketplace_program_id: ProgramId,
    address_nft: ActorId,
    payment_for_mint: u128,
) -> Result<MessageId, ()> {
    let gas_info = api
        .calculate_handle_gas(
            None,
            marketplace_program_id,
            NftMarketplaceAction::Mint {
                collection_address: address_nft,
            }
            .encode(),
            payment_for_mint,
            true,
        )
        .await
        .expect("Error calculate gas");

    let (message_id, _) = api
        .send_message(
            marketplace_program_id,
            NftMarketplaceAction::Mint {
                collection_address: address_nft,
            },
            gas_info.min_limit,
            payment_for_mint,
        )
        .await
        .expect("Error send message");

    Ok(message_id)
}

pub async fn approve(
    api: &GearApi,
    marketplace_program_id: ProgramId,
    nft_program_id: ProgramId,
) -> Result<MessageId, ()> {
    let address_marketplace: ActorId = marketplace_program_id.into_bytes().into();
    let gas_info = api
        .calculate_handle_gas(
            None,
            nft_program_id,
            NftAction::Approve {
                to: address_marketplace,
                token_id: 0,
            }
            .encode(),
            0,
            true,
        )
        .await
        .expect("Error calculate gas");

    let (message_id, _) = api
        .send_message(
            nft_program_id,
            NftAction::Approve {
                to: address_marketplace,
                token_id: 0,
            },
            gas_info.min_limit,
            0,
        )
        .await
        .expect("Error send message");

    Ok(message_id)
}

pub async fn get_new_client(api: &GearApi, name: &str) -> GearApi {
    let alice_balance = api
        .total_balance(api.account_id())
        .await
        .expect("Error total balance");
    let amount = alice_balance / 10;
    api.transfer(
        api.get_specific_actor_id(name)
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid `ProgramId`."),
        amount,
    )
    .await
    .expect("Error transfer");

    api.clone()
        .with(USERS_STR[0])
        .expect("Unable to change signer.")
}
