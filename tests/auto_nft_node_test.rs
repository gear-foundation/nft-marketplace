// use auto_changed_nft_io::{
//     Action, AutoNftAction, AutoNftInit, AutoNftState, Config, StateQuery, StateReply,
// };
// use gclient::{EventProcessor, GearApi, Result};
// use gear_core::ids::ProgramId;
// use gstd::Encode;

// #[tokio::test]
// #[ignore]
// async fn create_test() -> Result<()> {
//     let api = GearApi::dev_from_path("target/tmp/gear").await?;

//     let mut listener = api.subscribe().await?; // Subscribing for events.

//     // Checking that blocks still running.
//     assert!(listener.blocks_running().await?);

//     let links: Vec<String> = (0..4).map(|i| format!("Img-{}", i)).collect();
//     let img_links: Vec<(Vec<String>, u32)> = (0..10).map(|_| (links.clone(), 1 as u32)).collect();

//     // let time_for_change = 5;
//     let time_to_action = vec![(15, Action::ChangeImg), (30, Action::ChangeImg)];

//     let init_nft_payload = AutoNftInit {
//         owner: 100.into(),
//         config: Config {
//             name: "User Collection".to_string(),
//             description: "User Collection".to_string(),
//             collection_banner: "Collection banner".to_string(),
//             collection_tags: vec!["tag1".to_string()],
//             user_mint_limit: 3.into(),
//             // time_for_change,
//             time_to_action,
//             transferable: true,
//             approvable: true,
//             burnable: true,
//             sellable: true,
//             attendable: true,
//         },
//         img_links,
//     }
//     .encode();

//     println!("!!!!!!");

//     let path = "target/wasm32-unknown-unknown/debug/auto_changed_nft.opt.wasm";

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

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, AutoNftAction::Mint.encode(), 0, true)
//         .await?;

//     println!("GAS_INFO MIN LIMIT: {:?}", gas_info.min_limit.clone());
//     let (message_id, _) = api
//         .send_message(program_id, AutoNftAction::Mint, 2 * gas_info.min_limit, 0)
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     let interval_sec: u64 = 15;

//     let state = get_all_state_nft(&api, &program_id)
//         .await
//         .expect("Unexpected invalid state.");

//     // println!("STATE: {:?}", state);
//     assert_eq!(state.tokens.get(0).unwrap().1.media_url.0, 0);
//     std::thread::sleep(std::time::Duration::from_secs(interval_sec.into()));
//     let state = get_all_state_nft(&api, &program_id)
//         .await
//         .expect("Unexpected invalid state.");

//     // println!("STATE: {:?}", state);
//     assert_eq!(state.tokens.get(0).unwrap().1.media_url.0, 1);

//     std::thread::sleep(std::time::Duration::from_secs(interval_sec.into()));
//     let state = get_all_state_nft(&api, &program_id)
//         .await
//         .expect("Unexpected invalid state.");

//     // println!("STATE: {:?}", state);
//     assert_eq!(state.tokens.get(0).unwrap().1.media_url.0, 2);

//     Ok(())
// }

// pub async fn get_all_state_nft(api: &GearApi, program_id: &ProgramId) -> Option<AutoNftState> {
//     let reply = api
//         .read_state(*program_id, StateQuery::All.encode())
//         .await
//         .expect("Unexpected invalid reply.");
//     if let StateReply::All(state) = reply {
//         Some(state)
//     } else {
//         None
//     }
// }
