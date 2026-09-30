use crate::refresh_families_table::REFRESH_FAMILIES_TABLE;
use crate::refresh_tokens_table::REFRESH_TOKENS_TABLE;

pub(crate) fn revoke_refresh_token_statement() -> String {
    let families = REFRESH_FAMILIES_TABLE;
    let tokens = REFRESH_TOKENS_TABLE;

    format!(
        r#"WITH presented AS (
    SELECT
        tokens."family",
        (families."grant_document"::jsonb ->> 'client_id') = $2 AS "client_matches"
    FROM "{tokens}" AS tokens
    JOIN "{families}" AS families
        ON families."id" = tokens."family" AND families."expires_at" > now()
    WHERE tokens."digest" = $1 AND NOT tokens."superseded" AND tokens."expires_at" > now()
),
revoked AS (
    DELETE FROM "{families}" AS families
    USING presented
    WHERE families."id" = presented."family" AND presented."client_matches"
)
SELECT "client_matches" FROM presented"#
    )
}
