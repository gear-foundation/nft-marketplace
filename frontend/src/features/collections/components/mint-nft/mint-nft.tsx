import { useAlert } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';

import { withAccount } from '@/components';
import { Collection, CollectionType, Nft } from '@/graphql/graphql';
import { useCollectionMessage } from '@/hooks';

type Props = Pick<Collection, 'id' | 'tokensLimit' | 'paymentForMint'> & {
  nfts: Pick<Nft, 'id'>[];
} & {
  type: Pick<CollectionType, 'id'>;
};

function Component({ id, type, tokensLimit, paymentForMint, nfts }: Props) {
  const sendMessage = useCollectionMessage(id, type.id);
  const alert = useAlert();

  const tokensToMint = tokensLimit ? Number(tokensLimit) - nfts.length : 1;

  const handleClick = () => {
    if (!paymentForMint) throw new Error('paymentForMint is not exists');

    const onSuccess = () => alert.success('NFT minted');

    sendMessage({ payload: { Mint: null }, value: paymentForMint, onSuccess });
  };

  return tokensToMint > 0 ? <Button text="Mint NFT" size="small" onClick={handleClick} block /> : null;
}

const MintNFT = withAccount(Component);

export { MintNFT };
