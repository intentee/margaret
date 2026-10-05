use crate::authorization_codes_table::AUTHORIZATION_CODES_TABLE;
use crate::refresh_families_table::REFRESH_FAMILIES_TABLE;
use crate::refresh_tokens_table::REFRESH_TOKENS_TABLE;

pub(crate) fn spend_code_statement() -> String {
    let codes = AUTHORIZATION_CODES_TABLE;
    let families = REFRESH_FAMILIES_TABLE;
    let tokens = REFRESH_TOKENS_TABLE;

    format!(
        r#"WITH locked AS (
    SELECT "digest", "redeemed_family"
    FROM "{codes}"
    WHERE "digest" = $1 AND "expires_at" > now()
    FOR UPDATE
),
spent AS (
    UPDATE "{codes}" AS codes SET "redeemed_family" = $2
    FROM locked
    WHERE codes."digest" = locked."digest" AND locked."redeemed_family" IS NULL
    RETURNING codes."grant_document"
),
opened AS (
    INSERT INTO "{families}" ("id", "grant_document", "expires_at")
    SELECT $2, "grant_document", now() + make_interval(secs => $4)
    FROM spent
    WHERE $3::bytea IS NOT NULL
    RETURNING "id", "expires_at"
),
issued AS (
    INSERT INTO "{tokens}" ("digest", "family", "superseded", "expires_at")
    SELECT $3, "id", FALSE, "expires_at" FROM opened
)
SELECT
    COALESCE("redeemed_family", $2) AS "family",
    "redeemed_family" IS NOT NULL AS "replayed"
FROM locked"#
    )
}
