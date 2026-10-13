use margaret_oauth_vocabulary::scope::Scope;

#[derive(Debug)]
pub enum DeclaredTokenExchange {
    Granted { scopes: Vec<Scope> },
    Withheld,
}
