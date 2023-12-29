import { Block } from '@subsquid/substrate-processor';
import { InitializedEvent } from '../../types/nft.events';
import { EntitiesStorage } from '../entities.storage';
import { INftEventHandler } from './nft.handler';
import { Collection } from '../../model';

export class InitializedHandler implements INftEventHandler {
  async handle(
    block: Block,
    collectionAddress: string,
    event: InitializedEvent,
    storage: EntitiesStorage,
  ): Promise<void> {
    const collection = await storage.getCollection(collectionAddress);
    if (collection === undefined) {
      console.warn(
        `[InitializedHandler] ${collectionAddress}: collection is not found`,
      );
      return;
    }
    let {
      name,
      description,
      additionalLinks,
      approvable,
      attendable,
      burnable,
      collectionImg,
      collectionLogo,
      collectionTags,
      paymentForMint,
      royalty,
      sellable,
      transferable,
      userMintLimit,
    } = event.config;
    storage.setCollection({
      ...collection,
      name,
      description,
      additionalLinks,
      approvable,
      attendable,
      burnable,
      collectionImage: collectionImg,
      collectionLogo,
      tags: collectionTags,
      paymentForMint,
      royalty,
      sellable,
      transferable,
      tokensLimit: userMintLimit,
    } as Collection);
  }
}
