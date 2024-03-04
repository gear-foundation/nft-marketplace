import { graphql } from '@/graphql';

const COLLECTIONS_QUERY = graphql(`
  subscription CollectionsQuery($admin: String!) {
    collections(where: { admin_contains: $admin }) {
      id
      name
      description
      collectionBanner
      collectionLogo
      admin
      tokensLimit

      nfts {
        id
        mediaUrl
      }
    }
  }
`);

const NFTS_CONNECTION_QUERY = graphql(`
  query NFTsConnectionQuery($owner: String!) {
    nftsConnection(orderBy: createdAt_DESC_NULLS_LAST, where: { owner_contains: $owner }) {
      totalCount
    }
  }
`);

const NFTS_QUERY = graphql(`
  subscription NFTsQuery($limit: Int, $offset: Int, $owner: String!) {
    nfts(limit: $limit, offset: $offset, orderBy: createdAt_DESC_NULLS_LAST, where: { owner_contains: $owner }) {
      id
      idInCollection
      name
      mediaUrl
      owner

      collection {
        id
        name
        transferable
        sellable
      }

      sales(where: { status_eq: "open" }) {
        price
      }

      auctions(where: { status_eq: "open" }) {
        minPrice
        lastPrice
        endTimestamp
      }
    }
  }
`);

export { COLLECTIONS_QUERY, NFTS_CONNECTION_QUERY, NFTS_QUERY };
