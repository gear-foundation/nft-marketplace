import { AuctionCanceled } from '../../types/marketplace.events';
import { EntitiesStorage } from '../entities.storage';
import { INftMarketplaceEventHandler } from './nft-marketplace.handler';
import { Block } from '@subsquid/substrate-processor';

export class AuctionCanceledHandler implements INftMarketplaceEventHandler {
  async handle(
    block: Block,
    event: AuctionCanceled,
    storage: EntitiesStorage,
  ): Promise<void> {
    const { collectionAddress, tokenId } = event;
    const nft = await storage.getNft(collectionAddress, tokenId);
    if (nft === undefined) {
      console.warn(
        `[AuctionCanceledHandler] ${collectionAddress}-${tokenId}: nft is not found`,
      );
      return;
    }
    const sale = await storage.getSale(nft);
    if (sale === undefined) {
      console.warn(
        `[AuctionCanceledHandler] ${collectionAddress}-${tokenId}: sale is not found`,
      );
      return;
    }
    await storage.deleteSale(sale);
  }
}
