import { graphql } from '@/graphql';

const NFT_QUERY = graphql(`
  query NFTQuery($id: String!) {
    nftById(id: $id) {
      id
      idInCollection
      name
      description
      mediaUrl
      owner
      createdAt

      collection {
        id
        name
        royalty
        sellable
        transferable

        type {
          type
        }
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

export { NFT_QUERY };
