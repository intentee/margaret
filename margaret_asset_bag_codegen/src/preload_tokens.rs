use esbuild_metafile::preloadable_asset::PreloadableAsset;
use proc_macro2::TokenStream;
use quote::quote;

use crate::href_tokens::href_tokens;

pub(crate) fn preload_tokens(output_path: &str) -> TokenStream {
    let href = href_tokens(output_path);

    match PreloadableAsset::from_path(output_path.to_string()) {
        PreloadableAsset::Fetch(_) => quote! {
            ::margaret::framework::asset_bag::preload::Preload::Fetch(#href)
        },
        PreloadableAsset::Font(_) => quote! {
            ::margaret::framework::asset_bag::preload::Preload::Font(#href)
        },
        PreloadableAsset::Image(_) => quote! {
            ::margaret::framework::asset_bag::preload::Preload::Image(#href)
        },
        PreloadableAsset::Module(_) => quote! {
            ::margaret::framework::asset_bag::preload::Preload::Module(#href)
        },
        PreloadableAsset::Stylesheet(_) => quote! {
            ::margaret::framework::asset_bag::preload::Preload::Style(#href)
        },
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::preload_tokens;

    fn expected(variant: &str, href: &str) -> String {
        let variant = quote::format_ident!("{variant}");

        quote! {
            ::margaret::framework::asset_bag::preload::Preload::#variant(
                ::margaret::framework::asset_bag::asset_href::AssetHref::Local(#href)
            )
        }
        .to_string()
    }

    #[test]
    fn classifies_a_module_preload() {
        assert_eq!(
            preload_tokens("assets/chunk_ABC12345.js").to_string(),
            expected("Module", "assets/chunk_ABC12345.js")
        );
    }

    #[test]
    fn classifies_a_style_preload() {
        assert_eq!(
            preload_tokens("assets/page_ABC12345.css").to_string(),
            expected("Style", "assets/page_ABC12345.css")
        );
    }

    #[test]
    fn classifies_a_font_preload() {
        assert_eq!(
            preload_tokens("assets/inter_ABC12345.woff2").to_string(),
            expected("Font", "assets/inter_ABC12345.woff2")
        );
    }

    #[test]
    fn classifies_an_image_preload() {
        assert_eq!(
            preload_tokens("assets/logo_ABC12345.png").to_string(),
            expected("Image", "assets/logo_ABC12345.png")
        );
    }

    #[test]
    fn classifies_an_unknown_extension_as_a_fetch_preload() {
        assert_eq!(
            preload_tokens("assets/data_ABC12345.bin").to_string(),
            expected("Fetch", "assets/data_ABC12345.bin")
        );
    }
}
