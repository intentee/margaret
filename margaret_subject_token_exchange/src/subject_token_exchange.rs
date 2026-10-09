use std::collections::BTreeSet;

use uuid::Uuid;

use margaret_oauth_vocabulary::scope::Scope;

pub enum SubjectTokenExchange {
    Granted {
        scopes: BTreeSet<Scope>,
        subject: Uuid,
    },
    Refused,
}
