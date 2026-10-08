CREATE SCHEMA "margaret";

CREATE TABLE "margaret"."authorization_codes" (
    "code" BYTEA NOT NULL CONSTRAINT "authorization_codes_code_byte_length" CHECK (length("code") = 32),
    "expires_at" BIGINT NOT NULL,
    "grant" TEXT NOT NULL,
    "redeemed_by" UUID,
    PRIMARY KEY ("code")
);

CREATE INDEX "authorization_codes_expires_at_index" ON "margaret"."authorization_codes" ("expires_at");

CREATE TABLE "margaret"."pending_authorizations" (
    "id" UUID NOT NULL DEFAULT uuidv7(),
    "expires_at" BIGINT NOT NULL,
    "grant" TEXT NOT NULL,
    "state" TEXT,
    PRIMARY KEY ("id")
);

CREATE INDEX "pending_authorizations_expires_at_index" ON "margaret"."pending_authorizations" ("expires_at");

CREATE TABLE "margaret"."refresh_families" (
    "id" UUID NOT NULL DEFAULT uuidv7(),
    "auth_time" TIMESTAMPTZ NOT NULL,
    "client_id" TEXT NOT NULL,
    "expires_at" BIGINT NOT NULL,
    "scopes" TEXT NOT NULL,
    "subject" UUID NOT NULL,
    PRIMARY KEY ("id")
);

CREATE INDEX "refresh_families_expires_at_index" ON "margaret"."refresh_families" ("expires_at");

CREATE TABLE "margaret"."refresh_family_revocations" (
    "family" UUID NOT NULL DEFAULT uuidv7(),
    "expires_at" BIGINT NOT NULL,
    PRIMARY KEY ("family")
);

CREATE INDEX "refresh_family_revocations_expires_at_index" ON "margaret"."refresh_family_revocations" ("expires_at");

CREATE TABLE "margaret"."refresh_tokens" (
    "token" BYTEA NOT NULL CONSTRAINT "refresh_tokens_token_byte_length" CHECK (length("token") = 32),
    "family_id" UUID NOT NULL,
    "current" BOOLEAN NOT NULL,
    PRIMARY KEY ("token"),
    FOREIGN KEY ("family_id") REFERENCES "margaret"."refresh_families" ("id") ON DELETE CASCADE
);

CREATE INDEX "refresh_tokens_family_id_index" ON "margaret"."refresh_tokens" ("family_id");

CREATE TABLE "margaret"."client_assertions" (
    "client_id" TEXT NOT NULL,
    "assertion" BYTEA NOT NULL CONSTRAINT "client_assertions_assertion_byte_length" CHECK (length("assertion") = 32),
    "expires_at" BIGINT NOT NULL,
    PRIMARY KEY ("client_id", "assertion")
);

CREATE INDEX "client_assertions_expires_at_index" ON "margaret"."client_assertions" ("expires_at");

CREATE TABLE "margaret"."signing_key_sets" (
    "name" TEXT NOT NULL,
    "generation" BIGINT NOT NULL,
    "document" TEXT NOT NULL,
    PRIMARY KEY ("name")
);
