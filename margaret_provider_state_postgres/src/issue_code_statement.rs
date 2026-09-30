use crate::authorization_codes_table::AUTHORIZATION_CODES_TABLE;
use crate::refresh_families_table::REFRESH_FAMILIES_TABLE;

pub(crate) fn issue_code_statement() -> String {
    let codes = AUTHORIZATION_CODES_TABLE;
    let families = REFRESH_FAMILIES_TABLE;

    format!(
        r#"WITH purged_codes AS (
    DELETE FROM "{codes}" WHERE "expires_at" <= now()
),
purged_families AS (
    DELETE FROM "{families}" WHERE "expires_at" <= now()
)
INSERT INTO "{codes}" ("digest", "grant_document", "redeemed_family", "expires_at")
VALUES ($1, $2::text, NULL, now() + make_interval(secs => $3))"#
    )
}
