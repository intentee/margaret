use margaret_attributes::canonical_path::CanonicalPath;
use margaret_oauth_vocabulary::scope::Scope;

pub enum ModuleSignIn<'declarations> {
    Available {
        admission: &'declarations CanonicalPath,
        scopes: &'declarations [Scope],
    },
    Unavailable,
}
