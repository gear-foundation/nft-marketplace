import {
  Entity as Entity_,
  Column as Column_,
  PrimaryColumn as PrimaryColumn_,
  ManyToOne as ManyToOne_,
  Index as Index_,
} from 'typeorm';
import { Marketplace } from './marketplace.model';

@Entity_()
export class CollectionType {
  constructor(props?: Partial<CollectionType>) {
    Object.assign(this, props);
  }

  @PrimaryColumn_()
  id!: string;

  @Column_('text', { nullable: false })
  description!: string;

  @Column_('text', { nullable: false })
  type!: string;

  @Column_('text', { nullable: false })
  metaUrl!: string;

  @Column_('text', { nullable: false })
  metaStr!: string;

  @Index_()
  @ManyToOne_(() => Marketplace, { nullable: true })
  marketplace!: Marketplace;
}
