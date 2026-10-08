use margaret_attributes::canonical_path::CanonicalPath;
use margaret_oauth_vocabulary::scope::Scope;

pub enum DeclaredSignIn {
    Declared {
        redirect_route: CanonicalPath,
        scopes: Vec<Scope>,
    },
    Undeclared,
}
