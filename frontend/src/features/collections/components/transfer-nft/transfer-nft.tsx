import { decodeAddress } from '@gear-js/api';
import { Button, Input } from '@gear-js/vara-ui';
import { z } from 'zod';

import { NFTActionFormModal, withAccount } from '@/components';
import { Collection, CollectionType, Nft } from '@/graphql/graphql';
import { useCollectionMessage, useIsOwner, useModal } from '@/hooks';

import PlaneSVG from '../../assets/plane.svg?react';

const defaultValues = {
  address: '',
};

const isValidAddress = (address: string) => {
  try {
    decodeAddress(address);
    return true;
  } catch {
    return false;
  }
};

const schema = z.object({
  address: z
    .string()
    .trim()
    .min(0)
    .refine((value) => isValidAddress(value), 'Invalid address')
    .transform((value) => decodeAddress(value)),
});

type Props = Pick<Nft, 'idInCollection' | 'name' | 'mediaUrl' | 'owner'> & {
  collection: Pick<Collection, 'id' | 'name' | 'transferable'> & {
    type: Pick<CollectionType, 'id'>;
  };
};

function Component({ collection, owner, ...nft }: Props) {
  const [isOpen, open, close] = useModal();
  const isOwner = useIsOwner(owner);

  const sendMessage = useCollectionMessage(collection.id, collection.type.id);

  const onSubmit = ({ address }: typeof defaultValues) => {
    const tokenId = nft.idInCollection;
    const to = address;

    const payload = { Transfer: { tokenId, to } };
    const onSuccess = close;

    sendMessage({ payload, onSuccess });
  };

  const modalProps = { heading: 'Transfer NFT', close };
  const formProps = { defaultValues, schema, onSubmit };

  return isOwner && collection.transferable ? (
    <>
      <Button icon={PlaneSVG} text="Transfer" size="small" color="dark" onClick={open} />

      {isOpen && (
        <NFTActionFormModal modal={modalProps} form={formProps} nft={nft} collection={collection}>
          <Input label="Account address" name="address" />
        </NFTActionFormModal>
      )}
    </>
  ) : null;
}

const TransferNFT = withAccount(Component);

export { TransferNFT };
