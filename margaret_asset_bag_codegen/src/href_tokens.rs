use proc_macro2::TokenStream;
use quote::quote;
use url::Url;

pub(crate) fn href_tokens(path: &str) -> TokenStream {
    match Url::parse(path) {
        Ok(_absolute) => quote! {
            ::margaret_asset_bag::asset_href::AssetHref::Absolute(#path)
        },
        Err(_relative) => quote! {
            ::margaret_asset_bag::asset_href::AssetHref::Local(#path)
        },
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::href_tokens;

    #[test]
    fn treats_a_relative_output_path_as_a_local_href() {
        assert_eq!(
            href_tokens("assets/app_ABC12345.js").to_string(),
            quote! { ::margaret_asset_bag::asset_href::AssetHref::Local("assets/app_ABC12345.js") }
                .to_string()
        );
    }

    #[test]
    fn treats_an_absolute_url_as_an_absolute_href() {
        assert_eq!(
            href_tokens("https://fonts.example/font.woff2").to_string(),
            quote! {
                ::margaret_asset_bag::asset_href::AssetHref::Absolute("https://fonts.example/font.woff2")
            }
            .to_string()
        );
    }
}
