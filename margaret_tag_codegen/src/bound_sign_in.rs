use margaret_attributes::canonical_path::CanonicalPath;
use margaret_oauth_vocabulary::scope::Scope;

#[derive(Debug, Eq, PartialEq)]
pub enum BoundSignIn<'declarations> {
    Available {
        redirect_route: &'declarations CanonicalPath,
        scopes: &'declarations [Scope],
    },
    Unavailable,
}
