import { Enum, Option, Text, Vec, u64, u128 } from '@polkadot/types';
import { Hash } from '@polkadot/types/interfaces';
import { CodeId } from '@gear-js/api';

export enum NftMarketplaceEventType {
  NewCollectionAdded = 'isNewCollectionAdded',
  CollectionCreated = 'isCollectionCreated',
  SaleNft = 'isSaleNft',
  SaleNftCanceled = 'isSaleNftCanceled',
  NftSold = 'isNftSold',
  AuctionCreated = 'isAuctionCreated',
  BidAdded = 'isBidAdded',
  AuctionCanceled = 'isAuctionCanceled',
  OfferCreated = 'isOfferCreated',
  OfferCanceled = 'isOfferCanceled',
  OfferAccepted = 'isOfferAccepted',
  CollectionDeleted = 'isCollectionDeleted',
  AdminsAdded = 'isAdminsAdded',
  AdminDeleted = 'isAdminDeleted',
  ConfigUpdated = 'isConfigUpdated',
}

export type NewCollectionAdded = {
  type: NftMarketplaceEventType.NewCollectionAdded;
  codeId: string;
  metaLink: string;
  typeName: string;
  typeDescription: string;
};

export type CollectionCreated = {
  type: NftMarketplaceEventType.CollectionCreated;
  typeName: string;
  collectionAddress: string;
};

export type SaleNft = {
  type: NftMarketplaceEventType.SaleNft;
  collectionAddress: string;
  tokenId: number;
  price: bigint;
  owner: string;
};

export type SaleNftCanceled = {
  type: NftMarketplaceEventType.SaleNftCanceled;
  collectionAddress: string;
  tokenId: number;
};

export type NftSold = {
  type: NftMarketplaceEventType.NftSold;
  currentOwner: string;
  tokenId: number;
  price: bigint;
  collectionAddress: string;
};

export type AuctionCreated = {
  type: NftMarketplaceEventType.AuctionCreated;
  collectionAddress: string;
  tokenId: number;
  price: bigint;
  currentOwner: string;
};

export type BidAdded = {
  type: NftMarketplaceEventType.BidAdded;
  collectionAddress: string;
  tokenId: number;
};

export type AuctionCanceled = {
  type: NftMarketplaceEventType.AuctionCanceled;
  collectionAddress: string;
  tokenId: number;
};

export type OfferCreated = {
  type: NftMarketplaceEventType.OfferCreated;
  collectionAddress: string;
  tokenId: number;
  price: bigint;
};

export type OfferCanceled = {
  type: NftMarketplaceEventType.OfferCanceled;
  collectionAddress: string;
  tokenId: number;
};

export type OfferAccepted = {
  type: NftMarketplaceEventType.OfferAccepted;
};

export type CollectionDeleted = {
  type: NftMarketplaceEventType.CollectionDeleted;
  collectionAddress: string;
};

export type AdminsAdded = {
  type: NftMarketplaceEventType.AdminsAdded;
  users: string[];
};

export type AdminDeleted = {
  type: NftMarketplaceEventType.AdminDeleted;
  user: string;
};

export type ConfigUpdated = {
  type: NftMarketplaceEventType.ConfigUpdated;
  gasForCreation: number | null;
  timeBetweenCreateCollections: number | null;
};

export type NftMarketplaceEvent =
  | NewCollectionAdded
  | CollectionCreated
  | SaleNft
  | SaleNftCanceled
  | NftSold
  | AuctionCreated
  | BidAdded
  | AuctionCanceled
  | OfferCreated
  | OfferCanceled
  | OfferAccepted
  | CollectionDeleted
  | AdminsAdded
  | AdminDeleted
  | ConfigUpdated;

export interface NftMarketplaceEventPlain extends Enum {
  newCollectionAdded: {
    codeId: CodeId;
    metaLink: Text;
    typeName: Text;
    typeDescription: Text;
  };
  collectionCreated: {
    typeName: Text;
    collectionAddress: Hash;
  };
  saleNft: {
    collectionAddress: Hash;
    tokenId: u64;
    price: u128;
    owner: Hash;
  };
  saleNftCanceled: {
    collectionAddress: Hash;
    tokenId: u64;
  };
  nftSold: {
    currentOwner: Hash;
    tokenId: u64;
    price: u128;
    collectionAddress: Hash;
  };
  auctionCreated: {
    collectionAddress: Hash;
    tokenId: u64;
    price: u128;
    currentOwner: Hash;
  };
  bidAdded: {
    collectionAddress: Hash;
    tokenId: u64;
  };
  auctionCanceled: {
    collectionAddress: Hash;
    tokenId: u64;
  };
  offerCreated: {
    collectionAddress: Hash;
    tokenId: u64;
    price: u128;
  };
  offerCanceled: {
    collectionAddress: Hash;
    tokenId: u64;
  };
  offerAccepted: {};
  collectionDeleted: {
    collectionAddress: Hash;
  };
  adminsAdded: {
    users: Vec<Hash>;
  };
  adminDeleted: {
    user: Hash;
  };
  configUpdated: {
    gasForCreation: Option<u64>;
    timeBetweenCreateCollections: Option<u64>;
  };
}

export function getMarketplaceEvent(
  event: NftMarketplaceEventPlain,
): NftMarketplaceEvent | undefined {
  if (event.newCollectionAdded) {
    return {
      type: NftMarketplaceEventType.NewCollectionAdded,
      codeId: event.newCollectionAdded.codeId.toString(),
      metaLink: event.newCollectionAdded.metaLink.toString(),
      typeName: event.newCollectionAdded.typeName.toString(),
      typeDescription: event.newCollectionAdded.typeDescription.toString(),
    };
  }
  if (event.collectionCreated) {
    return {
      type: NftMarketplaceEventType.CollectionCreated,
      typeName: event.collectionCreated.typeName.toString(),
      collectionAddress: event.collectionCreated.collectionAddress.toString(),
    };
  }
  if (event.saleNft) {
    return {
      type: NftMarketplaceEventType.SaleNft,
      collectionAddress: event.saleNft.collectionAddress.toString(),
      tokenId: event.saleNft.tokenId.toNumber(),
      price: event.saleNft.price.toBigInt(),
      owner: event.saleNft.owner.toString(),
    };
  }
  if (event.saleNftCanceled) {
    return {
      type: NftMarketplaceEventType.SaleNftCanceled,
      collectionAddress: event.saleNftCanceled.collectionAddress.toString(),
      tokenId: event.saleNftCanceled.tokenId.toNumber(),
    };
  }
  if (event.nftSold) {
    return {
      type: NftMarketplaceEventType.NftSold,
      currentOwner: event.nftSold.currentOwner.toString(),
      tokenId: event.nftSold.tokenId.toNumber(),
      price: event.nftSold.price.toBigInt(),
      collectionAddress: event.nftSold.collectionAddress.toString(),
    };
  }
  if (event.auctionCreated) {
    return {
      type: NftMarketplaceEventType.AuctionCreated,
      collectionAddress: event.auctionCreated.collectionAddress.toString(),
      tokenId: event.auctionCreated.tokenId.toNumber(),
      price: event.auctionCreated.price.toBigInt(),
      currentOwner: event.auctionCreated.currentOwner.toString(),
    };
  }
  if (event.bidAdded) {
    return {
      type: NftMarketplaceEventType.BidAdded,
      collectionAddress: event.bidAdded.collectionAddress.toString(),
      tokenId: event.bidAdded.tokenId.toNumber(),
    };
  }
  if (event.auctionCanceled) {
    return {
      type: NftMarketplaceEventType.AuctionCanceled,
      collectionAddress: event.auctionCanceled.collectionAddress.toString(),
      tokenId: event.auctionCanceled.tokenId.toNumber(),
    };
  }
  if (event.offerCreated) {
    return {
      type: NftMarketplaceEventType.OfferCreated,
      collectionAddress: event.offerCreated.collectionAddress.toString(),
      tokenId: event.offerCreated.tokenId.toNumber(),
      price: event.offerCreated.price.toBigInt(),
    };
  }
  if (event.offerCanceled) {
    return {
      type: NftMarketplaceEventType.OfferCanceled,
      collectionAddress: event.offerCanceled.collectionAddress.toString(),
      tokenId: event.offerCanceled.tokenId.toNumber(),
    };
  }
  if (event.offerAccepted) {
    return {
      type: NftMarketplaceEventType.OfferAccepted,
    };
  }
  if (event.collectionDeleted) {
    return {
      type: NftMarketplaceEventType.CollectionDeleted,
      collectionAddress: event.collectionDeleted.collectionAddress.toString(),
    };
  }
  if (event.adminsAdded) {
    return {
      type: NftMarketplaceEventType.AdminsAdded,
      users: event.adminsAdded.users.map((user) => user.toString()),
    };
  }
  if (event.adminDeleted) {
    return {
      type: NftMarketplaceEventType.AdminDeleted,
      user: event.adminDeleted.user.toString(),
    };
  }
  if (event.configUpdated) {
    return {
      type: NftMarketplaceEventType.ConfigUpdated,
      gasForCreation:
        event.configUpdated.gasForCreation.unwrapOr(null)?.toNumber() ?? null,
      timeBetweenCreateCollections:
        event.configUpdated.timeBetweenCreateCollections
          .unwrapOr(null)
          ?.toNumber() ?? null,
    };
  }
}
