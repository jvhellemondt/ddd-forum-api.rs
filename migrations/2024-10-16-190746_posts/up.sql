-- Your SQL goes here

CREATE TABLE "posts"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"member_id" INT4 NOT NULL,
	"post_type" VARCHAR NOT NULL,
	"title" VARCHAR NOT NULL,
	"content" TEXT NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("member_id") REFERENCES "members"("id")
);

