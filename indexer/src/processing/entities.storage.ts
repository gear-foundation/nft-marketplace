import {
  Collection,
  CollectionType,
  Marketplace,
  Nft,
  Sale,
  Transfer,
} from '../model';
import { Store } from '@subsquid/typeorm-store';

export class EntitiesStorage {
  // typeName -> CollectionType
  private collectionTypes: Record<string, CollectionType> = {};
  // collectionAddress -> Collection
  private collections: Record<string, Collection> = {};
  // // collectionAddress-tokenId -> Nft
  private nfts: Record<string, Nft> = {};
  private transfers: Transfer[] = [];
  private sale: Record<string, Sale> = {};
  private marketplace: Marketplace = new Marketplace();

  constructor(private readonly store: Store) {}

  async saveAll() {
    await Promise.all([
      this.store.save(Object.values(this.collectionTypes)),
      this.store.save(Object.values(this.collections)),
      this.store.save(Object.values(this.nfts)),
      this.store.save(this.transfers),
      this.store.save(Object.values(this.sale)),
    ]);
  }

  async getCollection(collectionAddress: string) {
    if (this.collections[collectionAddress] !== undefined) {
      return this.collections[collectionAddress];
    }
    return this.store.findOne(Collection, { where: { id: collectionAddress } });
  }

  async getNft(collectionAddress: string, tokenId: number) {
    const key = this.getNftKey(collectionAddress, tokenId);
    if (this.nfts[key] !== undefined) {
      return this.nfts[key];
    }
    return this.store.findOne(Nft, {
      where: {
        collection: {
          id: 'collectionAddress',
        },
        idInCollection: tokenId,
      },
    });
  }

  setCollection(collection: Collection) {
    this.collections[collection.id] = collection;
  }

  getSale(nft: Nft) {
    const key = this.getNftKey(nft.collection.id, nft.idInCollection);
    if (this.sale[key] !== undefined) {
      return this.sale[key];
    }
    return this.store.findOne(Sale, {
      where: {
        nft: {
          collection: {
            id: nft.collection.id,
          },
          idInCollection: nft.idInCollection,
        },
        isSold: false,
      },
    });
  }

  setSale(sale: Sale) {
    this.sale[this.getNftKey(sale.nft.collection.id, sale.nft.idInCollection)] =
      sale;
  }

  async deleteSale(sale: Sale) {
    delete this.sale[
      this.getNftKey(sale.nft.collection.id, sale.nft.idInCollection)
    ];
    await this.store.remove(sale);
  }

  async deleteCollection(collection: Collection) {
    delete this.collections[collection.id];
    await this.store.remove(collection);
  }

  async getCollectionType(
    typeName: string,
  ): Promise<CollectionType | undefined> {
    if (this.collectionTypes[typeName] !== undefined) {
      return this.collectionTypes[typeName];
    }
    return this.store.findOne(CollectionType, { where: { type: typeName } });
  }

  setCollectionType(collectionType: CollectionType) {
    this.collectionTypes[collectionType.type] = collectionType;
  }

  setNft(nft: Nft) {
    this.nfts[this.getNftKey(nft.collection.id, nft.idInCollection)] = nft;
  }

  setTransfer(transfer: Transfer) {
    this.transfers.push(transfer);
  }

  private getNftKey(collectionAddress: string, idInCollection: number) {
    return `${collectionAddress}-${idInCollection}`;
  }
}
