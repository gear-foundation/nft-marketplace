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
    "\n  subscription CollectionQuery($id: String!, $owner: String!) {\n    collectionById(id: $id) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n      permissionToMint\n      userMintLimit\n      paymentForMint\n      transferable\n      sellable\n\n      nfts(where: { owner_contains: $owner }) {\n        id\n        idInCollection\n        name\n        mediaUrl\n        owner\n        mintedBy\n\n        sales(where: { status_eq: \"open\" }) {\n          price\n        }\n\n        auctions(where: { status_eq: \"open\" }) {\n          minPrice\n          lastPrice\n          endTimestamp\n        }\n      }\n\n      additionalLinks {\n        discord\n        externalUrl\n        medium\n        xcom\n        telegram\n      }\n    }\n  }\n": types.CollectionQueryDocument,
    "\n  query LastCreatedCollectionQuery($admin: String!) {\n    collections(where: { admin_eq: $admin }, orderBy: createdAt_DESC, limit: 1) {\n      createdAt\n    }\n  }\n": types.LastCreatedCollectionQueryDocument,
    "\n  subscription CollectionsQuery($admin: String!) {\n    collections(where: { admin_contains: $admin }) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n\n      nfts {\n        id\n        mediaUrl\n      }\n    }\n  }\n": types.CollectionsQueryDocument,
    "\n  query NFTsQuery($first: Int, $after: String, $owner: String!) {\n    nftsConnection(\n      orderBy: createdAt_DESC_NULLS_LAST\n      first: $first\n      after: $after\n      where: { owner_contains: $owner }\n    ) {\n      totalCount\n\n      pageInfo {\n        hasNextPage\n        endCursor\n      }\n\n      edges {\n        cursor\n\n        node {\n          id\n          idInCollection\n          name\n          mediaUrl\n          owner\n\n          collection {\n            id\n            name\n            transferable\n            sellable\n          }\n\n          sales(where: { status_eq: \"open\" }) {\n            price\n          }\n\n          auctions(where: { status_eq: \"open\" }) {\n            minPrice\n            lastPrice\n            endTimestamp\n          }\n        }\n      }\n    }\n  }\n": types.NfTsQueryDocument,
    "\n  subscription NFTQuery($id: String!) {\n    nftById(id: $id) {\n      id\n      idInCollection\n      name\n      description\n      mediaUrl\n      owner\n      createdAt\n\n      collection {\n        id\n        name\n        royalty\n        sellable\n        transferable\n\n        type {\n          type\n        }\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n": types.NftQueryDocument,
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
export function graphql(source: "\n  subscription CollectionQuery($id: String!, $owner: String!) {\n    collectionById(id: $id) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n      permissionToMint\n      userMintLimit\n      paymentForMint\n      transferable\n      sellable\n\n      nfts(where: { owner_contains: $owner }) {\n        id\n        idInCollection\n        name\n        mediaUrl\n        owner\n        mintedBy\n\n        sales(where: { status_eq: \"open\" }) {\n          price\n        }\n\n        auctions(where: { status_eq: \"open\" }) {\n          minPrice\n          lastPrice\n          endTimestamp\n        }\n      }\n\n      additionalLinks {\n        discord\n        externalUrl\n        medium\n        xcom\n        telegram\n      }\n    }\n  }\n"): (typeof documents)["\n  subscription CollectionQuery($id: String!, $owner: String!) {\n    collectionById(id: $id) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n      permissionToMint\n      userMintLimit\n      paymentForMint\n      transferable\n      sellable\n\n      nfts(where: { owner_contains: $owner }) {\n        id\n        idInCollection\n        name\n        mediaUrl\n        owner\n        mintedBy\n\n        sales(where: { status_eq: \"open\" }) {\n          price\n        }\n\n        auctions(where: { status_eq: \"open\" }) {\n          minPrice\n          lastPrice\n          endTimestamp\n        }\n      }\n\n      additionalLinks {\n        discord\n        externalUrl\n        medium\n        xcom\n        telegram\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query LastCreatedCollectionQuery($admin: String!) {\n    collections(where: { admin_eq: $admin }, orderBy: createdAt_DESC, limit: 1) {\n      createdAt\n    }\n  }\n"): (typeof documents)["\n  query LastCreatedCollectionQuery($admin: String!) {\n    collections(where: { admin_eq: $admin }, orderBy: createdAt_DESC, limit: 1) {\n      createdAt\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  subscription CollectionsQuery($admin: String!) {\n    collections(where: { admin_contains: $admin }) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n\n      nfts {\n        id\n        mediaUrl\n      }\n    }\n  }\n"): (typeof documents)["\n  subscription CollectionsQuery($admin: String!) {\n    collections(where: { admin_contains: $admin }) {\n      id\n      name\n      description\n      collectionBanner\n      collectionLogo\n      admin\n      tokensLimit\n\n      nfts {\n        id\n        mediaUrl\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query NFTsQuery($first: Int, $after: String, $owner: String!) {\n    nftsConnection(\n      orderBy: createdAt_DESC_NULLS_LAST\n      first: $first\n      after: $after\n      where: { owner_contains: $owner }\n    ) {\n      totalCount\n\n      pageInfo {\n        hasNextPage\n        endCursor\n      }\n\n      edges {\n        cursor\n\n        node {\n          id\n          idInCollection\n          name\n          mediaUrl\n          owner\n\n          collection {\n            id\n            name\n            transferable\n            sellable\n          }\n\n          sales(where: { status_eq: \"open\" }) {\n            price\n          }\n\n          auctions(where: { status_eq: \"open\" }) {\n            minPrice\n            lastPrice\n            endTimestamp\n          }\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  query NFTsQuery($first: Int, $after: String, $owner: String!) {\n    nftsConnection(\n      orderBy: createdAt_DESC_NULLS_LAST\n      first: $first\n      after: $after\n      where: { owner_contains: $owner }\n    ) {\n      totalCount\n\n      pageInfo {\n        hasNextPage\n        endCursor\n      }\n\n      edges {\n        cursor\n\n        node {\n          id\n          idInCollection\n          name\n          mediaUrl\n          owner\n\n          collection {\n            id\n            name\n            transferable\n            sellable\n          }\n\n          sales(where: { status_eq: \"open\" }) {\n            price\n          }\n\n          auctions(where: { status_eq: \"open\" }) {\n            minPrice\n            lastPrice\n            endTimestamp\n          }\n        }\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  subscription NFTQuery($id: String!) {\n    nftById(id: $id) {\n      id\n      idInCollection\n      name\n      description\n      mediaUrl\n      owner\n      createdAt\n\n      collection {\n        id\n        name\n        royalty\n        sellable\n        transferable\n\n        type {\n          type\n        }\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n"): (typeof documents)["\n  subscription NFTQuery($id: String!) {\n    nftById(id: $id) {\n      id\n      idInCollection\n      name\n      description\n      mediaUrl\n      owner\n      createdAt\n\n      collection {\n        id\n        name\n        royalty\n        sellable\n        transferable\n\n        type {\n          type\n        }\n      }\n\n      sales(where: { status_eq: \"open\" }) {\n        price\n      }\n\n      auctions(where: { status_eq: \"open\" }) {\n        minPrice\n        lastPrice\n        endTimestamp\n      }\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;