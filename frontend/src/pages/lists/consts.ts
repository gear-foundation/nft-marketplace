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

const NFTS_QUERY = graphql(`
  query NFTsQuery($first: Int, $after: String, $owner: String!) {
    nftsConnection(
      orderBy: createdAt_DESC_NULLS_LAST
      first: $first
      after: $after
      where: { owner_contains: $owner }
    ) {
      totalCount

      pageInfo {
        hasNextPage
        endCursor
      }

      edges {
        cursor

        node {
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
    }
  }
`);

export { COLLECTIONS_QUERY, NFTS_QUERY };
