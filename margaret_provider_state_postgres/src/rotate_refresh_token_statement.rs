use crate::refresh_families_table::REFRESH_FAMILIES_TABLE;
use crate::refresh_tokens_table::REFRESH_TOKENS_TABLE;

pub(crate) fn rotate_refresh_token_statement() -> String {
    let families = REFRESH_FAMILIES_TABLE;
    let tokens = REFRESH_TOKENS_TABLE;

    format!(
        r#"WITH family AS (
    SELECT families."id", families."expires_at"
    FROM "{families}" AS families
    JOIN "{tokens}" AS tokens ON tokens."family" = families."id"
    WHERE tokens."digest" = $1 AND families."expires_at" > now()
    FOR SHARE OF families
),
locked AS (
    SELECT tokens."digest", tokens."superseded", family."id" AS "family", family."expires_at"
    FROM "{tokens}" AS tokens
    JOIN family ON family."id" = tokens."family"
    WHERE tokens."digest" = $1
    FOR UPDATE OF tokens
),
superseding AS (
    UPDATE "{tokens}" AS tokens SET "superseded" = TRUE
    FROM locked
    WHERE tokens."digest" = locked."digest" AND NOT locked."superseded"
),
issued AS (
    INSERT INTO "{tokens}" ("digest", "family", "superseded", "expires_at")
    SELECT $2, "family", FALSE, "expires_at" FROM locked WHERE NOT "superseded"
)
SELECT "family", "superseded" AS "replayed" FROM locked"#
    )
}
