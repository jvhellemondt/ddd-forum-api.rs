-- Your SQL goes here

CREATE TABLE "members"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"user_id" INT4 NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

