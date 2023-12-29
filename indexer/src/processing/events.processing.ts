import { readFileSync } from 'fs';
import { ProgramMetadata } from '@gear-js/api';
import {
  getMarketplaceEvent,
  NftMarketplaceEventPlain,
  NftMarketplaceEventType,
} from '../types/marketplace.events';
import { getNftEvent, NftEventPlain, NftEventType } from '../types/nft.events';
import { EntitiesStorage } from './entities.storage';
import { INftMarketplaceEventHandler } from './marketplace/nft-marketplace.handler';
import { NewCollectionAddedHandler } from './marketplace/new-collection-added.handler';
import { Store } from '@subsquid/typeorm-store';
import { Block } from '@subsquid/substrate-processor';
import { AuctionCreatedHandler } from './marketplace/auction-created.handler';
import { AuctionCanceledHandler } from './marketplace/auction-canceled.handler';
import { CollectionDeletedHandler } from './marketplace/collection-deleted.handler';
import { NftSoldHandler } from './marketplace/nft-sold.handler';
import { InitializedHandler } from './nft/initialized.handler';
import { INftEventHandler } from './nft/nft.handler';
import { NftMintedHandler } from './nft/nft-minted.handler';

const marketplaceMeta = ProgramMetadata.from(
  readFileSync('./assets/nft_marketplace.meta.txt', 'utf8'),
);
const nftMeta = ProgramMetadata.from(
  readFileSync('./assets/nft.meta.txt', 'utf8'),
);

const marketplaceEventsToHandler: Record<
  NftMarketplaceEventType,
  INftMarketplaceEventHandler | undefined
> = {
  [NftMarketplaceEventType.NewCollectionAdded]: new NewCollectionAddedHandler(),
  [NftMarketplaceEventType.CollectionCreated]: new NewCollectionAddedHandler(),
  [NftMarketplaceEventType.AuctionCreated]: new AuctionCreatedHandler(),
  [NftMarketplaceEventType.AuctionCanceled]: new AuctionCanceledHandler(),
  [NftMarketplaceEventType.CollectionDeleted]: new CollectionDeletedHandler(),
  [NftMarketplaceEventType.NftSold]: new NftSoldHandler(),
  // TODO: handle marketplace and admin events
  [NftMarketplaceEventType.AdminDeleted]: undefined,
  [NftMarketplaceEventType.AdminsAdded]: undefined,
  [NftMarketplaceEventType.ConfigUpdated]: undefined,
  [NftMarketplaceEventType.OfferCreated]: undefined,
  [NftMarketplaceEventType.OfferCanceled]: undefined,
  [NftMarketplaceEventType.OfferAccepted]: undefined,
  [NftMarketplaceEventType.BidAdded]: undefined,
  [NftMarketplaceEventType.SaleNft]: undefined,
  [NftMarketplaceEventType.SaleNftCanceled]: undefined,
};

const nftEventsToHandler: Record<NftEventType, INftEventHandler | undefined> = {
  [NftEventType.Initialized]: new InitializedHandler(),
  [NftEventType.Minted]: new NftMintedHandler(),
  [NftEventType.Approved]: undefined,
  [NftEventType.ApprovalRevoked]: undefined,
  [NftEventType.Expanded]: undefined,
  [NftEventType.ConfigChanged]: undefined,
  [NftEventType.ImageChanged]: undefined,
  [NftEventType.MetadataAdded]: undefined,
  [NftEventType.TokenInfoReceived]: undefined,
  [NftEventType.Transferred]: undefined,
};

export class EventsProcessing {
  private readonly entitiesStorage: EntitiesStorage;

  constructor(private readonly store: Store) {
    this.entitiesStorage = new EntitiesStorage(store);
  }

  saveAll() {
    return this.entitiesStorage.saveAll();
  }

  async handleMarketplaceEvent(block: Block, payload: string, source: string) {
    try {
      console.log(`${block.header.hash}: handling marketplace event`);
      const data = marketplaceMeta.createType<NftMarketplaceEventPlain>(
        marketplaceMeta.types.handle.output!,
        payload,
      );
      const parsed = data.toJSON() as { ok: NftMarketplaceEventPlain } | null;
      if (!parsed || !parsed.ok) {
        return;
      }
      const event = getMarketplaceEvent(parsed.ok);
      if (!event) {
        console.warn(`${block.header.hash}: unknown event type`, parsed);
        return;
      }
      const eventHandler = marketplaceEventsToHandler[event.type];
      if (!eventHandler) {
        console.warn(
          `${block.header.hash}: no event handlers found for ${event.type}`,
        );
        return;
      }
      await eventHandler.handle(block, event, this.entitiesStorage);
    } catch (e) {
      console.error(
        `${block.header.hash}: error handling marketplace event`,
        e,
      );
    }
  }

  async handleNftEvent(block: Block, payload: string, source: string) {
    try {
      console.log(`${block.header.hash}: handling nft event`);
      const data = marketplaceMeta.createType<NftEventPlain>(
        nftMeta.types.handle.output!,
        payload,
      );
      const parsed = data.toJSON() as { ok: NftEventPlain } | null;
      if (!parsed || !parsed.ok) {
        return;
      }
      const event = getNftEvent(data);
      if (!event) {
        console.warn(`${block.header.hash}: unknown event type`, parsed);
        return;
      }
      const eventHandler = nftEventsToHandler[event.type];
      if (!eventHandler) {
        console.warn(
          `${block.header.hash}: no nft event handlers found for ${event.type}`,
        );
        return;
      }
      await eventHandler.handle(block, source, event, this.entitiesStorage);
    } catch (e) {
      console.error(`${block.header.hash}: error handling nft event`, e);
    }
  }
}
