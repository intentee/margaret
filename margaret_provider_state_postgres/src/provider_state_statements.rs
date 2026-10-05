use std::sync::Arc;

use sqlx::AssertSqlSafe;
use sqlx::SqlSafeStr;
use sqlx::SqlStr;

use crate::decide_pending_authorization_statement::decide_pending_authorization_statement;
use crate::hold_pending_authorization_statement::hold_pending_authorization_statement;
use crate::issue_code_statement::issue_code_statement;
use crate::present_code_statement::present_code_statement;
use crate::present_refresh_token_statement::present_refresh_token_statement;
use crate::revoke_refresh_family_statement::revoke_refresh_family_statement;
use crate::revoke_refresh_token_statement::revoke_refresh_token_statement;
use crate::rotate_refresh_token_statement::rotate_refresh_token_statement;
use crate::spend_code_statement::spend_code_statement;

fn audited(statement: String) -> SqlStr {
    AssertSqlSafe(Arc::<str>::from(statement)).into_sql_str()
}

pub(crate) struct ProviderStateStatements {
    pub(crate) decide_pending_authorization: SqlStr,
    pub(crate) hold_pending_authorization: SqlStr,
    pub(crate) issue_code: SqlStr,
    pub(crate) present_code: SqlStr,
    pub(crate) present_refresh_token: SqlStr,
    pub(crate) revoke_refresh_family: SqlStr,
    pub(crate) revoke_refresh_token: SqlStr,
    pub(crate) rotate_refresh_token: SqlStr,
    pub(crate) spend_code: SqlStr,
}

impl ProviderStateStatements {
    pub(crate) fn prepare() -> Self {
        Self {
            decide_pending_authorization: audited(decide_pending_authorization_statement()),
            hold_pending_authorization: audited(hold_pending_authorization_statement()),
            issue_code: audited(issue_code_statement()),
            present_code: audited(present_code_statement()),
            present_refresh_token: audited(present_refresh_token_statement()),
            revoke_refresh_family: audited(revoke_refresh_family_statement()),
            revoke_refresh_token: audited(revoke_refresh_token_statement()),
            rotate_refresh_token: audited(rotate_refresh_token_statement()),
            spend_code: audited(spend_code_statement()),
        }
    }
}
