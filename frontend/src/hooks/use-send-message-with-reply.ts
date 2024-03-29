import { HexString, ProgramMetadata, UserMessageSent } from '@gear-js/api';
import { useApi, useAccount, useAlert, useSendMessageWithGas } from '@gear-js/react-hooks';
import { UnsubscribePromise } from '@polkadot/api/types';
import { AnyJson } from '@polkadot/types/types';

import { MintNFTPayload, TransferNFTPayload, ApproveNFTPayload } from '@/features/collections';
import { CreateCollectionPayload, CreateCollectionReply } from '@/features/create-simple-collection';
import { BuyNFTPayload, MakeBidPayload, StartAuctionPayload, StartSalePayload } from '@/features/marketplace';
import { isObject } from '@/utils';

type Payload =
  | CreateCollectionPayload
  | BuyNFTPayload
  | MakeBidPayload
  | StartAuctionPayload
  | StartSalePayload
  | MintNFTPayload
  | TransferNFTPayload
  | ApproveNFTPayload;

type Reply<T> = T extends CreateCollectionPayload
  ? CreateCollectionReply
  : T extends BuyNFTPayload
  ? { nftSold: unknown }
  : T extends MakeBidPayload
  ? { bidAdded: unknown }
  : T extends StartAuctionPayload
  ? { auctionCreated: unknown }
  : T extends StartSalePayload
  ? { saleNft: unknown }
  : T extends MintNFTPayload
  ? { minted: unknown }
  : T extends TransferNFTPayload
  ? { transferred: unknown }
  : T extends ApproveNFTPayload
  ? { approved: unknown }
  : AnyJson;

const useSendMessageWithReply = (programId: HexString, metadata: ProgramMetadata | undefined) => {
  const { api, isApiReady } = useApi();
  const { account } = useAccount();
  const alert = useAlert();

  const sendMessage = useSendMessageWithGas(programId, metadata, { disableAlerts: true, gasMultiplier: 1.1 });

  // TODO: different payload types for marketplace and collection hooks
  return <T extends Payload>({
    onSuccess = () => {},
    onFinally = () => {},
    ...sendMessageArgs
  }: {
    payload: T;
    value?: string | number;
    onSuccess?: (value: Reply<T>) => void;
    onFinally?: () => void;
  }) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!account) throw new Error('Account is not found');

    let unsub: UnsubscribePromise | undefined = undefined;

    const _onFinally = () => {
      onFinally();

      // for dev purposes only, since unsub is tricky
      if (!unsub) throw new Error('Failed to unsubscribe from reply');

      unsub.then((unsubCallback) => unsubCallback()).catch((error: Error) => alert.error(error.message));
    };

    const handleUserMessageSent = ({ data }: UserMessageSent) => {
      try {
        if (!metadata) throw new Error('Failed to get transaction result: metadata is not found');

        const typeIndex = metadata.types.handle.output;
        if (typeIndex === null)
          throw new Error('Failed to get transaction result: handle.output type index is not found');

        const { message } = data;
        const { source, destination, payload, details } = message;

        if (source.toHex() !== programId || destination.toHex() !== account.decodedAddress) return;

        const isSuccess = details.isSome ? details.unwrap().code.isSuccess : true;
        if (!isSuccess) throw new Error(payload.toHuman()?.toString());

        const decodedPayload = metadata.createType(typeIndex, payload).toJSON();

        if (!isObject(decodedPayload)) throw new Error('Failed to get transaction result: payload is not an object');

        const isErrorPayload = Object.prototype.hasOwnProperty.call(decodedPayload, 'err');
        if (isErrorPayload) throw new Error(decodedPayload.err?.toString());

        const isSuccessPayload = Object.prototype.hasOwnProperty.call(decodedPayload, 'ok');
        if (!isSuccessPayload) throw new Error('Failed to get transaction result: ok property is not found');

        onSuccess(decodedPayload.ok as Reply<T>);
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        alert.error(errorMessage);
      }

      _onFinally();
    };

    unsub = api.gearEvents.subscribeToGearEvent('UserMessageSent', handleUserMessageSent);

    sendMessage({ ...sendMessageArgs, onError: _onFinally });
  };
};

export { useSendMessageWithReply };
