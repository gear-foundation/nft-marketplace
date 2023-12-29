import { assertNotNull } from '@subsquid/util-internal';
import { lookupArchive } from '@subsquid/archive-registry';
import {
  BlockHeader,
  DataHandlerContext,
  SubstrateBatchProcessor,
  SubstrateBatchProcessorFields,
  Event as _Event,
  Call as _Call,
  Extrinsic as _Extrinsic,
} from '@subsquid/substrate-processor';

export const processor = new SubstrateBatchProcessor()
  .setDataSource({
    // archive: lookupArchive('vara-testnet', { release: 'ArrowSquid' }),
    // Chain RPC endpoint is required on Substrate for metadata and real-time updates
    chain: {
      url: assertNotNull(process.env.RPC_ENDPOINT),
      rateLimit: 100,
    },
  })
  .addEvent({
    name: ['Gear.UserMessageSent'],
    extrinsic: true,
  })
  .setFields({
    event: {
      args: true,
    },
    extrinsic: {
      hash: true,
      fee: true,
    },
    block: {
      timestamp: true,
    },
  })
  .setBlockRange({ from: 2608260 });

export type Fields = SubstrateBatchProcessorFields<typeof processor>;
export type Block = BlockHeader<Fields>;
export type Event = _Event<Fields>;
export type Call = _Call<Fields>;
export type Extrinsic = _Extrinsic<Fields>;
export type ProcessorContext<Store> = DataHandlerContext<Store, Fields>;
