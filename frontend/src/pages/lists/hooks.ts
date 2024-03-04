import { useState } from 'react';
import { useQuery, useSubscription } from 'urql';

import { NfTsQuerySubscription } from '@/graphql/graphql';

import { COLLECTIONS_QUERY, NFTS_CONNECTION_QUERY, NFTS_QUERY } from './consts';

function useCollections(admin: string) {
  const [result] = useSubscription({ query: COLLECTIONS_QUERY, variables: { admin } });

  const collections = result.data?.collections;
  const isCollectionsQueryReady = !result.fetching;

  return { collections, isCollectionsQueryReady };
}

function useNFTs(owner: string) {
  const LIMIT = 16;
  const [offset, setOffset] = useState(0);

  const [paginationResult] = useQuery({ query: NFTS_CONNECTION_QUERY, variables: { owner } });
  const nftsCount = paginationResult.data?.nftsConnection.totalCount || 0;

  const getMergedNFTs = (acc: NfTsQuerySubscription = { nfts: [] }, query: NfTsQuerySubscription) => ({
    nfts: [...acc.nfts, ...query.nfts],
  });

  const [result] = useSubscription({ query: NFTS_QUERY, variables: { owner, limit: LIMIT, offset } }, getMergedNFTs);

  const nfts = result.data?.nfts || [];
  const isNFTsQueryReady = !result.fetching && !paginationResult.fetching;
  const isMoreNFTs = nfts.length !== nftsCount;

  const fetchNFTs = () => setOffset((prevOffset) => prevOffset + LIMIT);

  return { nfts, isNFTsQueryReady, nftsCount, isMoreNFTs, fetchNFTs };
}

export { useNFTs, useCollections };
