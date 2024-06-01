// use composable_nft_io::{ComposableNftInit, Config as ComposableConfig};
// use gclient::{EventProcessor, GearApi, Result};
// use gstd::Encode;

// #[tokio::test]
// #[ignore]
// async fn create_test() -> Result<()> {
//     let api = GearApi::dev_from_path("target/tmp/gear").await?;

//     let mut listener = api.subscribe().await?; // Subscribing for events.

//     // Checking that blocks still running.
//     assert!(listener.blocks_running().await?);

//     let mut img_links: Vec<Vec<String>> = Vec::with_capacity(4);

//     for k in 0..10 {
//         let inner_vec: Vec<String> = (0..10).map(|i| format!("Value{}-{}", k, i)).collect();

//         img_links.push(inner_vec);
//     }

//     let init_nft_payload = ComposableNftInit {
//         owner: 100.into(),
//         config: ComposableConfig {
//             name: "User Collection".to_string(),
//             description: "User Collection".to_string(),
//             collection_banner: "Collection banner".to_string(),
//             collection_logo: "Collection logo".to_string(),
//             collection_tags: vec!["tag1".to_string()],
//             additional_links: None,
//             royalty: 0,
//             payment_for_mint: 0,
//             user_mint_limit: 3.into(),
//             tokens_limit: Some(500),
//             transferable: Some(0),
//             sellable: Some(0),
//         },
//         img_links,
//     }
//     .encode();

//     println!("!!!!!!");

//     let path = "target/wasm32-unknown-unknown/debug/composable_nft.opt.wasm";

//     let gas_info = api
//         .calculate_upload_gas(
//             None,
//             gclient::code_from_os(path)?,
//             init_nft_payload.clone(),
//             0,
//             true,
//         )
//         .await?;

//     println!("GAS_INFO MIN LIMIT: {:?}", gas_info.min_limit.clone());

//     let (message_id, program_id, _hash) = api
//         .upload_program_bytes(
//             gclient::code_from_os(path)?,
//             gclient::now_micros().to_le_bytes(),
//             init_nft_payload,
//             gas_info.min_limit,
//             0,
//         )
//         .await?;

//     println!("!!!!!!");
//     assert!(listener.message_processed(message_id).await?.succeed());
//     assert!(listener.blocks_running().await?);

//     Ok(())
// }
