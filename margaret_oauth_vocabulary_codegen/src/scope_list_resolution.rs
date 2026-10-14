use syn::Path;

use margaret_oauth_vocabulary::scope::Scope;

pub enum ScopeListResolution<'written> {
    Declared(Vec<Scope>),
    Repeated(&'written Path),
    Unknown(&'written Path),
}
