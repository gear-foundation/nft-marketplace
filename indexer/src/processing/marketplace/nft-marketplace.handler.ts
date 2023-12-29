import { NftMarketplaceEvent } from '../../types/marketplace.events';
import { EntitiesStorage } from '../entities.storage';
import { Block } from '@subsquid/substrate-processor';

export interface INftMarketplaceEventHandler {
  handle(
    block: Block,
    event: NftMarketplaceEvent,
    storage: EntitiesStorage,
  ): Promise<void>;
}
