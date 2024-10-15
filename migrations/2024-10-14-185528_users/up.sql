-- Your SQL goes here
CREATE TABLE "users"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"email" VARCHAR NOT NULL UNIQUE,
	"username" VARCHAR NOT NULL UNIQUE,
	"first_name" VARCHAR,
	"last_name" VARCHAR,
	"password" VARCHAR NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL
);

