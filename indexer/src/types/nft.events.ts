import { Enum, Option, Text, u128, u16, u32, u64, Vec } from '@polkadot/types';
import { Hash } from '@polkadot/types/interfaces';

export enum NftEventType {
  Transferred = 'Transferred',
  TokenInfoReceived = 'TokenInfoReceived',
  Initialized = 'Initialized',
  Minted = 'Minted',
  Approved = 'Approved',
  ApprovalRevoked = 'ApprovalRevoked',
  Expanded = 'Expanded',
  ConfigChanged = 'ConfigChanged',
  ImageChanged = 'ImageChanged',
  MetadataAdded = 'MetadataAdded',
}

export type TransferEvent = {
  type: NftEventType.Transferred;
  owner: string;
  recipient: string;
  tokenId: number;
};

export type TokenInfoReceivedEvent = {
  type: NftEventType.TokenInfoReceived;
  tokenOwner: string;
  approval: string;
  sellable: boolean;
  collectionOwner: string;
  royalty: number;
};

export type NftConfig = {
  name: string;
  description: string;
  collectionTags: string[];
  collectionImg: string;
  collectionLogo: string;
  userMintLimit: number | null;
  additionalLinks: number | null;
  royalty: number;
  paymentForMint: number;
  transferable: boolean;
  approvable: boolean;
  burnable: boolean;
  sellable: boolean;
  attendable: boolean;
};

export type InitializedEvent = {
  type: NftEventType.Initialized;
  config: NftConfig;
};

export type Nft = {
  owner: string;
  name: string;
  description: string;
  metadata: string[];
  mediaUrl: string;
};

export type MintedEvent = {
  type: NftEventType.Minted;
  tokenId: number;
  nftData: Nft;
};

export type ApprovedEvent = {
  type: NftEventType.Approved;
  to: string;
  tokenId: number;
};

export type ApprovalRevokedEvent = {
  type: NftEventType.ApprovalRevoked;
  tokenId: number;
};

export type ExpandedEvent = {
  type: NftEventType.Expanded;
  additionalLinks: string[];
};

export type ConfigChangedEvent = {
  type: NftEventType.ConfigChanged;
  config: NftConfig;
};

export type ImageChangedEvent = {
  type: NftEventType.ImageChanged;
  tokenId: number;
  imgLink: string;
};

export type MetadataAddedEvent = {
  type: NftEventType.MetadataAdded;
  tokenId: number;
  metadata: string;
};

export type NftEvent =
  | TransferEvent
  | TokenInfoReceivedEvent
  | InitializedEvent
  | MintedEvent
  | ApprovedEvent
  | ApprovalRevokedEvent
  | ExpandedEvent
  | ConfigChangedEvent
  | ImageChangedEvent
  | MetadataAddedEvent;

export interface ConfigPlain {
  name: Text;
  description: Text;
  collectionTags: Vec<Text>;
  collectionImg: Text;
  collectionLogo: Text;
  userMintLimit: Option<u32>;
  additionalLinks: Option<u32>;
  royalty: u16;
  paymentForMint: u128;
  transferable: boolean;
  approvable: boolean;
  burnable: boolean;
  sellable: boolean;
  attendable: boolean;
}

export interface NftEventPlain extends Enum {
  transferred: {
    owner: Hash;
    recipient: Hash;
    token_id: u64;
  };
  tokenInfoReceived: {
    token_owner: Hash;
    approval: Option<Hash>;
    sellable: boolean;
    collection_owner: Hash;
    royalty: u16;
  };
  initialized: ConfigPlain;
  minted: {
    token_id: u64;
    nft_data: {
      owner: Hash;
      name: Text;
      description: Text;
      metadata: Vec<Text>;
      mediaUrl: Text;
    };
  };
  approved: {
    to: Hash;
    tokenId: u64;
  };
  approvalRevoked: {
    tokenId: u64;
  };
  expanded: {
    additional_links: Vec<Text>;
  };
  configChanged: {
    config: ConfigPlain;
  };
  imageChanged: {
    token_id: u64;
    imgLink: Text;
  };
  metadataAdded: {
    token_id: u64;
    metadata: Text;
  };
}

export function getNftEvent(event: NftEventPlain): NftEvent | undefined {
  if (event.transferred) {
    return {
      type: NftEventType.Transferred,
      owner: event.transferred.owner.toString(),
      recipient: event.transferred.recipient.toString(),
      tokenId: event.transferred.token_id.toNumber(),
    };
  }
  if (event.tokenInfoReceived) {
    return {
      type: NftEventType.TokenInfoReceived,
      tokenOwner: event.tokenInfoReceived.token_owner.toString(),
      approval: event.tokenInfoReceived.approval?.toString() ?? null,
      sellable: event.tokenInfoReceived.sellable,
      collectionOwner: event.tokenInfoReceived.collection_owner.toString(),
      royalty: event.tokenInfoReceived.royalty.toNumber(),
    };
  }
  if (event.initialized) {
    return {
      type: NftEventType.Initialized,
      config: {
        name: event.initialized.name.toString(),
        description: event.initialized.description.toString(),
        collectionTags: event.initialized.collectionTags.map((tag) =>
          tag.toString(),
        ),
        collectionImg: event.initialized.collectionImg.toString(),
        collectionLogo: event.initialized.collectionLogo.toString(),
        userMintLimit:
          event.initialized.userMintLimit.unwrapOr(null)?.toNumber() ?? null,
        additionalLinks:
          event.initialized.additionalLinks.unwrapOr(null)?.toNumber() ?? null,
        royalty: event.initialized.royalty.toNumber(),
        paymentForMint: event.initialized.paymentForMint.toNumber(),
        transferable: event.initialized.transferable,
        approvable: event.initialized.approvable,
        burnable: event.initialized.burnable,
        sellable: event.initialized.sellable,
        attendable: event.initialized.attendable,
      },
    };
  }
  if (event.minted) {
    return {
      type: NftEventType.Minted,
      tokenId: event.minted.token_id.toNumber(),
      nftData: {
        owner: event.minted.nft_data.owner.toString(),
        name: event.minted.nft_data.name.toString(),
        description: event.minted.nft_data.description.toString(),
        metadata: event.minted.nft_data.metadata.map((m) => m.toString()),
        mediaUrl: event.minted.nft_data.mediaUrl.toString(),
      },
    };
  }
  if (event.approved) {
    return {
      type: NftEventType.Approved,
      to: event.approved.to.toString(),
      tokenId: event.approved.tokenId.toNumber(),
    };
  }
  if (event.approvalRevoked) {
    return {
      type: NftEventType.ApprovalRevoked,
      tokenId: event.approvalRevoked.tokenId.toNumber(),
    };
  }
  if (event.expanded) {
    return {
      type: NftEventType.Expanded,
      additionalLinks: event.expanded.additional_links.map((link) =>
        link.toString(),
      ),
    };
  }
  if (event.configChanged) {
    return {
      type: NftEventType.ConfigChanged,
      config: {
        name: event.configChanged.config.name.toString(),
        description: event.configChanged.config.description.toString(),
        collectionTags: event.configChanged.config.collectionTags.map((tag) =>
          tag.toString(),
        ),
        collectionImg: event.configChanged.config.collectionImg.toString(),
        collectionLogo: event.configChanged.config.collectionLogo.toString(),
        userMintLimit:
          event.configChanged.config.userMintLimit.unwrapOr(null)?.toNumber() ??
          null,
        additionalLinks:
          event.configChanged.config.additionalLinks
            .unwrapOr(null)
            ?.toNumber() ?? null,
        royalty: event.configChanged.config.royalty.toNumber(),
        paymentForMint: event.configChanged.config.paymentForMint.toNumber(),
        transferable: event.configChanged.config.transferable,
        approvable: event.configChanged.config.approvable,
        burnable: event.configChanged.config.burnable,
        sellable: event.configChanged.config.sellable,
        attendable: event.configChanged.config.attendable,
      },
    };
  }
  if (event.imageChanged) {
    return {
      type: NftEventType.ImageChanged,
      tokenId: event.imageChanged.token_id.toNumber(),
      imgLink: event.imageChanged.imgLink.toString(),
    };
  }
  if (event.metadataAdded) {
    return {
      type: NftEventType.MetadataAdded,
      tokenId: event.metadataAdded.token_id.toNumber(),
      metadata: event.metadataAdded.metadata.toString(),
    };
  }
  return undefined;
}
