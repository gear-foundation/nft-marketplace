use crate::nft_messages::*;
use crate::payment::*;
use crate::NftMarketplace;
use gstd::{exec, msg, prelude::*, ActorId};
use nft_marketplace_io::*;

impl NftMarketplace {
    pub async fn sell(
        &mut self,
        collection_address: ActorId,
        token_id: u64,
        price: u128,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        // check that this collection already exists in the marketplace
        if !self.collection_to_owner.contains_key(&collection_address) {
            return Err(NftMarketplaceError(
                "This collection address is not in the marketplace".to_owned(),
            ));
        }
        // check that this nft is not sale at this moment
        if self.sales.contains_key(&(collection_address, token_id)) {
            return Err(NftMarketplaceError(
                "This nft is already on sale.".to_owned(),
            ));
        }
        // check if this nft is currently at auction
        if self.auctions.contains_key(&(collection_address, token_id)) {
            return Err(NftMarketplaceError(
                "This token is on auction, cancel the auction if you wish to sale the nft"
                    .to_owned(),
            ));
        }
        // check that the price is more than the minimum transfer value
        if price < self.config.minimum_transfer_value {
            return Err(NftMarketplaceError(format!(
                "The price must be greater than existential deposit ({})",
                self.config.minimum_transfer_value
            )));
        }

        // send a message to the nft contract to find out information about the token
        let address_marketplace = exec::program_id();
        let msg_src = msg::source();
        let (collection_owner, royalty) = check_token_info(
            &collection_address,
            token_id,
            self.config.gas_for_get_token_info,
            &msg_src,
            &address_marketplace,
        )
        .await?;

        // send a message to the nft contract to transfer the token to the marketplace address 
        // so that the token can be immediately transferred to the buyer upon purchase.
        if let NftEvent::Transferred {
            owner,
            recipient: _,
            token_id,
        } = transfer_from_token(
            collection_address,
            msg_src,
            address_marketplace,
            token_id,
            self.config.gas_for_transfer_token,
        )
        .await?
        {
            // if the transfer was successful, add information about the sale to the contract
            self.sales.insert(
                (collection_address, token_id),
                NftInfoForSale {
                    price,
                    token_owner: owner,
                    collection_owner,
                    royalty,
                },
            );
        } else {
            return Err(NftMarketplaceError("Wrong received reply".to_owned()));
        }

        Ok(NftMarketplaceEvent::SaleNft {
            collection_address,
            token_id,
            price,
            owner: msg_src,
        })
    }
    pub async fn cancel_sale(
        &mut self,
        collection_address: ActorId,
        token_id: u64,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        // check that such a sale exists and that the sender of the message is the owner of the sale/nft
        if let Some(nft_info) = self.sales.get(&(collection_address, token_id)) {
            if nft_info.token_owner == msg::source() {
                // return the token to its owner
                if let NftEvent::Transferred {
                    owner: _,
                    recipient: _,
                    token_id,
                } = transfer_token(
                    collection_address,
                    nft_info.token_owner,
                    token_id,
                    self.config.gas_for_transfer_token,
                )
                .await?
                {
                    // in case of successful token transfer, remove the sale from the marketplace
                    self.sales.remove(&(collection_address, token_id));
                } else {
                    return Err(NftMarketplaceError("Wrong received reply".to_owned()));
                }
            } else {
                return Err(NftMarketplaceError(
                    "Only the nft owner can cancel the sale.".to_owned(),
                ));
            }
        } else {
            return Err(NftMarketplaceError("This sale does not exist".to_owned()));
        }

        Ok(NftMarketplaceEvent::SaleNftCanceled {
            collection_address,
            token_id,
        })
    }

    pub async fn buy(
        &mut self,
        collection_address: ActorId,
        token_id: u64,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        let buyer = msg::source();
        self.check_sale(&collection_address, &token_id, &buyer)?;

        // transfer the token to the buyer
        transfer_token(
            collection_address,
            buyer,
            token_id,
            self.config.gas_for_transfer_token,
        )
        .await?;

        let nft = self
            .sales
            .get(&(collection_address, token_id))
            .expect("Can't be None")
            .clone();

        // transfer value to owner of token and percent to collection creator
        currency_transfer(
            nft.collection_owner,
            nft.token_owner,
            nft.price,
            nft.royalty,
            self.config.minimum_transfer_value,
        );
        // remove the sale from the marketplace
        self.sales
            .remove(&(collection_address, token_id))
            .expect("Can't be None");
        Ok(NftMarketplaceEvent::NftSold {
            collection_address,
            token_id,
            price: nft.price,
            current_owner: buyer,
        })
    }

    fn check_sale(
        &self,
        collection_address: &ActorId,
        token_id: &u64,
        buyer: &ActorId,
    ) -> Result<(), NftMarketplaceError> {
        let payment = msg::value();
        let nft = self.sales.get(&(*collection_address, *token_id));
        // check that such a sale exists and check the attached amount
        if let Some(nft) = nft {
            if payment < nft.price {
                msg::send(*buyer, "", payment).expect("Error in sending value");
                return Err(NftMarketplaceError(
                    "The specified value is less than the price of the token".to_owned(),
                ));
            }
        } else {
            return Err(NftMarketplaceError("This sale does not exist".to_owned()));
        }
        Ok(())
    }
}
