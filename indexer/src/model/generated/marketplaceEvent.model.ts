import {
  Entity as Entity_,
  Column as Column_,
  PrimaryColumn as PrimaryColumn_,
  ManyToOne as ManyToOne_,
  Index as Index_,
} from 'typeorm';
import { Marketplace } from './marketplace.model';

@Entity_()
export class MarketplaceEvent {
  constructor(props?: Partial<MarketplaceEvent>) {
    Object.assign(this, props);
  }

  @PrimaryColumn_()
  id!: string;

  @Column_('text', { nullable: false })
  type!: string;

  @Column_('text', { nullable: false })
  raw!: string;

  @Index_()
  @ManyToOne_(() => Marketplace, { nullable: true })
  marketplace!: Marketplace;

  @Column_('int4', { nullable: false })
  blockNumber!: number;

  @Column_('text', { nullable: false })
  txHash!: string;

  @Column_('timestamp with time zone', { nullable: false })
  timestamp!: Date;
}
