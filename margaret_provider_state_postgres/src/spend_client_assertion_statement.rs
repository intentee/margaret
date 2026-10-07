use crate::client_assertions_table::CLIENT_ASSERTIONS_TABLE;

pub(crate) fn spend_client_assertion_statement() -> String {
    let assertions = CLIENT_ASSERTIONS_TABLE;

    format!(
        r#"WITH purged AS (
    DELETE FROM "{assertions}"
    WHERE "expires_at" <= to_timestamp($4::bigint::double precision) AND NOT ("client_id" = $1 AND "digest" = $2)
)
INSERT INTO "{assertions}" ("client_id", "digest", "expires_at")
VALUES ($1, $2, to_timestamp($3::bigint::double precision))
ON CONFLICT ("client_id", "digest") DO UPDATE SET "expires_at" = EXCLUDED."expires_at"
WHERE "{assertions}"."expires_at" <= to_timestamp($4::bigint::double precision)
RETURNING TRUE"#
    )
}
