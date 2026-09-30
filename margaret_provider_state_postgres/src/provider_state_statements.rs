use std::sync::Arc;

use sqlx::AssertSqlSafe;
use sqlx::SqlSafeStr;
use sqlx::SqlStr;

use crate::decide_pending_authorization_statement::decide_pending_authorization_statement;
use crate::hold_pending_authorization_statement::hold_pending_authorization_statement;
use crate::issue_code_statement::issue_code_statement;
use crate::redeem_code_statement::redeem_code_statement;
use crate::revoke_refresh_token_statement::revoke_refresh_token_statement;
use crate::rotate_refresh_token_statement::rotate_refresh_token_statement;

fn audited(statement: String) -> SqlStr {
    AssertSqlSafe(Arc::<str>::from(statement)).into_sql_str()
}

pub(crate) struct ProviderStateStatements {
    pub(crate) decide_pending_authorization: SqlStr,
    pub(crate) hold_pending_authorization: SqlStr,
    pub(crate) issue_code: SqlStr,
    pub(crate) redeem_code: SqlStr,
    pub(crate) revoke_refresh_token: SqlStr,
    pub(crate) rotate_refresh_token: SqlStr,
}

impl ProviderStateStatements {
    pub(crate) fn prepare() -> Self {
        Self {
            decide_pending_authorization: audited(decide_pending_authorization_statement()),
            hold_pending_authorization: audited(hold_pending_authorization_statement()),
            issue_code: audited(issue_code_statement()),
            redeem_code: audited(redeem_code_statement()),
            revoke_refresh_token: audited(revoke_refresh_token_statement()),
            rotate_refresh_token: audited(rotate_refresh_token_statement()),
        }
    }
}
