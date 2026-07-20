use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;
use esbuild_metafile::preloadable_asset::PreloadableAsset;
use proc_macro2::TokenStream;
use quote::quote;

use crate::asset_bag_codegen_error::AssetBagCodegenError;
use crate::href_tokens::href_tokens;

pub(crate) fn static_handle(
    metafile: &EsbuildMetafile,
    input: &str,
) -> Result<TokenStream, AssetBagCodegenError> {
    let InputLookup::Found(InputProperties { static_paths, .. }) = metafile.input(input) else {
        return Err(AssetBagCodegenError::InputNotInMetafile {
            input: input.to_string(),
        });
    };

    let [output_path] = static_paths.as_slice() else {
        return Err(AssetBagCodegenError::AmbiguousStaticInput {
            input: input.to_string(),
            output_count: static_paths.len(),
        });
    };

    let href = href_tokens(output_path);

    match PreloadableAsset::from_path(output_path.clone()) {
        PreloadableAsset::Image(_) => Ok(quote! {
            ::margaret_asset_bag::image_asset::ImageAsset::new(#href)
        }),
        PreloadableAsset::Fetch(_)
        | PreloadableAsset::Font(_)
        | PreloadableAsset::Module(_)
        | PreloadableAsset::Stylesheet(_) => Ok(quote! {
            ::margaret_asset_bag::static_asset::StaticAsset::new(#href)
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
    use quote::quote;

    use super::static_handle;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;

    fn metafile(json: &str) -> EsbuildMetafile {
        EsbuildMetafile::from_str(json).expect("the fixture metafile parses")
    }

    #[test]
    fn builds_an_image_handle_for_an_image_static_asset() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/logo_ABC.png": {
                        "imports": [],
                        "inputs": { "media/logo.png": {} }
                    }
                }
            }"#,
        );

        assert_eq!(
            static_handle(&metafile, "media/logo.png")
                .expect("the image handle is built")
                .to_string(),
            quote! {
                ::margaret_asset_bag::image_asset::ImageAsset::new(
                    ::margaret_asset_bag::asset_href::AssetHref::Local("assets/logo_ABC.png")
                )
            }
            .to_string()
        );
    }

    #[test]
    fn builds_a_file_handle_for_a_non_image_static_asset() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/data_ABC.bin": {
                        "imports": [],
                        "inputs": { "media/data.bin": {} }
                    }
                }
            }"#,
        );

        assert_eq!(
            static_handle(&metafile, "media/data.bin")
                .expect("the file handle is built")
                .to_string(),
            quote! {
                ::margaret_asset_bag::static_asset::StaticAsset::new(
                    ::margaret_asset_bag::asset_href::AssetHref::Local("assets/data_ABC.bin")
                )
            }
            .to_string()
        );
    }

    #[test]
    fn rejects_a_static_input_that_is_not_in_the_metafile() {
        let metafile = metafile(r#"{ "outputs": {} }"#);

        assert!(matches!(
            static_handle(&metafile, "media/missing.png"),
            Err(AssetBagCodegenError::InputNotInMetafile { input }) if input == "media/missing.png"
        ));
    }

    #[test]
    fn rejects_a_static_input_that_resolves_to_multiple_outputs() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/a_ABC.png": {
                        "imports": [],
                        "inputs": { "media/logo.png": {} }
                    },
                    "assets/b_DEF.png": {
                        "imports": [],
                        "inputs": { "media/logo.png": {} }
                    }
                }
            }"#,
        );

        assert!(matches!(
            static_handle(&metafile, "media/logo.png"),
            Err(AssetBagCodegenError::AmbiguousStaticInput { input, output_count })
                if input == "media/logo.png" && output_count == 2
        ));
    }
}
