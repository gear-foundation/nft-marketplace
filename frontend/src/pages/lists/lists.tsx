import { Link, useLocation, useMatch } from 'react-router-dom';

import { Container, List } from '@/components';
import { ROUTE } from '@/consts';
import { CollectionCard, NFTCard, Skeleton } from '@/features/collections';
import CollectionCardSkeletonSVG from '@/features/collections/assets/collection-card-skeleton.svg?react';
import NFTCardSkeletonSVG from '@/features/collections/assets/nft-card-skeleton.svg?react';
import { AccountFilter, GridSize, useAccountFilter, useGridSize } from '@/features/lists';
import { GRID_SIZE } from '@/features/lists/consts';
import { usePaginationQuery } from '@/hooks';
import { cx } from '@/utils';

import { COLLECTIONS_QUERY, NFTS_QUERY } from './consts';
import styles from './lists.module.scss';

const TABS = [
  { to: ROUTE.HOME, text: 'Collections' },
  { to: ROUTE.NFTS, text: 'NFTs' },
];

function Lists() {
  const { pathname } = useLocation();
  const match = useMatch(ROUTE.NFTS);

  const { gridSize, setGridSize } = useGridSize();
  const { accountFilterValue, accountFilterAddress, setAccountFilterValue } = useAccountFilter();

  const [collections, isCollectionsQueryReady, collectionsCount, hasMoreCollections, fetchCollections] =
    usePaginationQuery(COLLECTIONS_QUERY, { admin: accountFilterAddress }, 12, 'collectionsConnection');

  const [nfts, isNFTsQueryReady, nftsCount, hasMoreNFTs, fetchNFTs] = usePaginationQuery(
    NFTS_QUERY,
    { owner: accountFilterAddress },
    16,
    'nftsConnection',
  );

  const renderTabs = () =>
    TABS.map(({ to, text }, index) => {
      const counter = [collectionsCount, nftsCount][index];

      return (
        <li key={to}>
          <Link to={to} className={cx(styles.tab, to === pathname && styles.active)}>
            {text} {counter !== undefined && `(${counter})`}
          </Link>
        </li>
      );
    });

  return (
    <Container>
      <header className={styles.header}>
        <ul className={styles.tabs}>{renderTabs()}</ul>

        <div className={styles.options}>
          <AccountFilter value={accountFilterValue} onChange={setAccountFilterValue} />
          <GridSize value={gridSize} onChange={setGridSize} />
        </div>
      </header>

      {match ? (
        <List
          items={nfts}
          itemsPerRow={gridSize === GRID_SIZE.SMALL ? 4 : 3}
          emptyText="Mint NFTs"
          renderItem={(nft) => <NFTCard key={nft.id} {...nft} />}
          fetchItems={fetchNFTs}
          isMoreItems={hasMoreNFTs}
          skeleton={{
            rowsCount: 2,
            isVisible: !isNFTsQueryReady,
            renderItem: (index) => (
              <li key={index}>
                <Skeleton>
                  <NFTCardSkeletonSVG />
                </Skeleton>
              </li>
            ),
          }}
        />
      ) : (
        <List
          items={collections}
          itemsPerRow={gridSize === GRID_SIZE.SMALL ? 3 : 2}
          emptyText="Create collections"
          renderItem={(collection) => <CollectionCard key={collection.id} {...collection} />}
          fetchItems={fetchCollections}
          isMoreItems={hasMoreCollections}
          skeleton={{
            rowsCount: 2,
            isVisible: !isCollectionsQueryReady,
            renderItem: (index) => (
              <li key={index}>
                <Skeleton>
                  <CollectionCardSkeletonSVG />
                </Skeleton>
              </li>
            ),
          }}
        />
      )}
    </Container>
  );
}

export { Lists };
