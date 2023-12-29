import {
  Entity as Entity_,
  PrimaryColumn as PrimaryColumn_,
  OneToMany as OneToMany_,
} from 'typeorm';
import { Collection } from './collection.model';
import { CollectionType } from './collectionType.model';

@Entity_()
export class Marketplace {
  constructor(props?: Partial<Marketplace>) {
    Object.assign(this, props);
  }

  @PrimaryColumn_()
  id!: string;

  @OneToMany_(() => Collection, (e) => e.marketplace)
  collections!: Collection[];

  @OneToMany_(() => CollectionType, (e) => e.marketplace)
  collectionTypes!: CollectionType[];
}
