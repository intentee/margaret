use proc_macro2::TokenStream;

use margaret_attributes::canonical_path::CanonicalPath;

pub struct ProvidedSingleton {
    pub construction: TokenStream,
    pub path: CanonicalPath,
}
