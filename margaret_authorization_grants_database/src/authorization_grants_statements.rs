use margaret_model::qualified_framework_table::qualified_framework_table;

pub(crate) struct AuthorizationGrantsStatements {
    pub(crate) find_refresh_token: String,
    pub(crate) find_redeemed_code: String,
    pub(crate) find_superseded_refresh_token: String,
    pub(crate) hold_pending_authorization: String,
    pub(crate) issue_code: String,
    pub(crate) open_refresh_family: String,
    pub(crate) redeem_code: String,
    pub(crate) revoke_refresh_family: String,
    pub(crate) rotate_refresh_token: String,
    pub(crate) take_pending_authorization: String,
}

impl AuthorizationGrantsStatements {
    pub(crate) fn new() -> Self {
        let codes = qualified_framework_table("authorization_codes");
        let families = qualified_framework_table("refresh_families");
        let pending = qualified_framework_table("pending_authorizations");
        let revocations = qualified_framework_table("refresh_family_revocations");
        let tokens = qualified_framework_table("refresh_tokens");
        let sweep_families = |now: &str, kept_revocation: &str| {
            format!(
                "swept_tokens AS (DELETE FROM {tokens} WHERE family IN (SELECT family FROM {families} WHERE expires_at <= {now})), \
                 swept_families AS (DELETE FROM {families} WHERE expires_at <= {now}), \
                 swept_revocations AS (DELETE FROM {revocations} WHERE expires_at <= {now}{kept_revocation})"
            )
        };

        Self {
            find_refresh_token: format!(
                "SELECT token.token, token.family, token.current, family.auth_time, family.client_id, family.expires_at, family.scopes, family.subject \
                 FROM {tokens} token JOIN {families} family ON family.family = token.family \
                 WHERE token.token = $1 AND NOT EXISTS (SELECT 1 FROM {revocations} revocation WHERE revocation.family = token.family)"
            ),
            find_redeemed_code: format!(
                "SELECT redeemed_by FROM {codes} WHERE code = $1 AND redeemed_by IS NOT NULL"
            ),
            find_superseded_refresh_token: format!(
                "SELECT token.family FROM {tokens} token \
                 WHERE token.token = $1 AND NOT token.current \
                 AND NOT EXISTS (SELECT 1 FROM {revocations} revocation WHERE revocation.family = token.family)"
            ),
            hold_pending_authorization: format!(
                "WITH swept AS (DELETE FROM {pending} WHERE expires_at <= $5) \
                 INSERT INTO {pending} (id, expires_at, \"grant\", state) VALUES ($1, $2, $3, $4)"
            ),
            issue_code: format!(
                "WITH swept AS (DELETE FROM {codes} WHERE expires_at <= $4) \
                 INSERT INTO {codes} (code, expires_at, \"grant\", redeemed_by) VALUES ($1, $2, $3, NULL)"
            ),
            open_refresh_family: format!(
                "WITH {}, \
                 opened AS (INSERT INTO {families} (family, auth_time, client_id, expires_at, scopes, subject) \
                     SELECT $1, $2, $3, $4, $5, $6 \
                     WHERE NOT EXISTS (SELECT 1 FROM {revocations} WHERE family = $1 AND expires_at > $8) \
                     RETURNING family) \
                 INSERT INTO {tokens} (token, family, current) SELECT $7, family, true FROM opened",
                sweep_families("$8", "")
            ),
            redeem_code: format!(
                "UPDATE {codes} SET redeemed_by = $2 WHERE code = $1 AND redeemed_by IS NULL RETURNING code, expires_at, \"grant\", redeemed_by"
            ),
            revoke_refresh_family: format!(
                "WITH {} \
                 INSERT INTO {revocations} (family, expires_at) VALUES ($1, $2) \
                 ON CONFLICT (family) DO UPDATE SET expires_at = GREATEST({revocations}.expires_at, EXCLUDED.expires_at)",
                sweep_families("$3", " AND family <> $1")
            ),
            rotate_refresh_token: format!(
                "WITH {}, \
                 rotated AS (UPDATE {tokens} token SET current = false \
                     WHERE token.token = $1 AND token.current \
                     AND EXISTS (SELECT 1 FROM {families} family WHERE family.family = token.family AND family.expires_at > $3) \
                     AND NOT EXISTS (SELECT 1 FROM {revocations} revocation WHERE revocation.family = token.family) \
                     RETURNING token.family) \
                 INSERT INTO {tokens} (token, family, current) SELECT $2, family, true FROM rotated",
                sweep_families("$3", "")
            ),
            take_pending_authorization: format!(
                "DELETE FROM {pending} WHERE id = $1 RETURNING id, expires_at, \"grant\", state"
            ),
        }
    }
}
