import {
  Entity as Entity_,
  Column as Column_,
  PrimaryColumn as PrimaryColumn_,
  ManyToOne as ManyToOne_,
  Index as Index_,
} from 'typeorm';
import { Nft } from './nft.model';

@Entity_()
export class Transfer {
  constructor(props?: Partial<Transfer>) {
    Object.assign(this, props);
  }

  @PrimaryColumn_()
  id!: string;

  @Index_()
  @ManyToOne_(() => Nft, { nullable: true })
  nft!: Nft;

  @Column_('text', { nullable: false })
  from!: string;

  @Column_('text', { nullable: false })
  to!: string;

  @Column_('timestamp with time zone', { nullable: false })
  timestamp!: Date;

  @Column_('text', { nullable: false })
  blockNumber!: string;

  @Column_('text', { nullable: false })
  txHash!: string;
}
