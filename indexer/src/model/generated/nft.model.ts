import {
  Entity as Entity_,
  Column as Column_,
  PrimaryColumn as PrimaryColumn_,
  ManyToOne as ManyToOne_,
  Index as Index_,
  OneToMany as OneToMany_,
} from 'typeorm';
import { Collection } from './collection.model';
import { Transfer } from './transfer.model';
import { Sale } from './sale.model';

@Entity_()
export class Nft {
  constructor(props?: Partial<Nft>) {
    Object.assign(this, props);
  }

  @PrimaryColumn_()
  id!: string;

  @Column_('text', { nullable: false })
  owner!: string;

  @Column_('text', { nullable: false })
  name!: string;

  @Column_('text', { nullable: false })
  description!: string;

  @Column_('int', { nullable: false })
  idInCollection!: number;

  @Index_()
  @ManyToOne_(() => Collection, { nullable: true })
  collection!: Collection;

  @Column_('text', { nullable: false })
  mediaUrl!: string;

  @Column_('timestamp with time zone', { nullable: false })
  mintedAt!: Date;

  @Column_('text', { nullable: true })
  approvedAccount!: string | undefined | null;

  @Column_('bool', { nullable: false })
  onSale!: boolean;

  @OneToMany_(() => Transfer, (e) => e.nft)
  transfers!: Transfer[];

  @OneToMany_(() => Sale, (e) => e.nft)
  sales!: Sale[];
}
