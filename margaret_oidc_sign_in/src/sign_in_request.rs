use std::collections::BTreeSet;

use url::Url;

use margaret_oauth_vocabulary::scope::Scope;

pub struct SignInRequest {
    pub callback: Url,
    pub scopes: BTreeSet<Scope>,
}
