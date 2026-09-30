use crate::authorization_codes_table::AUTHORIZATION_CODES_TABLE;
use crate::refresh_families_table::REFRESH_FAMILIES_TABLE;
use crate::refresh_tokens_table::REFRESH_TOKENS_TABLE;

pub(crate) fn redeem_code_statement() -> String {
    let codes = AUTHORIZATION_CODES_TABLE;
    let families = REFRESH_FAMILIES_TABLE;
    let tokens = REFRESH_TOKENS_TABLE;

    format!(
        r#"WITH presented AS (
    SELECT
        "digest",
        "grant_document",
        "redeemed_family",
        "redeemed_family" IS NULL
            AND ("grant_document"::jsonb ->> 'client_id') = $2
            AND ("grant_document"::jsonb ->> 'redirect_uri') = $3
            AND ("grant_document"::jsonb ->> 'code_challenge') = $4 AS "admitted"
    FROM "{codes}"
    WHERE "digest" = $1 AND "expires_at" > now()
    FOR UPDATE
),
burned AS (
    UPDATE "{codes}" AS codes SET "redeemed_family" = $5
    FROM presented
    WHERE codes."digest" = presented."digest" AND presented."redeemed_family" IS NULL
),
revoked AS (
    DELETE FROM "{families}" AS families
    USING presented
    WHERE families."id" = presented."redeemed_family"
),
opened AS (
    INSERT INTO "{families}" ("id", "grant_document", "expires_at")
    SELECT $5, "grant_document", now() + make_interval(secs => $7)
    FROM presented
    WHERE "admitted" AND $6::bytea IS NOT NULL
    RETURNING "id", "expires_at"
),
issued AS (
    INSERT INTO "{tokens}" ("digest", "family", "superseded", "expires_at")
    SELECT $6, "id", FALSE, "expires_at" FROM opened
)
SELECT "admitted", "grant_document", "redeemed_family" IS NOT NULL AS "replayed" FROM presented"#
    )
}
