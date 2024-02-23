use crate::utils::*;
use utils::prelude::*;
mod utils;
use gtest::Program;
use nft_io::{
    Action, AdditionalLinks, Config, ImageData, NftAction, NftError, NftInit, NftState,
    StateQuery as StateQueryNft, StateReply as StateReplyNft,
};
use nft_marketplace_io::*;

const USERS: &[u64] = &[5, 6, 7, 8];

#[test]
fn successful_basics() {
    let sys = utils::initialize_system();
    let marketplace = Program::init_marketplace(&sys);
    let nft_collection_code_id =
        sys.submit_code("target/wasm32-unknown-unknown/debug/nft.opt.wasm");

    let state_reply = marketplace
        .read_state(StateQuery::CollectionsInfo)
        .expect("Unexpected invalid state.");
    if let StateReply::CollectionsInfo(state) = state_reply {
        assert!(state.is_empty(), "Collection info should be empty");
        println!("Collection info: {:?}", state);
    }
    // Successful addition of a new collection
    let name_simple_nft = "Simple NFT".to_string();
    marketplace.add_new_collection(
        ADMINS[0],
        nft_collection_code_id.into_bytes().into(),
        name_simple_nft.clone(),
        None,
    );

    let state_reply = marketplace
        .read_state(StateQuery::CollectionsInfo)
        .expect("Unexpected invalid state.");
    if let StateReply::CollectionsInfo(state) = state_reply {
        assert!(!state.is_empty(), "Collection info shouldn't be empty");
        println!("Collection info: {:?}", state);
    }
    // Successful creation of a new collection
    let init_nft_payload = get_init_nft_payload(USERS[0].into(), 0, Some(3), 0, None);
    sys.mint_to(USERS[0], 100_000_000_000_000);
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        None,
    );

    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    let address_nft = if let StateReply::AllCollections(state) = state_reply {
        assert!(!state.is_empty(), "Collections shouldn't be empty");
        println!("Collections: {:?}", state);
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_list: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_list);
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        assert_eq!(state.collection_owner, USERS[0].into(), "Wrong Admin");
        println!("Collection NFT info: {:?}", state);
    }

    // Successful change config in the new collection
    let config = Config {
        name: "My Collection".to_string(),
        description: "My Collection".to_string(),
        collection_banner: "Collection banner".to_string(),
        collection_logo: "Collection logo".to_string(),
        collection_tags: vec!["tag1".to_string()],
        additional_links: None,
        royalty: 0,
        user_mint_limit: 3.into(),
        payment_for_mint: 0,
        transferable: Some(0),
        sellable: Some(0),
    };
    let res = nft_collection.send(USERS[0], NftAction::ChangeConfig { config });
    assert!(!res.main_failed());
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        assert_eq!(state.config.name, "My Collection".to_string());
        assert_eq!(state.config.description, "My Collection".to_string());
    }

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        println!("Collection NFT info: {:?}", state);
        assert_eq!(state.collection_owner, USERS[0].into(), "Wrong Admin");
        let (owner, token_id) = state.owners.get(0).expect("Can't be None");
        assert_eq!(*owner, USERS[1].into(), "Wrong owner");
        assert_eq!(*token_id, vec![0], "Wrong token id");
    }

    // Successful transfer NFT in the collection
    let res = nft_collection.send(
        USERS[1],
        NftAction::Transfer {
            to: USERS[2].into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        println!("!!!!!!!!!! STATE: {:?}", state);
        let (owner, token_id) = state.owners.get(0).expect("Can't be None");
        assert_eq!(*owner, USERS[2].into(), "Wrong owner");
        assert_eq!(*token_id, vec![0], "Wrong token id");
    }

    // Successful approve NFT in the collection (USERS[2] -Approve-> USERS[1])
    let res = nft_collection.send(
        USERS[2],
        NftAction::Approve {
            to: USERS[1].into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        let (token_id, approval) = state.token_approvals.get(0).expect("Can't be None");
        assert_eq!(*token_id, 0, "Wrong owner");
        assert_eq!(*approval, USERS[1].into(), "Wrong token id");
    }
    // Check approve (USERS[1] -Transfer-> USERS[3])
    let res = nft_collection.send(
        USERS[1],
        NftAction::TransferFrom {
            from: USERS[2].into(),
            to: USERS[3].into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        println!("STATE OWNERS: {:?}", state.owners);
        let (owner, token_id) = state.owners.get(0).expect("Can't be None");
        assert_eq!(*owner, USERS[3].into(), "Wrong owner");
        assert_eq!(*token_id, vec![0], "Wrong token id");
    }

    // Check limit of mint = 3
    marketplace.mint(USERS[3], address_nft, None);
    marketplace.mint(USERS[3], address_nft, None);
    marketplace.mint(USERS[3], address_nft, None);
    marketplace.mint(
        USERS[3],
        address_nft,
        Some(NftMarketplaceError::ErrorFromCollection),
    );

    // Successful Expand NFT in the collection
    let img_data = ImageData {
        limit_copies: Some(1),
        auto_changing_rules: None,
    };
    let res = nft_collection.send(
        USERS[0],
        NftAction::Expand {
            additional_links: vec![
                ("add_link_1".to_string(), img_data.clone()),
                ("add_link_2".to_string(), img_data.clone()),
            ],
        },
    );
    assert!(!res.main_failed());
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        assert_eq!(
            state.img_links_and_data.len(),
            8,
            "Wrong length of img_links_and_data"
        );
        println!("STATE: {:?}", state);
    }
}

#[test]
fn failures() {
    let sys = utils::initialize_system();
    let marketplace = Program::init_marketplace(&sys);
    let nft_collection_code_id =
        sys.submit_code("target/wasm32-unknown-unknown/debug/nft.opt.wasm");

    let name_simple_nft = "Simple NFT".to_string();
    marketplace.add_new_collection(
        ADMINS[0],
        nft_collection_code_id.into_bytes().into(),
        name_simple_nft.clone(),
        None,
    );

    // The mint limit must be greater than zero
    let mut init_nft_payload = get_init_nft_payload(USERS[0].into(), 0, Some(0), 0, None);
    sys.mint_to(USERS[0], 100_000_000_000_000);
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        Some(NftMarketplaceError::CreationError),
    );
    sys.claim_value_from_mailbox(USERS[0]);
    let balance = sys.balance_of(USERS[0]);
    assert_eq!(balance, 100_000_000_000_000, "Wrong balance");

    // There must be at least one link to create a collection
    init_nft_payload.config.user_mint_limit = 4.into();
    init_nft_payload.img_links_and_data = vec![];
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        Some(NftMarketplaceError::CreationError),
    );

    sys.claim_value_from_mailbox(USERS[0]);
    let balance = sys.balance_of(USERS[0]);
    assert_eq!(balance, 100_000_000_000_000, "Wrong balance");

    // Limit of copies value is equal to 0
    let img_data = ImageData {
        limit_copies: Some(0),
        auto_changing_rules: None,
    };
    init_nft_payload.img_links_and_data = vec![("Img-0".to_owned(), img_data.clone())];
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        Some(NftMarketplaceError::CreationError),
    );
    sys.claim_value_from_mailbox(USERS[0]);
    let balance = sys.balance_of(USERS[0]);
    assert_eq!(balance, 100_000_000_000_000, "Wrong balance");

    let img_data = ImageData {
        limit_copies: Some(1),
        auto_changing_rules: None,
    };
    let img_links_and_data: Vec<(String, ImageData)> = (0..5)
        .map(|i| (format!("Img-{}", i), img_data.clone()))
        .collect();

    init_nft_payload.img_links_and_data = img_links_and_data;
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        None,
    );

    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    let address_nft = if let StateReply::AllCollections(state) = state_reply {
        assert!(!state.is_empty(), "Collections shouldn't be empty");
        println!("Collections: {:?}", state);
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_list: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_list);

    for _ in 0..4 {
        marketplace.mint(USERS[1], address_nft, None);
    }
    marketplace.mint(
        USERS[1],
        address_nft,
        Some(NftMarketplaceError::ErrorFromCollection),
    );
    marketplace.mint(USERS[2], address_nft, None);
    marketplace.mint(
        USERS[2],
        address_nft,
        Some(NftMarketplaceError::ErrorFromCollection),
    );

    let config = Config {
        name: "User Collection".to_string(),
        description: "User Collection".to_string(),
        collection_banner: "Collection banner".to_string(),
        collection_logo: "Collection logo".to_string(),
        collection_tags: vec!["tag1".to_string()],
        additional_links: None,
        royalty: 0,
        user_mint_limit: 3.into(),
        payment_for_mint: 0,
        transferable: Some(0),
        sellable: Some(0),
    };
    let res = nft_collection.send(USERS[0], NftAction::ChangeConfig { config });
    check_nft_error(USERS[0], &res, NftError::ConfigCannotBeChanged);
    assert!(!res.main_failed());

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        println!("STATE: {:?}", state);
    }

    let img_data = ImageData {
        limit_copies: Some(4),
        auto_changing_rules: None,
    };
    let res = nft_collection.send(
        USERS[0],
        NftAction::Expand {
            additional_links: vec![("New_img".to_owned(), img_data)],
        },
    );
    assert!(!res.main_failed());
    marketplace.mint(USERS[2], address_nft, None);

    let res = nft_collection.send(
        USERS[2],
        NftAction::Approve {
            to: USERS[3].into(),
            token_id: 5,
        },
    );
    assert!(!res.main_failed());
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        println!("STATE: {:?}", state);
    }
    let res = nft_collection.send(
        USERS[2],
        NftAction::Transfer {
            to: USERS[1].into(),
            token_id: 5,
        },
    );
    assert!(!res.main_failed());
    let res = nft_collection.send(
        USERS[3],
        NftAction::TransferFrom {
            from: USERS[1].into(),
            to: USERS[3].into(),
            token_id: 5,
        },
    );
    assert!(!res.main_failed());
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        println!("STATE: {:?}", state);
    }
    check_nft_error(USERS[3], &res, NftError::AccessDenied);
}

#[test]
fn check_auto_changing_rules() {
    let sys = utils::initialize_system();
    let marketplace = Program::init_marketplace(&sys);
    let nft_collection_code_id =
        sys.submit_code("target/wasm32-unknown-unknown/debug/nft.opt.wasm");

    let state_reply = marketplace
        .read_state(StateQuery::CollectionsInfo)
        .expect("Unexpected invalid state.");
    if let StateReply::CollectionsInfo(state) = state_reply {
        assert!(state.is_empty(), "Collection info should be empty");
        println!("Collection info: {:?}", state);
    }
    // Successful addition of a new collection
    let name_simple_nft = "Simple NFT".to_string();
    marketplace.add_new_collection(
        ADMINS[0],
        nft_collection_code_id.into_bytes().into(),
        name_simple_nft.clone(),
        None,
    );

    let state_reply = marketplace
        .read_state(StateQuery::CollectionsInfo)
        .expect("Unexpected invalid state.");
    if let StateReply::CollectionsInfo(state) = state_reply {
        assert!(!state.is_empty(), "Collection info shouldn't be empty");
        println!("Collection info: {:?}", state);
    }
    let img_data = ImageData {
        limit_copies: Some(1),
        auto_changing_rules: Some(vec![
            (9, Action::ChangeImg("Auto change image".to_string())),
            (18, Action::AddMeta("Auto change metadata".to_string())),
        ]),
    };
    let img_links_and_data: Vec<(String, ImageData)> = (0..10)
        .map(|i| (format!("Img-{}", i), img_data.clone()))
        .collect();

    let additional_links = Some(AdditionalLinks {
        external_url: Some("External link".to_string()),
        telegram: None,
        xcom: None,
        medium: None,
        discord: None,
    });

    // Successful creation of a new collection
    let init_nft_payload = NftInit {
        collection_owner: USERS[0].into(),
        config: Config {
            name: "User Collection".to_string(),
            description: "User Collection".to_string(),
            collection_banner: "Collection banner".to_string(),
            collection_logo: "Collection logo".to_string(),
            collection_tags: vec!["tag1".to_string()],
            additional_links,
            royalty: 0,
            user_mint_limit: 3.into(),
            payment_for_mint: 0,
            transferable: Some(0),
            sellable: Some(0),
        },
        img_links_and_data,
        permission_to_mint: None,
        fee_per_uploaded_file: 257_142_857_100,
    };
    sys.mint_to(USERS[0], 100_000_000_000_000);
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        None,
    );
    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    let address_nft = if let StateReply::AllCollections(state) = state_reply {
        assert!(!state.is_empty(), "Collections shouldn't be empty");
        println!("Collections: {:?}", state);
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_list: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_list);
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        assert_eq!(state.collection_owner, USERS[0].into(), "Wrong Admin");
        println!("Collection NFT info: {:?}", state);
    }

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        println!("Collection NFT info: {:?}", state);
        assert_eq!(state.collection_owner, USERS[0].into(), "Wrong Admin");
        let (owner, token_id) = state.owners.get(0).expect("Can't be None");
        assert_eq!(*owner, USERS[1].into(), "Wrong owner");
        assert_eq!(*token_id, vec![0], "Wrong token id");
    }

    let state = get_state(&nft_collection).unwrap();
    assert_ne!(state.tokens[0].1.media_url, "Auto change image".to_string());
    assert_ne!(
        state.tokens[0].1.metadata,
        vec!["Auto change metadata".to_string()]
    );
    sys.spend_blocks(3);
    let state = get_state(&nft_collection).unwrap();
    assert_eq!(state.tokens[0].1.media_url, "Auto change image".to_string());
    assert_ne!(
        state.tokens[0].1.metadata,
        vec!["Auto change metadata".to_string()]
    );
    sys.spend_blocks(6);
    let state = get_state(&nft_collection).unwrap();
    assert_eq!(state.tokens[0].1.media_url, "Auto change image".to_string());
    assert_eq!(
        state.tokens[0].1.metadata,
        vec!["Auto change metadata".to_string()]
    );
}

#[test]
fn check_transferable() {
    let sys = utils::initialize_system();
    let marketplace = Program::init_marketplace(&sys);
    let nft_collection_code_id =
        sys.submit_code("target/wasm32-unknown-unknown/debug/nft.opt.wasm");

    // Successful addition of a new collection
    let name_simple_nft = "Simple NFT".to_string();
    marketplace.add_new_collection(
        ADMINS[0],
        nft_collection_code_id.into_bytes().into(),
        name_simple_nft.clone(),
        None,
    );

    let mut init_nft_payload = get_init_nft_payload(USERS[0].into(), 0, Some(3), 0, None);
    let transferable_time = 9_000;
    init_nft_payload.config.transferable = Some(transferable_time);
    sys.mint_to(USERS[0], 100_000_000_000_000);
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        None,
    );
    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    let address_nft = if let StateReply::AllCollections(state) = state_reply {
        assert!(!state.is_empty(), "Collections shouldn't be empty");
        println!("Collections: {:?}", state);
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_list: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_list);
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        assert_eq!(state.collection_owner, USERS[0].into(), "Wrong Admin");
        println!("Collection NFT info: {:?}", state);
    }

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    // Transfer NFT in the collection
    let res = nft_collection.send(
        USERS[1],
        NftAction::Transfer {
            to: USERS[2].into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());
    check_nft_error(USERS[1], &res, NftError::NotTransferable);

    sys.spend_blocks(3);

    let res = nft_collection.send(
        USERS[1],
        NftAction::Transfer {
            to: USERS[2].into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        println!("!!!!!!!!!! STATE: {:?}", state);
        let (owner, token_id) = state.owners.get(0).expect("Can't be None");
        assert_eq!(*owner, USERS[2].into(), "Wrong owner");
        assert_eq!(*token_id, vec![0], "Wrong token id");
    }
}

#[test]
fn check_payment_for_mint() {
    let sys = utils::initialize_system();
    let marketplace = Program::init_marketplace(&sys);
    let nft_collection_code_id =
        sys.submit_code("target/wasm32-unknown-unknown/debug/nft.opt.wasm");

    let state_reply = marketplace
        .read_state(StateQuery::CollectionsInfo)
        .expect("Unexpected invalid state.");
    if let StateReply::CollectionsInfo(state) = state_reply {
        assert!(state.is_empty(), "Collection info should be empty");
        println!("Collection info: {:?}", state);
    }
    // Successful addition of a new collection
    let name_simple_nft = "Simple NFT".to_string();
    marketplace.add_new_collection(
        ADMINS[0],
        nft_collection_code_id.into_bytes().into(),
        name_simple_nft.clone(),
        None,
    );

    let state_reply = marketplace
        .read_state(StateQuery::CollectionsInfo)
        .expect("Unexpected invalid state.");
    if let StateReply::CollectionsInfo(state) = state_reply {
        assert!(!state.is_empty(), "Collection info shouldn't be empty");
        println!("Collection info: {:?}", state);
    }

    // The payment for mint must be greater than existential deposit (10000000000000)
    let mut init_nft_payload = get_init_nft_payload(USERS[0].into(), 0, Some(3), 0, None);
    let payment_for_mint = 9_000_000_000_000;
    init_nft_payload.config.payment_for_mint = payment_for_mint;
    sys.mint_to(USERS[0], 100_000_000_000_000);
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        Some(NftMarketplaceError::CreationError),
    );
    sys.claim_value_from_mailbox(USERS[0]);
    let balance = sys.balance_of(USERS[0]);
    assert_eq!(balance, 100_000_000_000_000, "Wrong balance");

    // Successful creation of a new collection
    let payment_for_mint = 11_000_000_000_000;
    init_nft_payload.config.payment_for_mint = payment_for_mint;
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        None,
    );

    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    let address_nft = if let StateReply::AllCollections(state) = state_reply {
        assert!(!state.is_empty(), "Collections shouldn't be empty");
        println!("Collections: {:?}", state);
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_list: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_list);
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        assert_eq!(state.collection_owner, USERS[0].into(), "Wrong Admin");
        println!("Collection NFT info: {:?}", state);
    }

    // Successful mint NFT in the new collection
    sys.mint_to(USERS[1], 3 * payment_for_mint);
    let res = marketplace.send_with_value(
        USERS[1],
        NftMarketplaceAction::Mint {
            collection_address: address_nft,
        },
        payment_for_mint + payment_for_mint * 200 / 10_000,
    );
    assert!(!res.main_failed());

    let old_balance = sys.balance_of(USERS[0]);
    sys.claim_value_from_mailbox(USERS[0]);
    let balance = sys.balance_of(USERS[0]);
    assert_eq!(balance - old_balance, payment_for_mint, "Wrong balance");
    let res = marketplace.send_with_value(
        USERS[1],
        NftMarketplaceAction::Mint {
            collection_address: address_nft,
        },
        payment_for_mint,
    );

    let result = &res.decoded_log::<Result<NftMarketplaceEvent, NftMarketplaceError>>();
    println!("RES: {:?}", result);

    assert!(!res.main_failed());

    assert!(res.contains(&(
        USERS[1],
        Err::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceError::WrongValue).encode()
    )));
}

#[test]
fn permission_to_mint() {
    let sys = utils::initialize_system();
    let marketplace = Program::init_marketplace(&sys);
    let nft_collection_code_id =
        sys.submit_code("target/wasm32-unknown-unknown/debug/nft.opt.wasm");

    let state_reply = marketplace
        .read_state(StateQuery::CollectionsInfo)
        .expect("Unexpected invalid state.");
    if let StateReply::CollectionsInfo(state) = state_reply {
        assert!(state.is_empty(), "Collection info should be empty");
        println!("Collection info: {:?}", state);
    }
    // Successful addition of a new collection
    let name_simple_nft = "Simple NFT".to_string();
    marketplace.add_new_collection(
        ADMINS[0],
        nft_collection_code_id.into_bytes().into(),
        name_simple_nft.clone(),
        None,
    );
    let state_reply = marketplace
        .read_state(StateQuery::CollectionsInfo)
        .expect("Unexpected invalid state.");
    if let StateReply::CollectionsInfo(state) = state_reply {
        assert!(!state.is_empty(), "Collection info shouldn't be empty");
        println!("Collection info: {:?}", state);
    }
    // Successful creation of a new collection
    let init_nft_payload = get_init_nft_payload(USERS[0].into(), 0, Some(3), 0, Some(vec![]));
    sys.mint_to(USERS[0], 100_000_000_000_000);
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        None,
    );

    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    let address_nft = if let StateReply::AllCollections(state) = state_reply {
        assert!(!state.is_empty(), "Collections shouldn't be empty");
        println!("Collections: {:?}", state);
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_list: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_list);
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        assert_eq!(state.collection_owner, USERS[0].into(), "Wrong Admin");
    }

    // Succes mint NFT from admin
    marketplace.mint(USERS[0], address_nft, None);

    // Fail mint NFT from user
    marketplace.mint(
        USERS[1],
        address_nft,
        Some(NftMarketplaceError::ErrorFromCollection),
    );

    let res = nft_collection.send(
        USERS[0],
        NftAction::AddUsersForMint {
            users: vec![USERS[1].into()],
        },
    );
    assert!(!res.main_failed());

    // Success mint
    marketplace.mint(USERS[1], address_nft, None);

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        println!("Collection NFT info: {:?}", state);
        assert_eq!(state.collection_owner, USERS[0].into(), "Wrong Admin");
        let (owner, token_id) = state.owners.get(1).expect("Can't be None");
        assert_eq!(*owner, USERS[1].into(), "Wrong owner");
        assert_eq!(*token_id, vec![1], "Wrong token id");
    }
    let res = nft_collection.send(
        USERS[0],
        NftAction::DeleteUserForMint {
            user: USERS[1].into(),
        },
    );
    assert!(!res.main_failed());

    // Fail mint NFT in the new collection
    marketplace.mint(
        USERS[1],
        address_nft,
        Some(NftMarketplaceError::ErrorFromCollection),
    );
}

fn get_state(nft_collection: &Program) -> Option<NftState> {
    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");
    if let StateReplyNft::All(state) = state_reply {
        return Some(state);
    }
    None
}

// TODO
// #[test]
// fn admin_features() {

// }

// #[test]
// fn check() {
//     let input = "000080ee3600000000008c9de8de3b0000000000000000000000c800c800";
//     let decoded = hex::decode(input).expect("Decoding failed");
//     let mut res: &[u8] = &decoded;
//     let result = Result::<NftMarketplaceEvent, NftMarketplaceError>::decode(&mut res).ok();
//     println!("RES: {:?}", result);
// }
