use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum AccessPolicyBinding {
    Public,
    Singleton { field: Ident, path: CanonicalPath },
}
