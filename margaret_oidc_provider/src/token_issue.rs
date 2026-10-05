use std::collections::BTreeSet;

use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_registered_claims::audience::Audience;

use crate::refresh_token_issue::RefreshTokenIssue;

pub(crate) struct TokenIssue {
    pub(crate) id_token: Option<String>,
    pub(crate) issued_token_type: Option<SubjectTokenType>,
    pub(crate) refresh: RefreshTokenIssue,
    pub(crate) resource: Audience,
    pub(crate) scopes: BTreeSet<Scope>,
    pub(crate) subject: String,
}
