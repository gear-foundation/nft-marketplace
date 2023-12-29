import { Block } from '@subsquid/substrate-processor';
import { MintedEvent } from '../../types/nft.events';
import { EntitiesStorage } from '../entities.storage';
import { INftEventHandler } from './nft.handler';
import { Nft } from '../../model';

export class NftMintedHandler implements INftEventHandler {
  async handle(
    block: Block,
    collectionAddress: string,
    event: MintedEvent,
    storage: EntitiesStorage,
  ): Promise<void> {
    const collection = await storage.getCollection(collectionAddress);
    if (collection === undefined) {
      console.warn(
        `[NftMintedHandler] ${collectionAddress}: collection is not found`,
      );
      return;
    }
    const { tokenId } = event;
    let { description, mediaUrl, metadata, name, owner } = event.nftData;
    storage.setNft(
      new Nft({
        collection,
        description,
        idInCollection: tokenId,
        mediaUrl,
        name,
        owner,
        mintedAt: new Date(),
      }),
    );
  }
}
