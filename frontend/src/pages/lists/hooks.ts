import { useQuery } from '@apollo/client';
import { useCallback, useEffect, useMemo } from 'react';

import { CollectionWhereInput, NftWhereInput } from '@/graphql/graphql';

import {
  COLLECTIONS_CONNECTION_QUERY,
  COLLECTIONS_NFTS_COUNT_QUERY,
  COLLECTIONS_QUERY,
  COLLECTIONS_SUBSCRIPTION,
  DEFAULT_VARIABLES,
  NFTS_CONNECTION_QUERY,
  NFTS_QUERY,
  NFTS_SUBSCRIPTION,
} from './consts';
import { getCollectionFilters, getNftFilters } from './utils';

function useCollectionsNFTsCount(ids: string[]) {
  const { data } = useQuery(COLLECTIONS_NFTS_COUNT_QUERY, { variables: { ids }, skip: !ids.length });

  return data?.nftsInCollection || [];
}

function useTotalCollectionsCount(where: CollectionWhereInput) {
  const { data, loading } = useQuery(COLLECTIONS_CONNECTION_QUERY, {
    variables: { ...DEFAULT_VARIABLES.COLLECTIONS, where },
  });

  const totalCount = data?.collectionsConnection.totalCount || 0;

  return [totalCount, !loading] as const;
}

function useCollections(admin: string) {
  const where = useMemo(() => getCollectionFilters(admin), [admin]);
  const [totalCount, isTotalCountReady] = useTotalCollectionsCount(where);

  const { data, loading, fetchMore, subscribeToMore } = useQuery(COLLECTIONS_QUERY, {
    variables: { ...DEFAULT_VARIABLES.COLLECTIONS, where },
  });

  const collections = useMemo(() => data?.collections || [], [data]);
  const collectionsCount = collections.length;

  const collectionIds = useMemo(() => collections.map(({ id }) => id), [collections]);
  const nftsCounts = useCollectionsNFTsCount(collectionIds);

  const collectionsWithCounts = useMemo(() => {
    return collections.map((collection) => {
      const nftsCount = nftsCounts.find(({ collection: id }) => collection.id === id)?.count;

      return { ...collection, nftsCount };
    });
  }, [collections, nftsCounts]);

  const isReady = !loading && isTotalCountReady;
  const hasMore = totalCount && collectionsCount ? collectionsCount < totalCount : false;

  // useEffect(() => {
  //   if (loading) return;

  //   const limit = collectionsCount || 1; // 1 fallback, cuz in case of empty list with limit 0, subscription won't work
  //   const offset = 0;

  //   // same solution as for nfts, but for newly created collections
  //   const unsubscribe = subscribeToMore({
  //     document: COLLECTIONS_SUBSCRIPTION,
  //     variables: { limit, offset, where },
  //     updateQuery: (prev = { collections: [] }, { subscriptionData }) => {
  //       if (!subscriptionData.data) return { collections: [] };

  //       const newCollections = subscriptionData.data.collections.filter(
  //         (subCollection) => !prev.collections.some((collection) => collection.id === subCollection.id),
  //       );

  //       return { collections: [...newCollections, ...prev.collections] };
  //     },
  //   });

  //   return () => {
  //     unsubscribe();
  //   };
  // }, [subscribeToMore, collectionsCount, where, loading]);

  const fetchCollections = useCallback(() => {
    const offset = collectionsCount;

    fetchMore({ variables: { offset } }).catch(console.error);
  }, [collectionsCount, fetchMore]);

  return [collectionsWithCounts, totalCount, hasMore, isReady, fetchCollections] as const;
}

function useTotalNFTsCount(where: NftWhereInput) {
  const { data, loading } = useQuery(NFTS_CONNECTION_QUERY, { variables: { where } });

  const totalCount = data?.nftsConnection?.totalCount || 0;

  return [totalCount, !loading] as const;
}

function useNFTs(owner: string, collectionId?: string) {
  const where = useMemo(() => getNftFilters(owner, collectionId), [owner, collectionId]);
  const [totalCount, isTotalCountReady] = useTotalNFTsCount(where);

  const { data, loading, fetchMore, subscribeToMore } = useQuery(NFTS_QUERY, {
    variables: { ...DEFAULT_VARIABLES.NFTS, where },
  });

  const nfts = data?.nfts || [];
  const nftsCount = nfts.length;

  // TODO: if new nfts would be minted, totalCount will remain the same
  const hasMoreNFTs = totalCount && nftsCount ? nftsCount < totalCount : false;
  const isNFTsQueryReady = !loading && isTotalCountReady;

  // useEffect(() => {
  //   if (loading) return;

  //   const limit = nftsCount || 1; // 1 fallback, cuz in case of empty list with limit 0, subscription won't work
  //   const offset = 0;

  //   // kinda tricky subscription to handle live interaction,
  //   // works for now, but worth to reconsider them later.
  //   // maybe would be better to use connection's cursor pagination?
  //   const unsubscribe = subscribeToMore({
  //     document: NFTS_SUBSCRIPTION,
  //     variables: { limit, offset, where },
  //     updateQuery: (prev = { nfts: [] }, { subscriptionData }) => {
  //       // extracting newly minted nfts, merge type policy should handle the rest
  //       if (!subscriptionData.data) return { nfts: [] };

  //       // important to preserve consistent sorting of nfts, otherwise results will be inaccurate
  //       const mintedNfts = subscriptionData.data.nfts.filter(
  //         (subNft) => !prev.nfts.some((nft) => nft.id === subNft.id),
  //       );

  //       return { nfts: [...mintedNfts, ...prev.nfts] };
  //     },
  //   });

  //   return () => {
  //     unsubscribe();
  //   };
  // }, [subscribeToMore, nftsCount, where, loading]);

  const fetchNFTs = useCallback(() => {
    const offset = nftsCount;

    fetchMore({ variables: { offset } }).catch(console.error);
  }, [fetchMore, nftsCount]);

  return [nfts, totalCount, hasMoreNFTs, isNFTsQueryReady, fetchNFTs] as const;
}

export { useNFTs, useCollections };
