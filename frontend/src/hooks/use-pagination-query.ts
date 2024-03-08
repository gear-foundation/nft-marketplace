import { useState, useLayoutEffect, useCallback } from 'react';
import { DocumentInput, useQuery } from 'urql';

import { PageInfo, Scalars } from '@/graphql/graphql';

type Edge<T> = {
  node: T;
};

type Connection<T> = {
  edges: Array<Edge<T>>;
  pageInfo: Pick<PageInfo, 'endCursor' | 'hasNextPage'>;
  totalCount: Scalars['Int']['output'];
};

type ConnectionVariables = {
  first: number;
  after: string | null;
};

type Query<T> = {
  [key: string]: Connection<T>;
};

function usePaginationQuery<TData, TVariables extends ConnectionVariables>(
  query: DocumentInput<Query<TData>, TVariables>,
  _variables: Omit<TVariables, 'first' | 'after'>,
  limit: number,
  connectionKey: string,
) {
  const [variables, setVariables] = useState<typeof _variables>({ ..._variables, first: limit, after: null });
  const [result, setResult] = useState<TData[]>([]);

  const [{ data, fetching }] = useQuery({ query, variables });
  const isReady = !fetching;

  const connection = data?.[connectionKey];
  const { pageInfo, edges, totalCount } = connection || {};
  const { endCursor, hasNextPage } = pageInfo || {};

  // useLayoutEffect to preserve consistency between accumulated result and useQuary state variables
  useLayoutEffect(() => {
    setResult([]);
    setVariables((prevVariables) => ({ ...prevVariables, ..._variables, after: null }));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [...Object.values(_variables)]);

  useLayoutEffect(() => {
    // looking for a fetching cuz variables reset doesn't guarantee data change
    if (fetching) return;

    const nodes = edges?.map(({ node }) => node) || [];
    setResult((prevData) => (prevData ? [...prevData, ...nodes] : nodes));
  }, [edges, fetching]);

  const fetchNFTs = useCallback(() => {
    if (!endCursor) throw new Error('Pointer to the end of the list is not found');

    setVariables((prevVariables) => ({ ...prevVariables, after: endCursor }));
  }, [endCursor]);

  return [result, isReady, totalCount, Boolean(hasNextPage), fetchNFTs] as const;
}

export { usePaginationQuery };
