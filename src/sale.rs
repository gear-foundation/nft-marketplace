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
        let msg_src = msg::source();
        self.check_allow_message(&msg_src)?;
        // check that this collection already exists in the marketplace
        if !self.collection_to_owner.contains_key(&collection_address) {
            return Err(NftMarketplaceError::WrongCollectionAddress);
        }
        // check that this nft is not sale at this moment
        if self.sales.contains_key(&(collection_address, token_id)) {
            return Err(NftMarketplaceError::AlreadyOnSale);
        }
        // check if this nft is currently at auction
        if self.auctions.contains_key(&(collection_address, token_id)) {
            return Err(NftMarketplaceError::AlreadyOnAuction);
        }
        // check that the price is more than the minimum value for trade
        if price < self.minimum_value_for_trade {
            return Err(NftMarketplaceError::LessThanMinimumValueForTrade);
        }

        // send a message to the nft contract to find out information about the token
        let address_marketplace = exec::program_id();
        let (collection_owner, royalty) = check_token_info(
            &collection_address,
            token_id,
            self.config.gas_for_get_info,
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
            return Err(NftMarketplaceError::WrongReply);
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
        let msg_src = msg::source();
        self.check_allow_message(&msg_src)?;
        // check that such a sale exists and that the sender of the message is the owner of the sale/nft
        if let Some(nft_info) = self.sales.get(&(collection_address, token_id)) {
            if nft_info.token_owner == msg_src {
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
                    return Err(NftMarketplaceError::WrongReply);
                }
            } else {
                return Err(NftMarketplaceError::AccessDenied);
            }
        } else {
            return Err(NftMarketplaceError::SaleDoesNotExist);
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
        buyer: ActorId,
        msg_value: u128,
    ) -> Result<NftMarketplaceEvent, NftMarketplaceError> {
        self.check_allow_message(&buyer)?;
        self.check_sale(&collection_address, &token_id, msg_value)?;
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
            self.config.royalty_to_marketplace_for_trade,
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
        payment: u128,
    ) -> Result<(), NftMarketplaceError> {
        // check that such a sale exists and check the attached amount
        let nft = self
            .sales
            .get(&(*collection_address, *token_id))
            .ok_or(NftMarketplaceError::SaleDoesNotExist)?;
        if payment < nft.price {
            return Err(NftMarketplaceError::ValueIsLessThanPrice);
        }

        Ok(())
    }
}
