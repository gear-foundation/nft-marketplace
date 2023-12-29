module.exports = class Data1703490897133 {
    name = 'Data1703490897133'

    async up(db) {
        await db.query(`CREATE TABLE "collection_type" ("id" character varying NOT NULL, "description" text NOT NULL, "type" text NOT NULL, "meta_url" text NOT NULL, "meta_str" text NOT NULL, "marketplace_id" character varying, CONSTRAINT "PK_75c673f46e25c52205fae22130c" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_be423ee528eb245b57165bd597" ON "collection_type" ("marketplace_id") `)
        await db.query(`CREATE TABLE "transfer" ("id" character varying NOT NULL, "from" text NOT NULL, "to" text NOT NULL, "timestamp" TIMESTAMP WITH TIME ZONE NOT NULL, "block_number" text NOT NULL, "tx_hash" text NOT NULL, "nft_id" character varying, CONSTRAINT "PK_fd9ddbdd49a17afcbe014401295" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_52fc453ba5ff660f331db4b359" ON "transfer" ("nft_id") `)
        await db.query(`CREATE TABLE "sale" ("id" character varying NOT NULL, "owner" text NOT NULL, "price" numeric NOT NULL, "timestamp" TIMESTAMP WITH TIME ZONE NOT NULL, "block_number" text NOT NULL, "is_sold" boolean NOT NULL, "nft_id" character varying, CONSTRAINT "PK_d03891c457cbcd22974732b5de2" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_8524438f82167bcb795bcb8663" ON "sale" ("nft_id") `)
        await db.query(`CREATE TABLE "nft" ("id" character varying NOT NULL, "owner" text NOT NULL, "id_in_collection" text NOT NULL, "media_url" text NOT NULL, "minted_at" TIMESTAMP WITH TIME ZONE NOT NULL, "approved_account" text, "on_sale" boolean NOT NULL, "collection_id" character varying, CONSTRAINT "PK_8f46897c58e23b0e7bf6c8e56b0" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_ffe58aa05707db77c2f20ecdbc" ON "nft" ("collection_id") `)
        await db.query(`CREATE TABLE "collection" ("id" character varying NOT NULL, "admin" text NOT NULL, "name" text NOT NULL, "description" text NOT NULL, "user_mint_limit" numeric, "tokens_limit" numeric, "collection_image" text NOT NULL, "transferable" boolean NOT NULL, "approvable" boolean NOT NULL, "burnable" boolean NOT NULL, "sellable" boolean NOT NULL, "attandable" boolean NOT NULL, "tags" text array NOT NULL, "marketplace_id" character varying, "type_id" character varying, CONSTRAINT "PK_ad3f485bbc99d875491f44d7c85" PRIMARY KEY ("id"))`)
        await db.query(`CREATE INDEX "IDX_4698f01445e4833a95f214f862" ON "collection" ("marketplace_id") `)
        await db.query(`CREATE INDEX "IDX_75c673f46e25c52205fae22130" ON "collection" ("type_id") `)
        await db.query(`CREATE TABLE "marketplace" ("id" character varying NOT NULL, CONSTRAINT "PK_d9c9a956a1a45b27b56db53bfc8" PRIMARY KEY ("id"))`)
        await db.query(`ALTER TABLE "collection_type" ADD CONSTRAINT "FK_be423ee528eb245b57165bd597b" FOREIGN KEY ("marketplace_id") REFERENCES "marketplace"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "transfer" ADD CONSTRAINT "FK_52fc453ba5ff660f331db4b3591" FOREIGN KEY ("nft_id") REFERENCES "nft"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "sale" ADD CONSTRAINT "FK_8524438f82167bcb795bcb86637" FOREIGN KEY ("nft_id") REFERENCES "nft"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "nft" ADD CONSTRAINT "FK_ffe58aa05707db77c2f20ecdbc3" FOREIGN KEY ("collection_id") REFERENCES "collection"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "collection" ADD CONSTRAINT "FK_4698f01445e4833a95f214f862b" FOREIGN KEY ("marketplace_id") REFERENCES "marketplace"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
        await db.query(`ALTER TABLE "collection" ADD CONSTRAINT "FK_75c673f46e25c52205fae22130c" FOREIGN KEY ("type_id") REFERENCES "collection_type"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
    }

    async down(db) {
        await db.query(`DROP TABLE "collection_type"`)
        await db.query(`DROP INDEX "public"."IDX_be423ee528eb245b57165bd597"`)
        await db.query(`DROP TABLE "transfer"`)
        await db.query(`DROP INDEX "public"."IDX_52fc453ba5ff660f331db4b359"`)
        await db.query(`DROP TABLE "sale"`)
        await db.query(`DROP INDEX "public"."IDX_8524438f82167bcb795bcb8663"`)
        await db.query(`DROP TABLE "nft"`)
        await db.query(`DROP INDEX "public"."IDX_ffe58aa05707db77c2f20ecdbc"`)
        await db.query(`DROP TABLE "collection"`)
        await db.query(`DROP INDEX "public"."IDX_4698f01445e4833a95f214f862"`)
        await db.query(`DROP INDEX "public"."IDX_75c673f46e25c52205fae22130"`)
        await db.query(`DROP TABLE "marketplace"`)
        await db.query(`ALTER TABLE "collection_type" DROP CONSTRAINT "FK_be423ee528eb245b57165bd597b"`)
        await db.query(`ALTER TABLE "transfer" DROP CONSTRAINT "FK_52fc453ba5ff660f331db4b3591"`)
        await db.query(`ALTER TABLE "sale" DROP CONSTRAINT "FK_8524438f82167bcb795bcb86637"`)
        await db.query(`ALTER TABLE "nft" DROP CONSTRAINT "FK_ffe58aa05707db77c2f20ecdbc3"`)
        await db.query(`ALTER TABLE "collection" DROP CONSTRAINT "FK_4698f01445e4833a95f214f862b"`)
        await db.query(`ALTER TABLE "collection" DROP CONSTRAINT "FK_75c673f46e25c52205fae22130c"`)
    }
}
