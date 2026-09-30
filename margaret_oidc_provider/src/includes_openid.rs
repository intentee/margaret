use std::collections::BTreeSet;

use margaret_oauth_vocabulary::scope::Scope;

use crate::openid_scope::OPENID_SCOPE;

pub(crate) fn includes_openid(scopes: &BTreeSet<Scope>) -> bool {
    scopes.iter().any(|scope| scope.as_str() == OPENID_SCOPE)
}
