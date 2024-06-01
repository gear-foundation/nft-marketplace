use gcore::exec;
use gstd::{msg, ActorId};
use nft_marketplace_io::{NftMarketplaceError, NftMarketplaceEvent};

pub fn currency_transfer(
    collection_owner: ActorId,
    token_owner: ActorId,
    price: u128,
    royalty: u16,
    royalty_to_marketplace: u16,
) {
    // calculate the percentage to the creator of the collection
    // current_price * royalty / 10_000
    let percent_to_collection_creator = price * (royalty as u128) / 10_000u128;
    let percent_to_marketplace = price * (royalty_to_marketplace as u128) / 10_000u128;
    if percent_to_collection_creator > exec::env_vars().existential_deposit {
        // use send_with_gas to transfer the value directly to the balance, not to the mailbox.
        msg::send_with_gas(
            collection_owner,
            Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::ValueSent),
            0,
            percent_to_collection_creator,
        )
        .expect("Error in sending value");
        msg::send_with_gas(
            token_owner,
            Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::ValueSent),
            0,
            price - percent_to_collection_creator - percent_to_marketplace,
        )
        .expect("Error in sending value");
    } else {
        msg::send_with_gas(
            token_owner,
            Ok::<NftMarketplaceEvent, NftMarketplaceError>(NftMarketplaceEvent::ValueSent),
            0,
            price - percent_to_marketplace,
        )
        .expect("Error in sending value");
    }
}
