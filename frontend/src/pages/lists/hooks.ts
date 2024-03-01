import { useQuery, useSubscription } from 'urql';

import { COLLECTIONS_QUERY, NFTS_QUERY } from './consts';

function useCollections(admin: string) {
  const [result] = useSubscription({ query: COLLECTIONS_QUERY, variables: { admin } });

  const collections = result.data?.collections;
  const isCollectionsQueryReady = !result.fetching;

  return { collections, isCollectionsQueryReady };
}

function useNFTs(owner: string) {
  const [result] = useQuery({ query: NFTS_QUERY, variables: { owner, first: 9, after: null } });

  const nfts = result.data?.nftsConnection.edges.map(({ node }) => node) || [];
  const isNFTsQueryReady = !result.fetching;

  return { nfts, isNFTsQueryReady };
}

export { useNFTs, useCollections };
