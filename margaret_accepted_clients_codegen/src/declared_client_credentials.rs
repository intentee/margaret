use margaret_oauth_vocabulary::resource_scope::ResourceScope;

#[derive(Debug)]
pub enum DeclaredClientCredentials {
    Granted { scopes: Vec<ResourceScope> },
    Withheld,
}
