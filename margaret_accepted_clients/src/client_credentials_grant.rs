use std::collections::BTreeSet;

use margaret_oauth_vocabulary::resource_scope::ResourceScope;

pub enum ClientCredentialsGrant {
    Granted { scopes: BTreeSet<ResourceScope> },
    Withheld,
}
