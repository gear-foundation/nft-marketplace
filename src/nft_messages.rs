use gstd::{msg, prelude::*, ActorId};
use nft_marketplace_io::*;

pub async fn mint(
    collection_address: ActorId,
    minter: ActorId,
    msg_value: u128,
    gas_for_mint: u64,
    gas_for_get_info: u64,
    royalty_to_marketplace: u16,
) -> Result<NftMarketplaceEvent, NftMarketplaceError> {

    let get_payment_for_mint_payload = NftAction::GetPaymentForMint;
    let reply = msg::send_with_gas_for_reply_as::<NftAction, Result<NftEvent, NftError>>(
        collection_address,
        get_payment_for_mint_payload,
        gas_for_get_info,
        0,
        0,
    )
    .expect("Error during send message `NftAction::GetPaymentForMint`")
    .await
    .expect("Program was problem with GetPaymentForMint");

    let reply = check_reply(reply)?;

    let payment_for_mint = match reply {
        NftEvent::PaymentForMintReceived { payment_for_mint } => payment_for_mint,
        _ => return Err(NftMarketplaceError::WrongReply),
    };

    let percent_to_marketplace = payment_for_mint * (royalty_to_marketplace as u128) / 10_000u128;
    if percent_to_marketplace + payment_for_mint != msg_value {
        return Err(NftMarketplaceError::WrongValue);
    }
    let mint_payload = NftAction::Mint { minter };

    let reply = msg::send_with_gas_for_reply_as::<NftAction, Result<NftEvent, NftError>>(
        collection_address,
        mint_payload,
        gas_for_mint,
        msg_value - percent_to_marketplace,
        0,
    )
    .expect("Error during send message `NftAction::Mint`")
    .await
    .expect("Program was problem with mint");

    check_reply(reply)?;

    Ok(NftMarketplaceEvent::Minted {
        collection_address,
        minter,
    })
}

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
    gas_for_get_info: u64,
    msg_src: &ActorId,
    address_marketplace: &ActorId,
) -> Result<(ActorId, u16), NftMarketplaceError> {
    let get_token_info_payload = NftAction::GetTokenInfo { token_id };
    let reply = msg::send_with_gas_for_reply_as::<NftAction, Result<NftEvent, NftError>>(
        *collection_address,
        get_token_info_payload,
        gas_for_get_info,
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
            return Err(NftMarketplaceError::NotSellable);
        }
        // the owner must be the same as the one who wants to sell
        if token_owner != *msg_src {
            return Err(NftMarketplaceError::AccessDenied);
        }
        // must be approved by the marketplace
        let approve_acc = approval.ok_or(NftMarketplaceError::NoApproveToMarketplace)?;
        if approve_acc != *address_marketplace {
            return Err(NftMarketplaceError::NoApproveToMarketplace);
        }
        (collection_owner, royalty)
    } else {
        return Err(NftMarketplaceError::WrongReply);
    };
    Ok((collection_owner, royalty))
}

fn check_reply(reply: Result<NftEvent, NftError>) -> Result<NftEvent, NftMarketplaceError> {
    match reply {
        Ok(result) => Ok(result),
        Err(_) => Err(NftMarketplaceError::ErrorFromCollection),
    }
}
