use margaret_attributes::canonical_path::CanonicalPath;
use margaret_oauth_vocabulary::scope::Scope;

use crate::sign_in_callback::SignInCallback;

pub struct ServedSignIn<'declarations> {
    pub admission: CanonicalPath,
    pub callback: SignInCallback,
    pub scopes: &'declarations [Scope],
    pub start: CanonicalPath,
}
