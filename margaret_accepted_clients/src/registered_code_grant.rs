use std::sync::Arc;

use margaret_database::database::Database;

use crate::code_grant_policy::CodeGrantPolicy;

pub enum RegisteredCodeGrant {
    Granted {
        database: Arc<Database>,
        policy: CodeGrantPolicy,
        redirect_uris: Vec<String>,
    },
    Withheld,
}
