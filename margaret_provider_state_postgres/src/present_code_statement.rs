use crate::authorization_codes_table::AUTHORIZATION_CODES_TABLE;
use crate::refresh_families_table::REFRESH_FAMILIES_TABLE;

pub(crate) fn present_code_statement() -> String {
    let codes = AUTHORIZATION_CODES_TABLE;
    let families = REFRESH_FAMILIES_TABLE;

    format!(
        r#"WITH presented AS (
    SELECT "grant_document", "redeemed_family"
    FROM "{codes}"
    WHERE "digest" = $1 AND "expires_at" > now()
),
revoked AS (
    DELETE FROM "{families}" AS families
    USING presented
    WHERE families."id" = presented."redeemed_family"
)
SELECT "grant_document", "redeemed_family" IS NOT NULL AS "spent" FROM presented"#
    )
}
