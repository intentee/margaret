use margaret_oauth_vocabulary::scope::Scope;

pub enum DeclaredSignIn {
    Declared { scopes: Vec<Scope> },
    Undeclared,
}
