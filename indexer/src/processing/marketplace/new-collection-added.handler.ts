import { NewCollectionAdded } from '../../types/marketplace.events';
import { EntitiesStorage } from '../entities.storage';
import { INftMarketplaceEventHandler } from './nft-marketplace.handler';
import { CollectionType } from '../../model';
import { Block } from '@subsquid/substrate-processor';

export class NewCollectionAddedHandler implements INftMarketplaceEventHandler {
  async handle(
    block: Block,
    event: NewCollectionAdded,
    storage: EntitiesStorage,
  ): Promise<void> {
    const { codeId, metaLink, typeName, typeDescription } = event;
    const existingCollectionType = await storage.getCollectionType(typeName);
    if (existingCollectionType !== undefined) {
      console.warn(
        `[NewCollectionAddedHandler]: Collection type ${typeName} already exists`,
      );
      return;
    }
    storage.setCollectionType(
      new CollectionType({
        id: codeId,
        description: typeDescription,
        type: typeName,
        metaUrl: metaLink,
        metaStr: '',
      }),
    );
  }
}
