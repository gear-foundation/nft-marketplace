import { ADDRESS } from './consts';
import { Entries } from './types';

const cx = (...args: unknown[]) =>
  args
    .filter((arg) => typeof arg === 'string')
    .join(' ')
    .trim();

const getTypedEntries = <T extends object>(value: T) => Object.entries(value) as Entries<T>;

const getIpfsLink = (value: string) => {
  const [, cid = ''] = value.split('ipfs://');

  // if (!cid) throw new Error(`Can't find CID in a link: ${value}`);

  return `${ADDRESS.IPFS_GATEWAY}/${cid}`;
};

export { cx, getTypedEntries, getIpfsLink };
