use crate::pending_authorizations_table::PENDING_AUTHORIZATIONS_TABLE;

pub(crate) fn hold_pending_authorization_statement() -> String {
    let pending = PENDING_AUTHORIZATIONS_TABLE;

    format!(
        r#"WITH purged AS (
    DELETE FROM "{pending}" WHERE "expires_at" <= now()
)
INSERT INTO "{pending}" ("id", "pending_document", "expires_at")
VALUES ($1, $2::text, now() + make_interval(secs => $3))"#
    )
}
