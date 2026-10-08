use margaret_oauth_vocabulary::scope::Scope;

pub enum ModuleSignIn<'declarations> {
    Available { scopes: &'declarations [Scope] },
    Unavailable,
}
