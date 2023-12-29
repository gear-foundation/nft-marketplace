import { NftSold } from '../../types/marketplace.events';
import { EntitiesStorage } from '../entities.storage';
import { INftMarketplaceEventHandler } from './nft-marketplace.handler';
import { Sale, Transfer } from '../../model';
import { Block } from '@subsquid/substrate-processor';

export class NftSoldHandler implements INftMarketplaceEventHandler {
  async handle(
    block: Block,
    event: NftSold,
    storage: EntitiesStorage,
  ): Promise<void> {
    const { collectionAddress, tokenId, price, currentOwner } = event;
    const collection = await storage.getCollection(collectionAddress);
    if (collection === undefined) {
      console.warn(
        `[NftSoldHandler] ${collectionAddress}: collection is not found`,
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
        isSold: true,
        blockNumber: block.header.hash,
      }),
    );
    storage.setTransfer(
      new Transfer({
        nft,
        from: nft.owner,
        to: currentOwner,
        blockNumber: block.header.hash,
      }),
    );
    storage.setNft({
      ...nft,
      owner: currentOwner,
    });
  }
}
