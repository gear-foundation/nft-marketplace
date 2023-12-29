import { TypeormDatabase } from '@subsquid/typeorm-store';

import { processor } from './processor';
import { config } from './config';
import { EventsProcessing } from './processing/events.processing';
import {Collection} from "./model";

processor.run(new TypeormDatabase(), async (ctx) => {
  const processing = new EventsProcessing(ctx.store);
  for (const block of ctx.blocks) {
    const events = [
      ...block.events,
      ...block.extrinsics.flatMap((e) => e.events),
    ].filter((e) => e.args && e.args.message);
    for (const item of events) {
      const {
        message: { source, payload, details },
      } = item.args;
      if (payload === '0x') {
        continue;
      }
      if (details && details.code.__kind !== 'Success') {
        continue;
      }
      const collection = await ctx.store.findOne(Collection, { where: { id: source } });
      if (collection) {
        await processing.handleNftEvent(block, payload, source);
      }
      if (config.marketplaceProgram && config.marketplaceProgram === source) {
        await processing.handleMarketplaceEvent(block, payload, source);
      }
    }
  }
  await processing.saveAll();
});
