use std::collections::BTreeSet;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::output_lookup::OutputLookup;
use esbuild_metafile::output_properties::OutputProperties;
use proc_macro2::TokenStream;
use quote::quote;

use crate::asset_bag_codegen_error::AssetBagCodegenError;
use crate::entrypoint_output::EntrypointOutput;
use crate::include_tokens::include_tokens;
use crate::preload_tokens::preload_tokens;

pub(crate) fn bundle_handle(
    metafile: &EsbuildMetafile,
    EntrypointOutput {
        css_bundle,
        main_output,
    }: &EntrypointOutput,
) -> Result<TokenStream, AssetBagCodegenError> {
    let mut bundle_outputs: Vec<&str> = vec![main_output.as_str()];

    if let Some(css_bundle) = css_bundle {
        bundle_outputs.push(css_bundle.as_str());
    }

    let mut includes: Vec<TokenStream> = Vec::new();
    let mut output_preloads: Vec<TokenStream> = Vec::new();

    for output in &bundle_outputs {
        includes.push(include_tokens(output)?);
        output_preloads.push(preload_tokens(output));
    }

    let OutputLookup::Found(OutputProperties { preloads }) = metafile.output(main_output) else {
        return Err(AssetBagCodegenError::EntrypointOutputMissing {
            output: main_output.clone(),
        });
    };

    let mut preload_paths: BTreeSet<String> = BTreeSet::new();

    for preload in preloads {
        preload_paths.insert(preload);
    }

    let preloads: Vec<TokenStream> = preload_paths
        .iter()
        .map(|preload| preload_tokens(preload))
        .collect();

    Ok(quote! {
        ::margaret_asset_bag::bundle_asset::BundleAsset::new(
            &[#(#includes),*],
            &[#(#output_preloads),*],
            &[#(#preloads),*],
        )
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
    use quote::quote;

    use super::bundle_handle;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;
    use crate::entrypoint_output::EntrypointOutput;

    fn metafile(json: &str) -> EsbuildMetafile {
        EsbuildMetafile::from_str(json).expect("the fixture metafile parses")
    }

    #[test]
    fn builds_a_bundle_with_includes_output_preloads_and_transitive_preloads() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/app_ABC.js": {
                        "imports": [
                            { "path": "assets/chunk_ABC.js" },
                            { "path": "https://fonts.example/font.woff2" }
                        ],
                        "cssBundle": "assets/app_ABC.css",
                        "entryPoint": "src/app.ts"
                    },
                    "assets/app_ABC.css": { "imports": [] },
                    "assets/chunk_ABC.js": { "imports": [] }
                }
            }"#,
        );

        let handle = bundle_handle(
            &metafile,
            &EntrypointOutput {
                css_bundle: Some("assets/app_ABC.css".to_string()),
                main_output: "assets/app_ABC.js".to_string(),
            },
        )
        .expect("the bundle handle is built");

        assert_eq!(
            handle.to_string(),
            quote! {
                ::margaret_asset_bag::bundle_asset::BundleAsset::new(
                    &[
                        ::margaret_asset_bag::include::Include::Script(
                            ::margaret_asset_bag::asset_href::AssetHref::Local("assets/app_ABC.js")
                        ),
                        ::margaret_asset_bag::include::Include::Stylesheet(
                            ::margaret_asset_bag::asset_href::AssetHref::Local("assets/app_ABC.css")
                        )
                    ],
                    &[
                        ::margaret_asset_bag::preload::Preload::Module(
                            ::margaret_asset_bag::asset_href::AssetHref::Local("assets/app_ABC.js")
                        ),
                        ::margaret_asset_bag::preload::Preload::Style(
                            ::margaret_asset_bag::asset_href::AssetHref::Local("assets/app_ABC.css")
                        )
                    ],
                    &[
                        ::margaret_asset_bag::preload::Preload::Module(
                            ::margaret_asset_bag::asset_href::AssetHref::Local("assets/chunk_ABC.js")
                        ),
                        ::margaret_asset_bag::preload::Preload::Font(
                            ::margaret_asset_bag::asset_href::AssetHref::Absolute("https://fonts.example/font.woff2")
                        )
                    ],
                )
            }
            .to_string()
        );
    }

    #[test]
    fn builds_a_bundle_without_a_css_bundle_or_preloads() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/solo_ABC.js": {
                        "imports": [],
                        "entryPoint": "src/solo.ts"
                    }
                }
            }"#,
        );

        let handle = bundle_handle(
            &metafile,
            &EntrypointOutput {
                css_bundle: None,
                main_output: "assets/solo_ABC.js".to_string(),
            },
        )
        .expect("the bundle handle is built");

        assert_eq!(
            handle.to_string(),
            quote! {
                ::margaret_asset_bag::bundle_asset::BundleAsset::new(
                    &[
                        ::margaret_asset_bag::include::Include::Script(
                            ::margaret_asset_bag::asset_href::AssetHref::Local("assets/solo_ABC.js")
                        )
                    ],
                    &[
                        ::margaret_asset_bag::preload::Preload::Module(
                            ::margaret_asset_bag::asset_href::AssetHref::Local("assets/solo_ABC.js")
                        )
                    ],
                    &[],
                )
            }
            .to_string()
        );
    }

    #[test]
    fn rejects_an_entry_point_whose_main_output_is_not_a_script_or_stylesheet() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/mod_ABC.wasm": {
                        "imports": [],
                        "entryPoint": "src/mod.ts"
                    }
                }
            }"#,
        );

        assert!(matches!(
            bundle_handle(
                &metafile,
                &EntrypointOutput {
                    css_bundle: None,
                    main_output: "assets/mod_ABC.wasm".to_string(),
                },
            ),
            Err(AssetBagCodegenError::UnsupportedIncludeOutput { output }) if output == "assets/mod_ABC.wasm"
        ));
    }

    #[test]
    fn rejects_a_main_output_that_is_absent_from_the_metafile() {
        let metafile = metafile(r#"{ "outputs": { "assets/real_ABC.js": { "imports": [] } } }"#);

        assert!(matches!(
            bundle_handle(
                &metafile,
                &EntrypointOutput {
                    css_bundle: None,
                    main_output: "assets/ghost_ABC.js".to_string(),
                },
            ),
            Err(AssetBagCodegenError::EntrypointOutputMissing { output }) if output == "assets/ghost_ABC.js"
        ));
    }
}
