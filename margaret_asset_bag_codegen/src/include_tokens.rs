use esbuild_metafile::asset::Asset;
use proc_macro2::TokenStream;
use quote::quote;

use crate::asset_bag_codegen_error::AssetBagCodegenError;
use crate::href_tokens::href_tokens;

pub(crate) fn include_tokens(output_path: &str) -> Result<TokenStream, AssetBagCodegenError> {
    let href = href_tokens(output_path);

    match Asset::from_path(output_path.to_string()) {
        Asset::Script(_) => Ok(quote! {
            ::margaret_asset_bag::include::Include::Script(#href)
        }),
        Asset::Stylesheet(_) => Ok(quote! {
            ::margaret_asset_bag::include::Include::Stylesheet(#href)
        }),
        Asset::Unknown(_) => Err(AssetBagCodegenError::UnsupportedIncludeOutput {
            output: output_path.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::include_tokens;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;

    #[test]
    fn classifies_a_javascript_output_as_a_script_include() {
        assert_eq!(
            include_tokens("assets/app_ABC12345.js")
                .expect("a script include is produced")
                .to_string(),
            quote! {
                ::margaret_asset_bag::include::Include::Script(
                    ::margaret_asset_bag::asset_href::AssetHref::Local("assets/app_ABC12345.js")
                )
            }
            .to_string()
        );
    }

    #[test]
    fn classifies_a_stylesheet_output_as_a_stylesheet_include() {
        assert_eq!(
            include_tokens("assets/app_ABC12345.css")
                .expect("a stylesheet include is produced")
                .to_string(),
            quote! {
                ::margaret_asset_bag::include::Include::Stylesheet(
                    ::margaret_asset_bag::asset_href::AssetHref::Local("assets/app_ABC12345.css")
                )
            }
            .to_string()
        );
    }

    #[test]
    fn rejects_an_entry_point_output_that_is_neither_script_nor_stylesheet() {
        assert!(matches!(
            include_tokens("assets/model_ABC12345.wasm"),
            Err(AssetBagCodegenError::UnsupportedIncludeOutput { output }) if output == "assets/model_ABC12345.wasm"
        ));
    }
}
