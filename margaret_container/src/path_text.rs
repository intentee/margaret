use quote::ToTokens;
use syn::Path;

pub(crate) fn path_text(path: &Path) -> String {
    path.to_token_stream().to_string()
}
