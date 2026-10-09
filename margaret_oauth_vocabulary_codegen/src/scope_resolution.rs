use margaret_oauth_vocabulary::scope::Scope;

#[derive(Debug, Eq, PartialEq)]
pub enum ScopeResolution {
    Declared(Scope),
    Unknown,
}
