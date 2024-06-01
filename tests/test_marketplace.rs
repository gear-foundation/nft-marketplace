use crate::utils::*;
use utils::prelude::*;
mod utils;
use gtest::Program;
use nft_io::{StateQuery as StateQueryNft, StateReply as StateReplyNft};
use nft_marketplace_io::*;

const USERS: &[u64] = &[5, 6, 7, 8];

#[test]
fn create_success() {
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
    // Сreating a new type of collection
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
        // assert_eq!(state[0].1.collection_id, 1,  "Collection id should be equal by 0");
        println!("Collection info: {:?}", state);
    }

    // Add admin
    let users: Vec<ActorId> = vec![100.into(), 101.into()];
    marketplace.add_admin(ADMINS[0], users, None);

    let state_reply = marketplace
        .read_state(StateQuery::Admins)
        .expect("Unexpected invalid state.");
    if let StateReply::Admins(admins) = state_reply {
        assert_eq!(
            admins,
            vec![123.into(), 100.into(), 101.into()],
            "Wrong admins"
        );
    }

    // Delete admin
    marketplace.delete_admin(ADMINS[0], 100.into(), None);

    let state_reply = marketplace
        .read_state(StateQuery::Admins)
        .expect("Unexpected invalid state.");
    if let StateReply::Admins(admins) = state_reply {
        assert_eq!(admins, vec![123.into(), 101.into()], "Wrong admins");
    }

    // Update config
    marketplace.update_config(
        ADMINS[0],
        Some(200_000_000_000),
        None,
        None,
        None,
        None,
        None,
        Some(7_200_000),
        None,
        None,
        None,
        None,
    );
    let state_reply = marketplace
        .read_state(StateQuery::Config)
        .expect("Unexpected invalid state.");
    if let StateReply::Config(config) = state_reply {
        assert_eq!(config.gas_for_creation, 200_000_000_000, "Wrong config");
        assert_eq!(
            config.time_between_create_collections, 7_200_000,
            "Wrong config"
        );
        println!("CONFIG: {:?}", config);
    }

    let init_nft_payload = get_init_nft_payload(USERS[0].into(), 0, Some(3), 0, None);

    sys.mint_to(USERS[0], 100_000_000_000_000);
    // Create collection
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        None,
    );
    // Check marketplace balance
    let balance = sys.balance_of(1);
    assert_eq!(balance, 10_000_000_000_000, "Wrong balance");

    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    if let StateReply::AllCollections(state) = state_reply {
        assert!(!state.is_empty(), "Collections shouldn't be empty");
        assert_eq!(state[0].1 .1, USERS[0].into(), "Wrong owner of collection");
    }
    sys.spend_blocks(7200);
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
    if let StateReply::AllCollections(state) = state_reply {
        assert!(!state.is_empty(), "Collections shouldn't be empty");
        assert_eq!(state.len(), 2, "The number of collection is wrong");
    }

    // Delete collection
    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    let address_nft = if let StateReply::AllCollections(state) = state_reply {
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };
    marketplace.delete_collection(ADMINS[0], address_nft, None);
    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    if let StateReply::AllCollections(state) = state_reply {
        assert_eq!(state.len(), 1, "The number of collection is wrong");
    }
}

#[test]
fn create_failures() {
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
    // not admin
    let name_simple_nft = "Simple NFT".to_string();
    marketplace.add_new_collection(
        USERS[0],
        nft_collection_code_id.into_bytes().into(),
        name_simple_nft.clone(),
        Some(NftMarketplaceError::AccessDenied),
    );

    marketplace.add_admin(
        USERS[0],
        vec![100.into()],
        Some(NftMarketplaceError::AccessDenied),
    );

    marketplace.update_config(
        USERS[0],
        Some(200_000_000_000),
        None,
        None,
        None,
        None,
        None,
        Some(7_200_000),
        None,
        None,
        None,
        Some(NftMarketplaceError::AccessDenied),
    );

    // Add type of collection
    marketplace.add_new_collection(
        ADMINS[0],
        nft_collection_code_id.into_bytes().into(),
        name_simple_nft.clone(),
        None,
    );

    let init_nft_payload = get_init_nft_payload(USERS[0].into(), 0, Some(3), 0, None);

    // Сan only create one collection per hour
    sys.mint_to(USERS[0], 100_000_000_000_000);
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        None,
    );
    sys.spend_blocks(1100); // less than 3,600
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        Some(NftMarketplaceError::DeadlineError),
    );

    // Delete collection
    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    let address_nft = if let StateReply::AllCollections(state) = state_reply {
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };
    marketplace.delete_collection(
        USERS[0],
        1.into(),
        Some(NftMarketplaceError::WrongCollectionAddress),
    );
    marketplace.delete_collection(
        USERS[1],
        address_nft,
        Some(NftMarketplaceError::AccessDenied),
    );

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    marketplace.delete_collection(
        USERS[0],
        address_nft,
        Some(NftMarketplaceError::AccessDenied),
    );
}

#[test]
fn sale_success() {
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
    // Сreating a new type of collection
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
        // assert_eq!(state[0].1.collection_id, 1,  "Collection id should be equal by 0");
        println!("Collection info: {:?}", state);
    }

    // Create collection
    let royalty = 1_000;
    let init_nft_payload = get_init_nft_payload(USERS[0].into(), royalty, Some(3), 0, None);
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
        assert_eq!(state[0].1 .1, USERS[0].into(), "Wrong owner of collection");
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_2: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_2);

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    // Successful approve NFT in the collection
    let addres_marketplace: [u8; 32] = marketplace.id().into();
    let res = nft_collection.send(
        USERS[1],
        nft_io::NftAction::Approve {
            to: addres_marketplace.into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    let price = 150_000_000_000_000;

    marketplace.sale(USERS[1], address_nft, 0, price, None);

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        assert!(!state.sales.is_empty());
        println!("STATE: {:?}", state);
    }

    sys.mint_to(USERS[2], price);
    // sys.mint_to(1, 100_000_000_000_000);
    marketplace.buy(USERS[2], address_nft, 0, price, None);

    let percent_to_collection_owner = price * royalty as u128 / 10_000;
    let percent_to_marketplace = price * 200 as u128 / 10_000;
    sys.claim_value_from_mailbox(USERS[1]);
    let balance = sys.balance_of(USERS[1]);
    assert_eq!(
        balance,
        price - percent_to_collection_owner - percent_to_marketplace,
        "Wrong balance"
    );

    let old_balance = sys.balance_of(USERS[0]);
    sys.claim_value_from_mailbox(USERS[0]);
    let balance = sys.balance_of(USERS[0]);
    assert_eq!(
        balance - old_balance,
        percent_to_collection_owner,
        "Wrong balance"
    );

    let balance = sys.balance_of(1);

    assert_eq!(
        balance,
        percent_to_marketplace + 10_000_000_000_000,
        "Wrong balance"
    );

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");

    if let StateReplyNft::All(state) = state_reply {
        println!("STATE: {:?}", state);
        let token = state.tokens.get(0).expect("Can't be None");
        assert_eq!(token.0, 0);
        assert_eq!(token.1.owner, USERS[2].into())
    }
}

#[test]
fn sale_failures() {
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
    // Сreating a new type of collection
    let name_simple_nft = "Simple NFT".to_string();
    marketplace.add_new_collection(
        ADMINS[0],
        nft_collection_code_id.into_bytes().into(),
        name_simple_nft.clone(),
        None,
    );

    // Create collection
    let royalty = 1_000;
    let init_nft_payload = get_init_nft_payload(USERS[0].into(), royalty, Some(3), 0, None);

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
        assert_eq!(state[0].1 .1, USERS[0].into(), "Wrong owner of collection");
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_2: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_2);

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    // low price
    marketplace.sale(
        USERS[1],
        address_nft,
        0,
        9_000_000_000_000,
        Some(NftMarketplaceError::LessThanMinimumValueForTrade),
    );

    // Only owner can send this action
    let price = 150_000_000_000_000;
    marketplace.sale(
        USERS[2],
        address_nft,
        0,
        price,
        Some(NftMarketplaceError::AccessDenied),
    );

    // No approve to the marketplace
    marketplace.sale(
        USERS[1],
        address_nft,
        0,
        price,
        Some(NftMarketplaceError::NoApproveToMarketplace),
    );

    // wrong collection address
    marketplace.sale(
        USERS[1],
        1.into(),
        0,
        price,
        Some(NftMarketplaceError::WrongCollectionAddress),
    );

    // Successful approve NFT in the collection
    let addres_marketplace: [u8; 32] = marketplace.id().into();
    let res = nft_collection.send(
        USERS[1],
        nft_io::NftAction::Approve {
            to: addres_marketplace.into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    // Success
    marketplace.sale(USERS[1], address_nft, 0, price, None);

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        assert!(!state.sales.is_empty());
    }

    // is already on sale
    marketplace.sale(
        USERS[1],
        address_nft,
        0,
        price,
        Some(NftMarketplaceError::AlreadyOnSale),
    );

    // wrong owner
    marketplace.cancel_sale(
        USERS[2],
        address_nft,
        0,
        Some(NftMarketplaceError::AccessDenied),
    );

    // Wrong token_id
    marketplace.cancel_sale(
        USERS[1],
        address_nft,
        1,
        Some(NftMarketplaceError::SaleDoesNotExist),
    );

    // value is less than the price
    sys.mint_to(USERS[2], price);

    marketplace.buy(
        USERS[2],
        address_nft,
        0,
        price - 1,
        Some(NftMarketplaceError::ValueIsLessThanPrice),
    );

    sys.claim_value_from_mailbox(USERS[2]);
    let balance = sys.balance_of(USERS[2]);
    assert_eq!(balance, price - 1, "Wrong balance");

    // sale does not exist
    marketplace.buy(
        USERS[2],
        address_nft,
        1,
        price - 1,
        Some(NftMarketplaceError::SaleDoesNotExist),
    );
}

#[test]
fn auction_success() {
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
    // Сreating a new type of collection
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
        // assert_eq!(state[0].1.collection_id, 1,  "Collection id should be equal by 0");
        println!("Collection info: {:?}", state);
    }

    // Create collection

    let royalty = 1_000;
    let init_nft_payload = get_init_nft_payload(USERS[0].into(), royalty, Some(3), 0, None);
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
        assert_eq!(state[0].1 .1, USERS[0].into(), "Wrong owner of collection");
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_2: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_2);

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    // Successful approve NFT in the collection
    let addres_marketplace: [u8; 32] = marketplace.id().into();
    let res = nft_collection.send(
        USERS[1],
        nft_io::NftAction::Approve {
            to: addres_marketplace.into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    let duration = 10;
    marketplace.create_auction(USERS[1], address_nft, 0, 10_300_000_000_000, duration, None);

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        println!("STATE: {:?}", state);
    }

    let current_balance = 200_000_000_000_000;
    sys.mint_to(USERS[2], current_balance);
    sys.mint_to(USERS[3], current_balance);

    marketplace.add_bid(USERS[2], address_nft, 0, 100_000_000_000_000, None);

    let balance = sys.balance_of(USERS[2]);
    assert_eq!(balance, 100_000_000_000_000, "Wrong balance");

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        assert_eq!(state.auctions[0].1.current_winner, USERS[2].into());
        println!("STATE: {:?}", state);
    }

    let final_bid = 150_000_000_000_000;
    marketplace.add_bid(USERS[3], address_nft, 0, final_bid, None);

    let balance = sys.balance_of(USERS[3]);
    assert_eq!(balance, current_balance - final_bid, "Wrong balance");
    sys.claim_value_from_mailbox(USERS[2]);
    let balance = sys.balance_of(USERS[2]);
    assert_eq!(balance, current_balance, "Wrong balance");

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        assert_eq!(state.auctions[0].1.current_winner, USERS[3].into());
        println!("STATE: {:?}", state);
    }

    sys.spend_blocks(duration + 1);

    let percent_to_collection_owner = final_bid * royalty as u128 / 10_000;
    let percent_to_marketplace = final_bid * 200 as u128 / 10_000;
    sys.claim_value_from_mailbox(USERS[1]);
    let balance = sys.balance_of(USERS[1]);
    assert_eq!(
        balance,
        final_bid - percent_to_collection_owner - percent_to_marketplace,
        "Wrong balance"
    );

    let old_balance = sys.balance_of(USERS[0]);
    sys.claim_value_from_mailbox(USERS[0]);
    let balance = sys.balance_of(USERS[0]);
    assert_eq!(
        balance - old_balance,
        percent_to_collection_owner,
        "Wrong balance"
    );

    let balance = sys.balance_of(1);
    assert_eq!(
        balance,
        percent_to_marketplace + 10_000_000_000_000,
        "Wrong balance"
    );

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");

    if let StateReplyNft::All(state) = state_reply {
        println!("STATE: {:?}", state);
        let token = state.tokens.get(0).expect("Can't be None");
        assert_eq!(token.0, 0);
        assert_eq!(token.1.owner, USERS[3].into())
    }
}

#[test]
fn auction_cancel() {
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
    // Сreating a new type of collection
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
        // assert_eq!(state[0].1.collection_id, 1,  "Collection id should be equal by 0");
        println!("Collection info: {:?}", state);
    }

    // Create collection
    let royalty = 1_000;
    let init_nft_payload = get_init_nft_payload(USERS[0].into(), royalty, Some(3), 0, None);
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
        assert_eq!(state[0].1 .1, USERS[0].into(), "Wrong owner of collection");
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_2: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_2);

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    // Successful approve NFT in the collection
    let addres_marketplace: [u8; 32] = marketplace.id().into();
    let res = nft_collection.send(
        USERS[1],
        nft_io::NftAction::Approve {
            to: addres_marketplace.into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    // Create auction
    let duration_1 = 5;
    marketplace.create_auction(
        USERS[1],
        address_nft,
        0,
        10_300_000_000_000,
        duration_1,
        None,
    );

    // Add bid
    let current_balance = 200_000_000_000_000;
    sys.mint_to(USERS[2], current_balance);

    marketplace.add_bid(USERS[2], address_nft, 0, 100_000_000_000_000, None);
    let balance = sys.balance_of(USERS[2]);
    assert_eq!(balance, 100_000_000_000_000, "Wrong balance");

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        assert_eq!(state.auctions[0].1.current_winner, USERS[2].into());
    }

    // Cancel auction
    marketplace.cancel_auction(USERS[1], address_nft, 0, None);

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        assert!(state.auctions.is_empty());
    }

    // Check balance
    sys.claim_value_from_mailbox(USERS[2]);
    let balance = sys.balance_of(USERS[2]);
    assert_eq!(balance, 200_000_000_000_000, "Wrong balance");

    // Successful approve
    let addres_marketplace: [u8; 32] = marketplace.id().into();
    let res = nft_collection.send(
        USERS[1],
        nft_io::NftAction::Approve {
            to: addres_marketplace.into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    // Create the same auction
    let duration_2 = 10;
    marketplace.create_auction(
        USERS[1],
        address_nft,
        0,
        10_300_000_000_000,
        duration_2,
        None,
    );

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        assert!(!state.auctions.is_empty());
    }

    sys.spend_blocks(duration_1 + 1);

    // the delayed message from the first version of the auction will come,
    // but it should end with an error, because the auction was canceled and a new one was created.
    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        assert!(!state.auctions.is_empty());
    }

    sys.spend_blocks(duration_2 - duration_1);

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        assert!(state.auctions.is_empty());
    }

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");

    if let StateReplyNft::All(state) = state_reply {
        println!("STATE: {:?}", state);
        let token = state.tokens.get(0).expect("Can't be None");
        assert_eq!(token.0, 0);
        assert_eq!(token.1.owner, USERS[1].into())
    }
}

#[test]
fn auction_failures() {
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
    // Сreating a new type of collection
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
        // assert_eq!(state[0].1.collection_id, 1,  "Collection id should be equal by 0");
        println!("Collection info: {:?}", state);
    }

    // Create collection
    let royalty = 1_000;
    let init_nft_payload = get_init_nft_payload(USERS[0].into(), royalty, Some(3), 0, None);
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
        assert_eq!(state[0].1 .1, USERS[0].into(), "Wrong owner of collection");
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_2: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_2);

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    // low price
    let duration = 10;

    marketplace.create_auction(
        USERS[1],
        address_nft,
        0,
        9_000_000_000_000,
        duration,
        Some(NftMarketplaceError::LessThanMinimumValueForTrade),
    );

    // Only token owner can send
    marketplace.create_auction(
        USERS[2],
        address_nft,
        0,
        10_300_000_000_000,
        duration,
        Some(NftMarketplaceError::AccessDenied),
    );

    // No approve
    marketplace.create_auction(
        USERS[1],
        address_nft,
        0,
        10_300_000_000_000,
        duration,
        Some(NftMarketplaceError::NoApproveToMarketplace),
    );

    // Successful approve NFT in the collection
    let addres_marketplace: [u8; 32] = marketplace.id().into();
    let res = nft_collection.send(
        USERS[1],
        nft_io::NftAction::Approve {
            to: addres_marketplace.into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    // Successful create auction
    marketplace.create_auction(USERS[1], address_nft, 0, 11_000_000_000_000, duration, None);

    // No auction with this collection address and token id
    marketplace.cancel_auction(
        USERS[1],
        address_nft,
        1,
        Some(NftMarketplaceError::ThereIsNoSuchAuction),
    );

    // Only the creator of the auction can send cancel_auction
    marketplace.cancel_auction(
        USERS[2],
        address_nft,
        0,
        Some(NftMarketplaceError::AccessDenied),
    );

    let current_balance = 20_000_000_000_000;
    sys.mint_to(USERS[2], current_balance);

    marketplace.add_bid(
        USERS[2],
        address_nft,
        0,
        10_000_000_000_000,
        Some(NftMarketplaceError::LessOrEqualThanBid),
    );

    sys.claim_value_from_mailbox(USERS[2]);
    let balance = sys.balance_of(USERS[2]);
    assert_eq!(balance, 20_000_000_000_000, "Wrong balance");

    sys.spend_blocks(duration + 1);
    sys.mint_to(USERS[3], 15_000_000_000_000);
    marketplace.add_bid(
        USERS[3],
        address_nft,
        0,
        15_000_000_000_000,
        Some(NftMarketplaceError::ThereIsNoSuchAuction),
    );

    sys.claim_value_from_mailbox(USERS[3]);
    let balance = sys.balance_of(USERS[3]);
    assert_eq!(balance, 15_000_000_000_000, "Wrong balance");
}

#[test]
fn offer_success() {
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
    // Сreating a new type of collection
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
        // assert_eq!(state[0].1.collection_id, 1,  "Collection id should be equal by 0");
        println!("Collection info: {:?}", state);
    }

    // Create collection
    let royalty = 1_000;
    let init_nft_payload = get_init_nft_payload(USERS[0].into(), royalty, Some(3), 0, None);
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
        assert_eq!(state[0].1 .1, USERS[0].into(), "Wrong owner of collection");
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_2: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_2);

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    // Successful approve NFT in the collection
    let addres_marketplace: [u8; 32] = marketplace.id().into();
    let res = nft_collection.send(
        USERS[1],
        nft_io::NftAction::Approve {
            to: addres_marketplace.into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    let offer_price = 150_000_000_000_000;
    sys.mint_to(USERS[2], offer_price);
    marketplace.create_offer(USERS[2], address_nft, 0, offer_price, None);

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        println!("STATE: {:?}", state);
        let creator: ActorId = USERS[2].into();
        assert_eq!(creator, state.offers[0].0.creator);
        assert_eq!(offer_price, state.offers[0].1);
    }

    marketplace.accept_offer(USERS[1], address_nft, 0, USERS[2].into(), None);
    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        println!("STATE: {:?}", state);
        assert!(state.offers.is_empty());
    }

    let percent_to_collection_owner = offer_price * royalty as u128 / 10_000;
    let percent_to_marketplace = offer_price * 200 as u128 / 10_000;

    sys.claim_value_from_mailbox(USERS[1]);
    let balance = sys.balance_of(USERS[1]);
    assert_eq!(
        balance,
        offer_price - percent_to_collection_owner - percent_to_marketplace,
        "Wrong balance"
    );

    let old_balance = sys.balance_of(USERS[0]);
    sys.claim_value_from_mailbox(USERS[0]);
    let balance = sys.balance_of(USERS[0]);
    assert_eq!(
        balance - old_balance,
        percent_to_collection_owner,
        "Wrong balance"
    );

    let balance = sys.balance_of(1);
    assert_eq!(
        balance,
        percent_to_marketplace + 10_000_000_000_000,
        "Wrong balance"
    );

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");

    if let StateReplyNft::All(state) = state_reply {
        println!("STATE: {:?}", state);
        let token = state.tokens.get(0).expect("Can't be None");
        assert_eq!(token.0, 0);
        assert_eq!(token.1.owner, USERS[2].into())
    }
}

#[test]
fn offer_failures() {
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
    // Сreating a new type of collection
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
        // assert_eq!(state[0].1.collection_id, 1,  "Collection id should be equal by 0");
        println!("Collection info: {:?}", state);
    }

    // Create collection
    let royalty = 1_000;
    let init_nft_payload = get_init_nft_payload(USERS[0].into(), royalty, Some(3), 0, None);

    sys.mint_to(USERS[0], 100_000_000_000_000);
    marketplace.create_collection(
        USERS[0],
        name_simple_nft.clone(),
        init_nft_payload.encode(),
        10_000_000_000_000,
        None,
    );
    let balance = sys.balance_of(1);
    assert_eq!(balance, 10_000_000_000_000, "Wrong balance");
    let state_reply = marketplace
        .read_state(StateQuery::AllCollections)
        .expect("Unexpected invalid state.");
    let address_nft = if let StateReply::AllCollections(state) = state_reply {
        assert!(!state.is_empty(), "Collections shouldn't be empty");
        assert_eq!(state[0].1 .1, USERS[0].into(), "Wrong owner of collection");
        state[0].0
    } else {
        assert!(false, "Unexpected StateReply variant");
        0.into()
    };

    let address_nft_2: [u8; 32] = address_nft.into();
    let nft_collection = sys.get_program(address_nft_2);

    // NonFungibleToken: token does not exist
    let offer_price = 150_000_000_000_000;
    sys.mint_to(USERS[2], offer_price);
    marketplace.create_offer(
        USERS[2],
        address_nft,
        0,
        offer_price,
        Some(NftMarketplaceError::ErrorFromCollection),
    );

    sys.claim_value_from_mailbox(USERS[2]);
    let balance = sys.balance_of(USERS[2]);
    assert_eq!(balance, offer_price, "Wrong balance");

    // wrong collection address
    marketplace.create_offer(
        USERS[2],
        1.into(),
        0,
        offer_price,
        Some(NftMarketplaceError::WrongCollectionAddress),
    );
    sys.claim_value_from_mailbox(USERS[2]);
    let balance = sys.balance_of(USERS[2]);
    assert_eq!(balance, offer_price, "Wrong balance");

    // Successful mint NFT in the new collection
    marketplace.mint(USERS[1], address_nft, None);

    // Successful approve NFT in the collection
    let addres_marketplace: [u8; 32] = marketplace.id().into();
    let res = nft_collection.send(
        USERS[1],
        nft_io::NftAction::Approve {
            to: addres_marketplace.into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    let price = 150_000_000_000_000;

    marketplace.sale(USERS[1], address_nft, 0, price, None);

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        println!("STATE: {:?}", state);
    }

    let offer_price = 150_000_000_000_000;
    sys.mint_to(USERS[2], offer_price);
    marketplace.create_offer(USERS[2], address_nft, 0, offer_price, None);

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        println!("STATE: {:?}", state);
        let creator: ActorId = USERS[2].into();
        assert_eq!(creator, state.offers[0].0.creator);
        assert_eq!(offer_price, state.offers[0].1);
    }

    marketplace.accept_offer(
        USERS[1],
        address_nft,
        0,
        USERS[2].into(),
        Some(NftMarketplaceError::AlreadyOnSale),
    );
    marketplace.cancel_sale(USERS[1], address_nft, 0, None);

    let res = nft_collection.send(
        USERS[1],
        nft_io::NftAction::Approve {
            to: addres_marketplace.into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    let duration = 10;
    marketplace.create_auction(USERS[1], address_nft, 0, 10_300_000_000_000, duration, None);

    marketplace.accept_offer(
        USERS[1],
        address_nft,
        0,
        USERS[2].into(),
        Some(NftMarketplaceError::AlreadyOnAuction),
    );
    marketplace.cancel_auction(USERS[1], address_nft, 0, None);

    let res = nft_collection.send(
        USERS[1],
        nft_io::NftAction::Approve {
            to: addres_marketplace.into(),
            token_id: 0,
        },
    );
    assert!(!res.main_failed());

    marketplace.accept_offer(USERS[1], address_nft, 0, USERS[2].into(), None);

    let state_reply = marketplace
        .read_state(StateQuery::All)
        .expect("Unexpected invalid state.");
    if let StateReply::All(state) = state_reply {
        println!("STATE: {:?}", state);
        assert!(state.offers.is_empty());
    }

    let percent_to_collection_owner = offer_price * royalty as u128 / 10_000;
    let percent_to_marketplace = offer_price * 200 as u128 / 10_000;
    sys.claim_value_from_mailbox(USERS[1]);
    let balance = sys.balance_of(USERS[1]);
    assert_eq!(
        balance,
        offer_price - percent_to_collection_owner - percent_to_marketplace,
        "Wrong balance"
    );

    let old_balance = sys.balance_of(USERS[0]);
    sys.claim_value_from_mailbox(USERS[0]);
    let balance = sys.balance_of(USERS[0]);
    assert_eq!(
        balance - old_balance,
        percent_to_collection_owner,
        "Wrong balance"
    );

    let balance = sys.balance_of(1);
    assert_eq!(
        balance,
        percent_to_marketplace + 10_000_000_000_000,
        "Wrong balance"
    );

    let state_reply = nft_collection
        .read_state(StateQueryNft::All)
        .expect("Unexpected invalid state.");

    if let StateReplyNft::All(state) = state_reply {
        println!("STATE: {:?}", state);
        let token = state.tokens.get(0).expect("Can't be None");
        assert_eq!(token.0, 0);
        assert_eq!(token.1.owner, USERS[2].into())
    }
}
