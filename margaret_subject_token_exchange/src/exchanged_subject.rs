use std::collections::BTreeSet;

use uuid::Uuid;

use margaret_oauth_vocabulary::scope::Scope;

use crate::subject_token_refusal::SubjectTokenRefusal;

pub enum ExchangedSubject {
    Granted {
        scopes: BTreeSet<Scope>,
        subject: Uuid,
    },
    Refused(SubjectTokenRefusal),
    SigningKeysAwaited,
}
