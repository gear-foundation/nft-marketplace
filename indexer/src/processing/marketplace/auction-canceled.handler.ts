import { AuctionCanceled } from '../../types/marketplace.events';
import { EntitiesService } from '../entities.service';
import { INftMarketplaceEventHandler } from './nft-marketplace.handler';
import { AuctionStatus } from '../../model/types';
import { EventInfo } from '../event-info.type';

export class AuctionCanceledHandler implements INftMarketplaceEventHandler {
  async handle(
    event: AuctionCanceled,
    eventInfo: EventInfo,
    storage: EntitiesService,
  ): Promise<void> {
    const { collectionAddress, tokenId } = event;
    const nft = await storage.getNft(collectionAddress, tokenId);
    if (nft === undefined) {
      console.warn(
        `[AuctionCanceledHandler] ${collectionAddress}-${tokenId}: nft is not found`,
      );
      return;
    }
    const auction = await storage.getAuction(nft);
    if (auction === undefined) {
      console.warn(
        `[AuctionCanceledHandler] ${collectionAddress}-${tokenId}: auction is not found`,
      );
      return;
    }
    await storage.setAuction({
      ...auction,
      status: AuctionStatus.Canceled,
      updatedAt: eventInfo.timestamp,
      endTimestamp: eventInfo.timestamp,
    });
  }
}
