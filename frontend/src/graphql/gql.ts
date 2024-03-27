/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n  query MarketplaceQuery {\n    marketplaceById(id: \"1\") {\n      address\n      collectionTypes {\n        description\n        metaUrl\n        type\n      }\n      metadata\n      config {\n        feePerUploadedFile\n        minimumValueForTrade\n        royaltyToMarketplaceForMint\n        royaltyToMarketplaceForTrade\n        timeBetweenCreateCollections\n      }\n      admins\n    }\n  }\n": types.MarketplaceQueryDocument,
    "\n  query CollectionSub($id: String!) {\n    collectionById(id: $id) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n      permissionToMint\n      userMintLimit\n      paymentForMint\n      transferable\n      sellable\n\n      additionalLinks {\n        discord\n        externalUrl\n        medium\n        xcom\n        telegram\n      }\n    }\n  }\n": types.CollectionSubDocument,
    "\n  query LastCreatedCollectionQuery($admin: String!) {\n    collections(where: { admin_eq: $admin }, orderBy: [createdAt_DESC, name_DESC, id_DESC], limit: 1) {\n      createdAt\n    }\n  }\n": types.LastCreatedCollectionQueryDocument,
    "\n  query CollectionsConnectionQuery($where: CollectionWhereInput!) {\n    collectionsConnection(orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      totalCount\n    }\n  }\n": types.CollectionsConnectionQueryDocument,
    "\n  query CollectionsQuery($limit: Int!, $offset: Int!, $where: CollectionWhereInput!) {\n    collections(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n\n      nfts(limit: 5) {\n        id\n        mediaUrl\n      }\n    }\n  }\n": types.CollectionsQueryDocument,
    "\n  query CollectionsSub($limit: Int!, $offset: Int!, $where: CollectionWhereInput!) {\n    collections(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n\n      nfts(limit: 5) {\n        id\n        mediaUrl\n      }\n    }\n  }\n": types.CollectionsSubDocument,
    "\n  query CollectionsNFTsCountQuery($ids: [String!]) {\n    nftsInCollection(collections: $ids) {\n      collection\n      count\n    }\n  }\n": types.CollectionsNfTsCountQueryDocument,
    "\n  query NFTsConnectionQuery($where: NftWhereInput!) {\n    nftsConnection(orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      totalCount\n    }\n  }\n": types.NfTsConnectionQueryDocument,
    "\n  query NFTsQuery($limit: Int!, $offset: Int!, $where: NftWhereInput!) {\n    nfts(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      idInCollection\n      name\n      mediaUrl\n      owner\n\n      mintedBy\n\n      collection {\n        id\n        name\n        transferable\n        sellable\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n": types.NfTsQueryDocument,
    "\n  query NFTsSubscription($limit: Int!, $offset: Int!, $where: NftWhereInput!) {\n    nfts(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      idInCollection\n      name\n      mediaUrl\n      owner\n\n      mintedBy\n\n      collection {\n        id\n        name\n        transferable\n        sellable\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n": types.NfTsSubscriptionDocument,
    "\n  query NFTQuery($id: String!) {\n    nftById(id: $id) {\n      id\n      idInCollection\n      name\n      description\n      mediaUrl\n      owner\n      createdAt\n\n      collection {\n        id\n        name\n        royalty\n        sellable\n        transferable\n\n        type {\n          type\n        }\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n": types.NftQueryDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query MarketplaceQuery {\n    marketplaceById(id: \"1\") {\n      address\n      collectionTypes {\n        description\n        metaUrl\n        type\n      }\n      metadata\n      config {\n        feePerUploadedFile\n        minimumValueForTrade\n        royaltyToMarketplaceForMint\n        royaltyToMarketplaceForTrade\n        timeBetweenCreateCollections\n      }\n      admins\n    }\n  }\n"): (typeof documents)["\n  query MarketplaceQuery {\n    marketplaceById(id: \"1\") {\n      address\n      collectionTypes {\n        description\n        metaUrl\n        type\n      }\n      metadata\n      config {\n        feePerUploadedFile\n        minimumValueForTrade\n        royaltyToMarketplaceForMint\n        royaltyToMarketplaceForTrade\n        timeBetweenCreateCollections\n      }\n      admins\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query CollectionSub($id: String!) {\n    collectionById(id: $id) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n      permissionToMint\n      userMintLimit\n      paymentForMint\n      transferable\n      sellable\n\n      additionalLinks {\n        discord\n        externalUrl\n        medium\n        xcom\n        telegram\n      }\n    }\n  }\n"): (typeof documents)["\n  query CollectionSub($id: String!) {\n    collectionById(id: $id) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n      permissionToMint\n      userMintLimit\n      paymentForMint\n      transferable\n      sellable\n\n      additionalLinks {\n        discord\n        externalUrl\n        medium\n        xcom\n        telegram\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query LastCreatedCollectionQuery($admin: String!) {\n    collections(where: { admin_eq: $admin }, orderBy: [createdAt_DESC, name_DESC, id_DESC], limit: 1) {\n      createdAt\n    }\n  }\n"): (typeof documents)["\n  query LastCreatedCollectionQuery($admin: String!) {\n    collections(where: { admin_eq: $admin }, orderBy: [createdAt_DESC, name_DESC, id_DESC], limit: 1) {\n      createdAt\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query CollectionsConnectionQuery($where: CollectionWhereInput!) {\n    collectionsConnection(orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      totalCount\n    }\n  }\n"): (typeof documents)["\n  query CollectionsConnectionQuery($where: CollectionWhereInput!) {\n    collectionsConnection(orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      totalCount\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query CollectionsQuery($limit: Int!, $offset: Int!, $where: CollectionWhereInput!) {\n    collections(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n\n      nfts(limit: 5) {\n        id\n        mediaUrl\n      }\n    }\n  }\n"): (typeof documents)["\n  query CollectionsQuery($limit: Int!, $offset: Int!, $where: CollectionWhereInput!) {\n    collections(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n\n      nfts(limit: 5) {\n        id\n        mediaUrl\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query CollectionsSub($limit: Int!, $offset: Int!, $where: CollectionWhereInput!) {\n    collections(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n\n      nfts(limit: 5) {\n        id\n        mediaUrl\n      }\n    }\n  }\n"): (typeof documents)["\n  query CollectionsSub($limit: Int!, $offset: Int!, $where: CollectionWhereInput!) {\n    collections(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n\n      nfts(limit: 5) {\n        id\n        mediaUrl\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query CollectionsNFTsCountQuery($ids: [String!]) {\n    nftsInCollection(collections: $ids) {\n      collection\n      count\n    }\n  }\n"): (typeof documents)["\n  query CollectionsNFTsCountQuery($ids: [String!]) {\n    nftsInCollection(collections: $ids) {\n      collection\n      count\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query NFTsConnectionQuery($where: NftWhereInput!) {\n    nftsConnection(orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      totalCount\n    }\n  }\n"): (typeof documents)["\n  query NFTsConnectionQuery($where: NftWhereInput!) {\n    nftsConnection(orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      totalCount\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query NFTsQuery($limit: Int!, $offset: Int!, $where: NftWhereInput!) {\n    nfts(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      idInCollection\n      name\n      mediaUrl\n      owner\n\n      mintedBy\n\n      collection {\n        id\n        name\n        transferable\n        sellable\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n"): (typeof documents)["\n  query NFTsQuery($limit: Int!, $offset: Int!, $where: NftWhereInput!) {\n    nfts(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      idInCollection\n      name\n      mediaUrl\n      owner\n\n      mintedBy\n\n      collection {\n        id\n        name\n        transferable\n        sellable\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query NFTsSubscription($limit: Int!, $offset: Int!, $where: NftWhereInput!) {\n    nfts(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      idInCollection\n      name\n      mediaUrl\n      owner\n\n      mintedBy\n\n      collection {\n        id\n        name\n        transferable\n        sellable\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n"): (typeof documents)["\n  query NFTsSubscription($limit: Int!, $offset: Int!, $where: NftWhereInput!) {\n    nfts(limit: $limit, offset: $offset, orderBy: [createdAt_DESC, name_DESC, id_DESC], where: $where) {\n      id\n      idInCollection\n      name\n      mediaUrl\n      owner\n\n      mintedBy\n\n      collection {\n        id\n        name\n        transferable\n        sellable\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query NFTQuery($id: String!) {\n    nftById(id: $id) {\n      id\n      idInCollection\n      name\n      description\n      mediaUrl\n      owner\n      createdAt\n\n      collection {\n        id\n        name\n        royalty\n        sellable\n        transferable\n\n        type {\n          type\n        }\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n"): (typeof documents)["\n  query NFTQuery($id: String!) {\n    nftById(id: $id) {\n      id\n      idInCollection\n      name\n      description\n      mediaUrl\n      owner\n      createdAt\n\n      collection {\n        id\n        name\n        royalty\n        sellable\n        transferable\n\n        type {\n          type\n        }\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;