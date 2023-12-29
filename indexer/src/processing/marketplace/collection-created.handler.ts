import { CollectionCreated } from '../../types/marketplace.events';
import { EntitiesStorage } from '../entities.storage';
import { INftMarketplaceEventHandler } from './nft-marketplace.handler';
import { Collection } from '../../model';
import { Block } from '@subsquid/substrate-processor';

export class CollectionCreatedHandler implements INftMarketplaceEventHandler {
  async handle(
    block: Block,
    event: CollectionCreated,
    storage: EntitiesStorage,
  ): Promise<void> {
    const { typeName, collectionAddress } = event;
    const existingCollection = await storage.getCollection(collectionAddress);
    if (existingCollection !== undefined) {
      console.warn(
        `[CollectionCreatedHandler] ${collectionAddress}: collection already exists`,
      );
      return;
    }
    const collectionType = await storage.getCollectionType(typeName);
    if (collectionType === undefined) {
      console.warn(
        `[CollectionCreatedHandler] ${collectionAddress}: collection type ${typeName} does not exist`,
      );
      return;
    }
    storage.setCollection(
      new Collection({
        id: collectionAddress,
        type: collectionType,
      }),
    );
  }
}
