import { EntitiesStorage } from '../entities.storage';
import { Block } from '@subsquid/substrate-processor';
import { NftEvent } from '../../types/nft.events';

export interface INftEventHandler {
  handle(
    block: Block,
    collectionAddress: string,
    event: NftEvent,
    storage: EntitiesStorage,
  ): Promise<void>;
}
