use proc_macro2::TokenStream;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct DeferredForeignKey {
    pub(crate) field_name: String,
    pub(crate) index: bool,
    pub(crate) nullable: bool,
    pub(crate) on_delete: TokenStream,
    pub(crate) rust_type: String,
    pub(crate) target_path: CanonicalPath,
    pub(crate) unique: bool,
}
