import { Identicon } from '@polkadot/react-identicon';
import { Link, generatePath } from 'react-router-dom';

import { ResponsiveSquareImage } from '@/components';
import { ROUTE } from '@/consts';
import { Collection, Nft } from '@/graphql/graphql';
import { getIpfsLink } from '@/utils';

import PlaceholderSrc from '../../assets/placeholder.svg';
import { MintLimitInfoCard } from '../mint-limit-info-card';

import styles from './collection-card.module.scss';

type Props = Pick<Collection, 'id' | 'name' | 'collectionBanner' | 'collectionLogo' | 'admin' | 'tokensLimit'> & {
  nfts: Pick<Nft, 'id' | 'mediaUrl'>[];
} & {
  nftsCount: number | undefined;
};

const PREVIEW_NFTS_COUNT = 5;
const DEFAULT_NFTS = new Array<null>(PREVIEW_NFTS_COUNT).fill(null);

function CollectionCard({ id, name, collectionBanner, collectionLogo, admin, tokensLimit, nfts, nftsCount }: Props) {
  const renderNFTs = () =>
    (nfts.length < PREVIEW_NFTS_COUNT ? [...nfts, ...DEFAULT_NFTS] : nfts)
      .slice(0, PREVIEW_NFTS_COUNT)
      .map((nft, index) => (
        <li key={nft ? nft.id : `${id}-${index}`}>
          {/* TODO: should be background color instead of placeholder svg  */}
          <ResponsiveSquareImage src={nft ? getIpfsLink(nft.mediaUrl) : PlaceholderSrc} />
        </li>
      ));

  return (
    // TODO: get rid of li
    <li className={styles.collection}>
      <Link to={generatePath(ROUTE.COLLECTION, { id })}>
        <div className={styles.cover}>
          <img src={getIpfsLink(collectionBanner)} alt="" />
        </div>

        <div className={styles.summary}>
          <img src={getIpfsLink(collectionLogo)} alt="" className={styles.logo} />

          <div className={styles.text}>
            <h3 className={styles.name}>{name}</h3>

            <div className={styles.user}>
              <Identicon value={admin} size={14} theme="polkadot" />
              <span className={styles.address}>{admin}</span>
            </div>
          </div>
        </div>

        <div className={styles.cards}>
          <MintLimitInfoCard heading={tokensLimit} text={nftsCount} color="dark" />
        </div>

        <ul className={styles.nfts}>{renderNFTs()}</ul>
      </Link>
    </li>
  );
}

export { CollectionCard };
