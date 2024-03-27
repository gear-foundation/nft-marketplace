import { useQuery, useSubscription } from '@apollo/client';

// TODO: reusing temporary solution for nfts subscriptiption
import { useNFTs } from '../lists/hooks';

import { COLLECTION_SUBSCRIPTION } from './consts';

function useCollection(id: string, owner: string) {
  const [nfts, nftsCount, hasMoreNFTs, isNFTsQueryReady, fetchNFTs] = useNFTs(owner, id);

  // subscription to handle update in case of newly created collection
  const { data, loading } = useQuery(COLLECTION_SUBSCRIPTION, { variables: { id } });
  const collection = data?.collectionById;
  const isCollectionQueryReady = !loading && isNFTsQueryReady;

  return [collection, nfts, nftsCount, hasMoreNFTs, isCollectionQueryReady, fetchNFTs] as const;
}

export { useCollection };
