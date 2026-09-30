use crate::refresh_families_table::REFRESH_FAMILIES_TABLE;
use crate::refresh_tokens_table::REFRESH_TOKENS_TABLE;

pub(crate) fn rotate_refresh_token_statement() -> String {
    let families = REFRESH_FAMILIES_TABLE;
    let tokens = REFRESH_TOKENS_TABLE;

    format!(
        r#"WITH presented AS (
    SELECT
        tokens."digest",
        tokens."family",
        tokens."superseded",
        families."grant_document",
        families."expires_at" AS "family_expires_at"
    FROM "{tokens}" AS tokens
    LEFT JOIN "{families}" AS families
        ON families."id" = tokens."family" AND families."expires_at" > now()
    WHERE tokens."digest" = $1 AND tokens."expires_at" > now()
    FOR UPDATE OF tokens
),
judged AS (
    SELECT
        "digest",
        "family",
        "family_expires_at",
        "grant_document",
        "superseded",
        COALESCE(("grant_document"::jsonb ->> 'client_id') = $3, FALSE) AS "client_matches",
        COALESCE($4::jsonb IS NULL OR ("grant_document"::jsonb -> 'scopes') @> $4::jsonb, FALSE)
            AS "scope_within"
    FROM presented
),
admitted AS (
    SELECT * FROM judged
    WHERE NOT "superseded"
        AND "grant_document" IS NOT NULL
        AND "client_matches"
        AND "scope_within"
),
superseding AS (
    UPDATE "{tokens}" AS tokens SET "superseded" = TRUE
    FROM admitted
    WHERE tokens."digest" = admitted."digest"
),
issuing AS (
    INSERT INTO "{tokens}" ("digest", "family", "superseded", "expires_at")
    SELECT $2, "family", FALSE, "family_expires_at" FROM admitted
),
revoking AS (
    DELETE FROM "{families}" AS families
    USING judged
    WHERE families."id" = judged."family" AND judged."superseded"
)
SELECT "client_matches", "family", "grant_document", "scope_within", "superseded" FROM judged"#
    )
}
