use proc_macro2::TokenStream;

pub(crate) struct AssetArm {
    pub(crate) handle: TokenStream,
    pub(crate) input: String,
}
