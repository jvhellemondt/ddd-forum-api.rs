-- Your SQL goes here

CREATE TABLE "votes"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"post_id" INT4 NOT NULL,
	"member_id" INT4 NOT NULL,
	"vote_type" VARCHAR NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("post_id") REFERENCES "posts"("id"),
	FOREIGN KEY ("member_id") REFERENCES "members"("id")
);

