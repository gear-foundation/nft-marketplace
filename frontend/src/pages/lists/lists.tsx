import { Link, useLocation, useMatch } from 'react-router-dom';

import { Container, List } from '@/components';
import { LoadingList } from '@/components/list/loading-list';
import { ROUTE } from '@/consts';
import { CollectionCard, NFTCard, Skeleton } from '@/features/collections';
import CollectionCardSkeletonSVG from '@/features/collections/assets/collection-card-skeleton.svg?react';
import NFTCardSkeletonSVG from '@/features/collections/assets/nft-card-skeleton.svg?react';
import { AccountFilter, GridSize, useAccountFilter, useGridSize } from '@/features/lists';
import { GRID_SIZE } from '@/features/lists/consts';
import { cx } from '@/utils';

import { useCollections, useNFTs } from './hooks';
import styles from './lists.module.scss';

const TABS = [
  { to: ROUTE.HOME, text: 'Collections' },
  { to: ROUTE.NFTS, text: 'NFTs' },
];

const COLLECTION_SKELETONS = new Array<null>(6).fill(null);
const NFT_SKELETONS = new Array<null>(8).fill(null);

function Lists() {
  const { pathname } = useLocation();
  const match = useMatch(ROUTE.NFTS);

  const { gridSize, setGridSize } = useGridSize();
  const { accountFilterValue, accountFilterAddress, setAccountFilterValue } = useAccountFilter();

  const { collections, isCollectionsQueryReady } = useCollections(accountFilterAddress);
  const { nfts, isNFTsQueryReady } = useNFTs(accountFilterAddress);

  const renderTabs = () =>
    TABS.map(({ to, text }, index) => {
      const counter = [collections?.length, nfts?.length][index];

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
        <LoadingList
          items={nfts}
          itemsPerRow={gridSize === GRID_SIZE.SMALL ? 4 : 3}
          emptyText="Mint NFTs"
          renderItem={(nft) => <NFTCard key={nft.id} {...nft} />}
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
          items={isCollectionsQueryReady ? collections : COLLECTION_SKELETONS}
          itemsPerRow={gridSize === GRID_SIZE.SMALL ? 3 : 2}
          emptyText="Create collections"
          renderItem={(collection, index) =>
            collection ? (
              <CollectionCard key={collection.id} {...collection} />
            ) : (
              <li key={index}>
                <Skeleton>
                  <CollectionCardSkeletonSVG />
                </Skeleton>
              </li>
            )
          }
        />
      )}
    </Container>
  );
}

export { Lists };
