import {
  Entity as Entity_,
  Column as Column_,
  PrimaryColumn as PrimaryColumn_,
  ManyToOne as ManyToOne_,
  Index as Index_,
  OneToMany as OneToMany_,
} from 'typeorm';
import * as marshal from './marshal';
import { Marketplace } from './marketplace.model';
import { CollectionType } from './collectionType.model';
import { Nft } from './nft.model';

@Entity_()
export class Collection {
  constructor(props?: Partial<Collection>) {
    Object.assign(this, props);
  }

  @PrimaryColumn_()
  id!: string;

  @Index_()
  @ManyToOne_(() => Marketplace, { nullable: true })
  marketplace!: Marketplace;

  @Index_()
  @ManyToOne_(() => CollectionType, { nullable: true })
  type!: CollectionType;

  @Column_('text', { nullable: false })
  admin!: string;

  @Column_('text', { nullable: false })
  name!: string;

  @Column_('text', { nullable: false })
  description!: string;

  @Column_('numeric', {
    transformer: marshal.bigintTransformer,
    nullable: true,
  })
  userMintLimit!: bigint | undefined | null;

  @Column_('numeric', {
    transformer: marshal.bigintTransformer,
    nullable: true,
  })
  tokensLimit!: bigint | undefined | null;

  @Column_('text', { nullable: false })
  collectionImage!: string;

  @Column_('bool', { nullable: false })
  transferable!: boolean;

  @Column_('bool', { nullable: false })
  approvable!: boolean;

  @Column_('bool', { nullable: false })
  burnable!: boolean;

  @Column_('bool', { nullable: false })
  sellable!: boolean;

  @Column_('bool', { nullable: false })
  attandable!: boolean;

  @OneToMany_(() => Nft, (e) => e.collection)
  nfts!: Nft[];

  @Column_('text', { array: true, nullable: false })
  tags!: string[];
}
