use gstd::{msg, prelude::*, ActorId};
use nft_marketplace_io::*;

pub async fn transfer_token(
    collection_address: ActorId,
    to: ActorId,
    token_id: u64,
    gas_for_transfer: u64,
) -> Result<NftEvent, NftMarketplaceError> {
    let transfer_payload = NftAction::Transfer { to, token_id };
    let reply = msg::send_with_gas_for_reply_as::<NftAction, Result<NftEvent, NftError>>(
        collection_address,
        transfer_payload,
        gas_for_transfer,
        0,
        0,
    )
    .expect("Error during send message `NftAction::Transfer`")
    .await
    .expect("Program was problem with transfer");

    check_reply(reply)
}

pub async fn transfer_from_token(
    collection_address: ActorId,
    from: ActorId,
    to: ActorId,
    token_id: u64,
    gas_for_transfer: u64,
) -> Result<NftEvent, NftMarketplaceError> {
    let transfer_payload = NftAction::TransferFrom { from, to, token_id };
    let reply = msg::send_with_gas_for_reply_as::<NftAction, Result<NftEvent, NftError>>(
        collection_address,
        transfer_payload,
        gas_for_transfer,
        0,
        0,
    )
    .expect("Error during `NftAction::TransferFrom`")
    .await
    .expect("Program was problem with transfer");

    check_reply(reply)
}

pub async fn check_token_info(
    collection_address: &ActorId,
    token_id: u64,
    gas_for_get_token_info: u64,
    msg_src: &ActorId,
    address_marketplace: &ActorId,
) -> Result<(ActorId, u16), NftMarketplaceError> {
    let get_token_info_payload = NftAction::GetTokenInfo { token_id };
    let reply = msg::send_with_gas_for_reply_as::<NftAction, Result<NftEvent, NftError>>(
        *collection_address,
        get_token_info_payload,
        gas_for_get_token_info,
        0,
        0,
    )
    .expect("Error during `NftAction::GetTokenInfo`")
    .await
    .expect("Problem with get token info");

    let reply = check_reply(reply)?;

    let (collection_owner, royalty) = if let NftEvent::TokenInfoReceived {
        token_owner,
        approval,
        sellable,
        collection_owner,
        royalty,
    } = reply
    {
        // nft should be sellable
        if !sellable {
            return Err(NftMarketplaceError("Nft is not sellable".to_owned()));
        }
        // the owner must be the same as the one who wants to sell
        if token_owner != *msg_src {
            return Err(NftMarketplaceError(
                "Only the owner of the token can perform this action.".to_owned(),
            ));
        }
        // must be approved by the marketplace
        if let Some(approve_acc) = approval {
            if approve_acc != *address_marketplace {
                return Err(NftMarketplaceError(
                    "No approve to the marketplace".to_owned(),
                ));
            }
        } else {
            return Err(NftMarketplaceError(
                "No approve to the marketplace".to_owned(),
            ));
        }
        (collection_owner, royalty)
    } else {
        return Err(NftMarketplaceError("Wrong received reply".to_owned()));
    };
    Ok((collection_owner, royalty))
}

fn check_reply(reply: Result<NftEvent, NftError>) -> Result<NftEvent, NftMarketplaceError> {
    match reply {
        Ok(result) => Ok(result),
        Err(NftError(error_string)) => Err(NftMarketplaceError(error_string.clone())),
    }
}
