use crate::codec::Error;
use gstd::{ActorId, CodeId, Encode};
use gtest::{Program, RunResult, System};
use music_nft_io::{
    Config as MusicConfig, ImageData as MusicImageData, Links, ListenCapability, MusicNftError,
    MusicNftEvent, MusicNftInit,
};
use nft_io::{Config, ImageData, NftError, NftEvent, NftInit};
use nft_marketplace_io::{
    NftMarketplaceAction, NftMarketplaceError, NftMarketplaceEvent, NftMarketplaceInit, Offer,
};

mod common;
pub mod prelude;

pub use common::initialize_system;

pub const ADMINS: [u64; 2] = [123, 321];

pub trait NftMarketplace {
    fn init_marketplace(sys: &System) -> Program<'_>;
    fn add_new_collection(
        &self,
        admin: u64,
        code_id: CodeId,
        type_name: String,
        error: Option<NftMarketplaceError>,
    );
    fn create_collection(
        &self,
        user: u64,
        type_name: String,
        payload: Vec<u8>,
        value: u128,
        error: Option<NftMarketplaceError>,
    );
    fn mint(&self, user: u64, collection_address: ActorId, error: Option<NftMarketplaceError>);
    fn sale(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        price: u128,
        error: Option<NftMarketplaceError>,
    );
    fn cancel_sale(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        error: Option<NftMarketplaceError>,
    );
    fn create_auction(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        min_price: u128,
        duration: u32,
        error: Option<NftMarketplaceError>,
    );
    fn cancel_auction(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        error: Option<NftMarketplaceError>,
    );
    fn add_bid(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        price: u128,
        error: Option<NftMarketplaceError>,
    );
    fn buy(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        price: u128,
        error: Option<NftMarketplaceError>,
    );
    fn create_offer(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        value: u128,
        error: Option<NftMarketplaceError>,
    );
    fn accept_offer(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        creator: ActorId,
        error: Option<NftMarketplaceError>,
    );
    fn add_admin(&self, admin: u64, users: Vec<ActorId>, error: Option<NftMarketplaceError>);
    fn update_config(
        &self,
        admin: u64,
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
        error: Option<NftMarketplaceError>,
    );
    fn delete_collection(
        &self,
        admin: u64,
        collection_address: ActorId,
        error: Option<NftMarketplaceError>,
    );
    fn delete_admin(&self, admin: u64, user: ActorId, error: Option<NftMarketplaceError>);
}

impl NftMarketplace for Program<'_> {
    fn init_marketplace(sys: &System) -> Program<'_> {
        let marketplace = Program::current(sys);
        let init_payload = NftMarketplaceInit {
            gas_for_creation: 5_000_000_000,
            gas_for_mint: 7_000_000_000,
            gas_for_transfer_token: 4_000_000_000,
            gas_for_close_auction: 15_000_000_000,
            gas_for_delete_collection: 3_000_000_000,
            gas_for_get_info: 5_000_000_000,
            time_between_create_collections: 3_600_000, // 1 hour in milliseconds
            royalty_to_marketplace_for_trade: 200,
            royalty_to_marketplace_for_mint: 200,
            ms_in_block: 3_000,
            fee_per_uploaded_file: 257_142_857_100,
            max_creator_royalty: 1_000,
            max_number_of_images: 10_000,
        };
        let res = marketplace.send(ADMINS[0], init_payload);
        assert!(!res.main_failed());
        let res = marketplace.send(ADMINS[0], NftMarketplaceAction::AllowMessage(true));
        assert!(!res.main_failed());
        let res = marketplace.send(ADMINS[0], NftMarketplaceAction::AllowCreateCollection(true));
        assert!(!res.main_failed());
        marketplace
    }
    fn add_new_collection(
        &self,
        admin: u64,
        code_id: CodeId,
        type_name: String,
        error: Option<NftMarketplaceError>,
    ) {
        let type_description = String::from("My Collection");
        let meta_link = String::from("My Meta");

        let res = self.send(
            admin,
            NftMarketplaceAction::AddNewCollection {
                code_id,
                meta_link: meta_link.clone(),
                type_name: type_name.clone(),
                type_description: type_description.clone(),
                allow_create: true,
            },
        );
        assert!(!res.main_failed());

        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::NewCollectionAdded {
                code_id,
                meta_link,
                type_name,
                type_description,
            })
        };
        assert!(res.contains(&(admin, reply.encode())));
    }
    fn create_collection(
        &self,
        user: u64,
        type_name: String,
        payload: Vec<u8>,
        value: u128,
        error: Option<NftMarketplaceError>,
    ) {
        let res = self.send_with_value(
            user,
            NftMarketplaceAction::CreateCollection { type_name, payload },
            value,
        );
        assert!(!res.main_failed());
        if let Some(error) = error {
            let reply = Err::<NftMarketplaceEvent, NftMarketplaceError>(error);
            assert!(res.contains(&(user, reply.encode())));
        }
    }
    fn mint(&self, user: u64, collection_address: ActorId, error: Option<NftMarketplaceError>) {
        let res = self.send(user, NftMarketplaceAction::Mint { collection_address });
        assert!(!res.main_failed());

        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::Minted {
                collection_address,
                minter: user.into(),
            })
        };
        assert!(res.contains(&(user, reply.encode())));
    }
    fn sale(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        price: u128,
        error: Option<NftMarketplaceError>,
    ) {
        let res = self.send(
            user,
            NftMarketplaceAction::SaleNft {
                collection_address,
                token_id,
                price,
            },
        );
        assert!(!res.main_failed());

        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::SaleNft {
                collection_address,
                token_id,
                price,
                owner: user.into(),
            })
        };
        assert!(res.contains(&(user, reply.encode())));
    }
    fn cancel_sale(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        error: Option<NftMarketplaceError>,
    ) {
        let res = self.send(
            user,
            NftMarketplaceAction::CancelSaleNft {
                collection_address,
                token_id,
            },
        );
        assert!(!res.main_failed());

        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::SaleNftCanceled {
                collection_address,
                token_id,
            })
        };
        assert!(res.contains(&(user, reply.encode())));
    }
    fn create_auction(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        min_price: u128,
        duration: u32,
        error: Option<NftMarketplaceError>,
    ) {
        let res = self.send(
            user,
            NftMarketplaceAction::CreateAuction {
                collection_address,
                token_id,
                min_price,
                duration,
            },
        );
        assert!(!res.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::AuctionCreated {
                collection_address,
                token_id,
                min_price,
                duration_ms: (duration + 1) * 3000,
            })
        };
        let result = &res.decoded_log::<Result<NftMarketplaceEvent, NftMarketplaceError>>();
        println!("RES: {:?}", result);
        assert!(res.contains(&(user, reply.encode())));
    }
    fn cancel_auction(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        error: Option<NftMarketplaceError>,
    ) {
        let res = self.send(
            user,
            NftMarketplaceAction::CancelAuction {
                collection_address,
                token_id,
            },
        );
        assert!(!res.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::AuctionCanceled {
                collection_address,
                token_id,
            })
        };
        assert!(res.contains(&(user, reply.encode())));
    }
    fn add_bid(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        price: u128,
        error: Option<NftMarketplaceError>,
    ) {
        let res = self.send_with_value(
            user,
            NftMarketplaceAction::AddBid {
                collection_address,
                token_id,
            },
            price,
        );
        assert!(!res.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::BidAdded {
                collection_address,
                token_id,
                current_price: price,
            })
        };
        assert!(res.contains(&(user, reply.encode())));
    }
    fn buy(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        price: u128,
        error: Option<NftMarketplaceError>,
    ) {
        let res = self.send_with_value(
            user,
            NftMarketplaceAction::BuyNft {
                collection_address,
                token_id,
            },
            price,
        );
        assert!(!res.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::NftSold {
                collection_address,
                token_id,
                price,
                current_owner: user.into(),
            })
        };
        assert!(res.contains(&(user, reply.encode())));
    }

    fn create_offer(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        value: u128,
        error: Option<NftMarketplaceError>,
    ) {
        let res = self.send_with_value(
            user,
            NftMarketplaceAction::CreateOffer {
                collection_address,
                token_id,
            },
            value,
        );
        assert!(!res.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::OfferCreated {
                collection_address,
                token_id,
                price: value,
            })
        };
        assert!(res.contains(&(user, reply.encode())));
    }

    fn accept_offer(
        &self,
        user: u64,
        collection_address: ActorId,
        token_id: u64,
        creator: ActorId,
        error: Option<NftMarketplaceError>,
    ) {
        let offer = Offer {
            collection_address,
            token_id,
            creator,
        };
        let res = self.send(
            user,
            NftMarketplaceAction::AcceptOffer {
                offer: offer.clone(),
            },
        );
        assert!(!res.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::OfferAccepted { offer })
        };
        assert!(res.contains(&(user, reply.encode())));
    }
    fn add_admin(&self, admin: u64, users: Vec<ActorId>, error: Option<NftMarketplaceError>) {
        let res = self.send(
            admin,
            NftMarketplaceAction::AddAdmins {
                users: users.clone(),
            },
        );
        assert!(!res.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::AdminsAdded { users })
        };
        assert!(res.contains(&(admin, reply.encode())));
    }
    fn update_config(
        &self,
        admin: u64,
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
        error: Option<NftMarketplaceError>,
    ) {
        let res = self.send(
            admin,
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
                fee_per_uploaded_file: None,
                max_creator_royalty: None,
                max_number_of_images: None,
            },
        );
        assert!(!res.main_failed());
        if let Some(error) = error {
            let reply = Err::<NftMarketplaceEvent, NftMarketplaceError>(error);
            assert!(res.contains(&(admin, reply.encode())));
        }
    }

    fn delete_collection(
        &self,
        admin: u64,
        collection_address: ActorId,
        error: Option<NftMarketplaceError>,
    ) {
        let res = self.send(
            admin,
            NftMarketplaceAction::DeleteCollection { collection_address },
        );
        assert!(!res.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::CollectionDeleted { collection_address })
        };
        assert!(res.contains(&(admin, reply.encode())));
    }
    fn delete_admin(&self, admin: u64, user: ActorId, error: Option<NftMarketplaceError>) {
        let res = self.send(admin, NftMarketplaceAction::DeleteAdmin { user });
        assert!(!res.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(NftMarketplaceEvent::AdminDeleted { user })
        };
        assert!(res.contains(&(admin, reply.encode())));
    }
}

pub fn check_nft_error(from: u64, result: &RunResult, error: NftError) {
    assert!(result.contains(&(from, Err::<NftEvent, NftError>(error).encode())));
}
pub fn check_music_nft_error(from: u64, result: &RunResult, error: MusicNftError) {
    assert!(result.contains(&(from, Err::<MusicNftEvent, MusicNftError>(error).encode())));
}

pub fn get_init_nft_payload(
    collection_owner: ActorId,
    royalty: u16,
    user_mint_limit: Option<u32>,
    payment_for_mint: u128,
    permission_to_mint: Option<Vec<ActorId>>,
) -> NftInit {
    let img_data = ImageData {
        limit_copies: Some(1),
    };
    let img_links_and_data: Vec<(String, ImageData)> = (0..10)
        .map(|i| (format!("Img-{}", i), img_data.clone()))
        .collect();

    NftInit {
        collection_owner,
        config: Config {
            name: "User Collection".to_string(),
            description: "User Collection".to_string(),
            collection_banner: "Collection banner".to_string(),
            collection_logo: "Collection logo".to_string(),
            collection_tags: vec!["tag1".to_string()],
            additional_links: None,
            royalty,
            user_mint_limit,
            payment_for_mint,
            transferable: Some(0),
            sellable: Some(0),
            variable_meta: false,
        },
        img_links_and_data,
        permission_to_mint,
    }
}

pub fn get_init_music_nft_payload(
    collection_owner: ActorId,
    royalty: u16,
    user_mint_limit: Option<u32>,
    payment_for_mint: u128,
    permission_to_mint: Option<Vec<ActorId>>,
) -> MusicNftInit {
    let img_data = MusicImageData {
        limit_copies: Some(1),
        description: None,
    };
    let links_and_data: Vec<(Links, MusicImageData)> = (0..10)
        .map(|i| {
            let links = Links {
                img_link: Some(format!("Img-{}", i)),
                music_link: format!("Music link-{}", i),
            };
            (links, img_data.clone())
        })
        .collect();

    MusicNftInit {
        collection_owner,
        config: MusicConfig {
            name: "User Collection".to_string(),
            description: "User Collection".to_string(),
            collection_banner: "Collection banner".to_string(),
            collection_logo: "Collection logo".to_string(),
            collection_tags: vec!["tag1".to_string()],
            additional_links: None,
            royalty,
            user_mint_limit,
            payment_for_mint,
            transferable: Some(0),
            sellable: Some(0),
            listening_capabilities: ListenCapability::Demo,
        },
        links_and_data,
        permission_to_mint,
    }
}
// pub fn get_state(
//     marketplace: &Program,
//     admin: u64,
//     code_id: CodeId

// ) -> Option<VaraManState> {

//     let reply = self
//         .read_state(StateQuery::All)
//         .expect("Unexpected invalid state.");
//     if let StateReply::All(state) = reply {
//         Some(state)
//     } else {
//         None
//     }
// }
