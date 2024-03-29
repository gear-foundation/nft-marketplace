// in case Object.entries return value is immutable

import { FunctionComponent, SVGProps } from 'react';

// ref: https://stackoverflow.com/a/60142095
type Entries<T> = {
  [K in keyof T]: [K, T[K]];
}[keyof T][];

type SVGComponent = FunctionComponent<
  SVGProps<SVGSVGElement> & {
    title?: string | undefined;
  }
>;

export type { Entries, SVGComponent };
