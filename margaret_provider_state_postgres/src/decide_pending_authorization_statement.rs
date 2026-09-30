use crate::authorization_codes_table::AUTHORIZATION_CODES_TABLE;
use crate::pending_authorizations_table::PENDING_AUTHORIZATIONS_TABLE;

pub(crate) fn decide_pending_authorization_statement() -> String {
    let codes = AUTHORIZATION_CODES_TABLE;
    let pending = PENDING_AUTHORIZATIONS_TABLE;

    format!(
        r#"WITH taken AS (
    DELETE FROM "{pending}"
    WHERE "id" = $1 AND "expires_at" > now()
    RETURNING
        "pending_document",
        ("pending_document"::jsonb -> 'grant' ->> 'subject') = $2 AS "subject_matches"
),
issued AS (
    INSERT INTO "{codes}" ("digest", "grant_document", "redeemed_family", "expires_at")
    SELECT $3, ("pending_document"::jsonb -> 'grant')::text, NULL, now() + make_interval(secs => $4)
    FROM taken
    WHERE "subject_matches" AND $3::bytea IS NOT NULL
)
SELECT "pending_document", "subject_matches" FROM taken"#
    )
}
