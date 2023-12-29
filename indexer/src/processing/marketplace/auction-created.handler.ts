import { AuctionCreated } from '../../types/marketplace.events';
import { EntitiesStorage } from '../entities.storage';
import { INftMarketplaceEventHandler } from './nft-marketplace.handler';
import { Sale } from '../../model';
import { Block } from '@subsquid/substrate-processor';

export class AuctionCreatedHandler implements INftMarketplaceEventHandler {
  async handle(
    block: Block,
    event: AuctionCreated,
    storage: EntitiesStorage,
  ): Promise<void> {
    const { collectionAddress, tokenId, price, currentOwner } = event;
    const collection = await storage.getCollection(collectionAddress);
    if (collection === undefined) {
      console.warn(
        `[AuctionCreatedHandler] ${collectionAddress}: collection is not found`,
      );
      return;
    }
    const nft = await storage.getNft(collectionAddress, tokenId);
    if (nft === undefined) {
      console.warn(
        `[AuctionCreatedHandler] ${collectionAddress}-${tokenId}: nft is not found`,
      );
      return;
    }
    const sale = await storage.getSale(nft);
    if (sale !== undefined) {
      console.warn(
        `[AuctionCreatedHandler] ${collectionAddress}-${tokenId}: sale is already exists`,
      );
      return;
    }
    storage.setSale(
      new Sale({
        nft,
        price,
        owner: currentOwner,
        isSold: false,
        blockNumber: block.header.hash,
      }),
    );
  }
}
