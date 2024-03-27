import { InfoCard, InfoCardProps } from '@/components';

import LandscapeSVG from '../../assets/landscape.svg?react';

type Props = Omit<InfoCardProps, 'heading' | 'text' | 'SVG'> & {
  heading: string | null | undefined;
  text: number | undefined;
};

function MintLimitInfoCard({ heading, text, ...props }: Props) {
  return (
    <InfoCard
      heading={heading ? `of ${heading} to be minted` : 'Unlimited series'}
      text={text !== undefined ? `${text} NFTs` : undefined}
      SVG={LandscapeSVG}
      {...props}
    />
  );
}

export { MintLimitInfoCard };
