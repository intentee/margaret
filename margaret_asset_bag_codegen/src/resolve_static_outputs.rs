use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;
use esbuild_metafile::preloadable_asset::PreloadableAsset;
use proc_macro2::TokenStream;
use quote::quote;

use crate::asset_bag_codegen_error::AssetBagCodegenError;
use crate::asset_slot::AssetSlot;
use crate::href_tokens::href_tokens;
use crate::static_output_slots::StaticOutputSlots;

fn static_slot(
    input: &str,
    paths: &[String],
    wrap: impl Fn(&TokenStream) -> TokenStream,
) -> Result<AssetSlot, AssetBagCodegenError> {
    match paths {
        [] => Ok(AssetSlot::Absent),
        [single] => Ok(AssetSlot::Present(wrap(&href_tokens(single)))),
        multiple => Err(AssetBagCodegenError::AmbiguousStaticInput {
            input: input.to_string(),
            output_count: multiple.len(),
        }),
    }
}

pub(crate) fn resolve_static_outputs(
    metafile: &EsbuildMetafile,
    input: &str,
) -> Result<StaticOutputSlots, AssetBagCodegenError> {
    let InputLookup::Found(InputProperties { static_paths, .. }) = metafile.input(input) else {
        return Err(AssetBagCodegenError::InputNotInMetafile {
            input: input.to_string(),
        });
    };

    let mut images: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for path in static_paths {
        match PreloadableAsset::from_path(path.clone()) {
            PreloadableAsset::Image(_) => images.push(path),
            PreloadableAsset::Fetch(_) | PreloadableAsset::Font(_) => files.push(path),
            PreloadableAsset::Module(_) | PreloadableAsset::Stylesheet(_) => {}
        }
    }

    let image = static_slot(input, &images, |href| {
        quote! { ::margaret::framework::asset_bag::image_output::ImageOutput::new(#href) }
    })?;
    let file = static_slot(input, &files, |href| {
        quote! { ::margaret::framework::asset_bag::file_output::FileOutput::new(#href) }
    })?;

    Ok(StaticOutputSlots { image, file })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
    use quote::quote;

    use super::resolve_static_outputs;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;

    fn metafile(json: &str) -> EsbuildMetafile {
        EsbuildMetafile::from_str(json).expect("the fixture metafile parses")
    }

    #[test]
    fn resolves_the_image_output_ignoring_the_file_loader_chunk() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/logo_HASH.png": {
                        "imports": [],
                        "inputs": { "resources/media/logo.png": {} }
                    },
                    "assets/chunk_HASH.js": {
                        "imports": [],
                        "inputs": { "resources/media/logo.png": {} }
                    }
                }
            }"#,
        );

        let slots = resolve_static_outputs(&metafile, "resources/media/logo.png")
            .expect("the static outputs are resolved");

        assert_eq!(
            slots.image.into_tokens().to_string(),
            quote! {
                ::margaret::framework::asset_bag::image_output::ImageOutput::new(
                    ::margaret::framework::asset_bag::asset_href::AssetHref::Local("assets/logo_HASH.png")
                )
            }
            .to_string()
        );
        assert!(!slots.file.is_present());
    }

    #[test]
    fn resolves_a_font_output_as_a_file() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/inter_HASH.woff2": {
                        "imports": [],
                        "inputs": { "resources/fonts/inter.woff2": {} }
                    }
                }
            }"#,
        );

        let slots = resolve_static_outputs(&metafile, "resources/fonts/inter.woff2")
            .expect("the static outputs are resolved");

        assert!(!slots.image.is_present());
        assert_eq!(
            slots.file.into_tokens().to_string(),
            quote! {
                ::margaret::framework::asset_bag::file_output::FileOutput::new(
                    ::margaret::framework::asset_bag::asset_href::AssetHref::Local("assets/inter_HASH.woff2")
                )
            }
            .to_string()
        );
    }

    #[test]
    fn resolves_a_fetch_output_as_a_file() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/model_HASH.glb": {
                        "imports": [],
                        "inputs": { "resources/media/model.glb": {} }
                    }
                }
            }"#,
        );

        let slots = resolve_static_outputs(&metafile, "resources/media/model.glb")
            .expect("the static outputs are resolved");

        assert_eq!(
            slots.file.into_tokens().to_string(),
            quote! {
                ::margaret::framework::asset_bag::file_output::FileOutput::new(
                    ::margaret::framework::asset_bag::asset_href::AssetHref::Local("assets/model_HASH.glb")
                )
            }
            .to_string()
        );
    }

    #[test]
    fn skips_a_stylesheet_output_producing_no_static() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/page_HASH.css": {
                        "imports": [],
                        "inputs": { "resources/css/page.css": {} }
                    }
                }
            }"#,
        );

        let slots = resolve_static_outputs(&metafile, "resources/css/page.css")
            .expect("the static outputs are resolved");

        assert!(!slots.image.is_present());
        assert!(!slots.file.is_present());
    }

    #[test]
    fn rejects_multiple_image_outputs_for_the_same_input() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/a_HASH.svg": {
                        "imports": [],
                        "inputs": { "resources/media/icon.svg": {} }
                    },
                    "assets/b_HASH.svg": {
                        "imports": [],
                        "inputs": { "resources/media/icon.svg": {} }
                    }
                }
            }"#,
        );

        assert!(matches!(
            resolve_static_outputs(&metafile, "resources/media/icon.svg"),
            Err(AssetBagCodegenError::AmbiguousStaticInput { input, output_count })
                if input == "resources/media/icon.svg" && output_count == 2
        ));
    }

    #[test]
    fn rejects_multiple_file_outputs_for_the_same_input() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/a_HASH.woff2": {
                        "imports": [],
                        "inputs": { "resources/fonts/inter.woff2": {} }
                    },
                    "assets/b_HASH.woff2": {
                        "imports": [],
                        "inputs": { "resources/fonts/inter.woff2": {} }
                    }
                }
            }"#,
        );

        assert!(matches!(
            resolve_static_outputs(&metafile, "resources/fonts/inter.woff2"),
            Err(AssetBagCodegenError::AmbiguousStaticInput { input, output_count })
                if input == "resources/fonts/inter.woff2" && output_count == 2
        ));
    }

    #[test]
    fn rejects_an_input_absent_from_the_metafile() {
        let metafile = metafile(r#"{ "outputs": {} }"#);

        assert!(matches!(
            resolve_static_outputs(&metafile, "resources/media/missing.png"),
            Err(AssetBagCodegenError::InputNotInMetafile { input })
                if input == "resources/media/missing.png"
        ));
    }
}
