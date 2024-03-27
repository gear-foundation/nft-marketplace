import DiscordSVG from '@/assets/discord.svg?react';
import MediumSVG from '@/assets/medium.svg?react';
import TelegramSVG from '@/assets/telegram.svg?react';
import TwitterSVG from '@/assets/twitter.svg?react';
import WebSVG from '@/assets/web.svg?react';
import { graphql } from '@/graphql';

const SOCIAL_ICON = {
  externalUrl: WebSVG,
  telegram: TelegramSVG,
  xcom: TwitterSVG,
  medium: MediumSVG,
  discord: DiscordSVG,
};

const COLLECTION_SUBSCRIPTION = graphql(`
  query CollectionSub($id: String!) {
    collectionById(id: $id) {
      id
      name
      description
      collectionBanner
      collectionLogo
      admin
      tokensLimit
      permissionToMint
      userMintLimit
      paymentForMint
      transferable
      sellable

      additionalLinks {
        discord
        externalUrl
        medium
        xcom
        telegram
      }
    }
  }
`);

export { SOCIAL_ICON, COLLECTION_SUBSCRIPTION };
