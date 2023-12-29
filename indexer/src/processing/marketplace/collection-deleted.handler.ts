import { CollectionDeleted } from '../../types/marketplace.events';
import { EntitiesStorage } from '../entities.storage';
import { INftMarketplaceEventHandler } from './nft-marketplace.handler';
import { Block } from '@subsquid/substrate-processor';

export class CollectionDeletedHandler implements INftMarketplaceEventHandler {
  async handle(
    block: Block,
    event: CollectionDeleted,
    storage: EntitiesStorage,
  ): Promise<void> {
    const { collectionAddress } = event;
    const existingCollection = await storage.getCollection(collectionAddress);
    if (existingCollection === undefined) {
      console.warn(
        `[CollectionDeletedHandler] ${collectionAddress}: collection not exists`,
      );
      return;
    }
    await storage.deleteCollection(existingCollection);
  }
}
