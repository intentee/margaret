use std::collections::BTreeSet;

use esbuild_metafile::asset::Asset;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::output_lookup::OutputLookup;
use esbuild_metafile::output_properties::OutputProperties;
use proc_macro2::TokenStream;
use quote::quote;

use crate::asset_bag_codegen_error::AssetBagCodegenError;
use crate::entrypoint_output::EntrypointOutput;
use crate::href_tokens::href_tokens;
use crate::preload_tokens::preload_tokens;

pub(crate) fn bundle_tokens(
    metafile: &EsbuildMetafile,
    EntrypointOutput {
        css_bundle,
        main_output,
    }: &EntrypointOutput,
) -> Result<TokenStream, AssetBagCodegenError> {
    let OutputLookup::Found(OutputProperties { preloads }) = metafile.output(main_output) else {
        return Err(AssetBagCodegenError::EntrypointOutputMissing {
            output: main_output.clone(),
        });
    };

    let preload_paths: BTreeSet<String> = preloads.into_iter().collect();
    let preload_list: Vec<TokenStream> = preload_paths
        .iter()
        .map(|preload| preload_tokens(preload))
        .collect();
    let main_href = href_tokens(main_output);

    match Asset::from_path(main_output.clone()) {
        Asset::Script(_) => match css_bundle {
            Some(css_bundle) => {
                let stylesheet_href = href_tokens(css_bundle);

                Ok(quote! {
                    ::margaret_asset_bag::script_stylesheet_bundle::ScriptStylesheetBundle::new(
                        #main_href,
                        #stylesheet_href,
                        &[#(#preload_list),*],
                    )
                })
            }
            None => Ok(quote! {
                ::margaret_asset_bag::script_bundle::ScriptBundle::new(
                    #main_href,
                    &[#(#preload_list),*],
                )
            }),
        },
        Asset::Stylesheet(_) => Ok(quote! {
            ::margaret_asset_bag::stylesheet_bundle::StylesheetBundle::new(
                #main_href,
                &[#(#preload_list),*],
            )
        }),
        Asset::Unknown(_) => Err(AssetBagCodegenError::UnsupportedIncludeOutput {
            output: main_output.clone(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
    use quote::quote;

    use super::bundle_tokens;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;
    use crate::entrypoint_output::EntrypointOutput;

    fn metafile(json: &str) -> EsbuildMetafile {
        EsbuildMetafile::from_str(json).expect("the fixture metafile parses")
    }

    #[test]
    fn builds_a_script_stylesheet_bundle_with_transitive_preloads() {
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

        let handle = bundle_tokens(
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
                ::margaret_asset_bag::script_stylesheet_bundle::ScriptStylesheetBundle::new(
                    ::margaret_asset_bag::asset_href::AssetHref::Local("assets/app_ABC.js"),
                    ::margaret_asset_bag::asset_href::AssetHref::Local("assets/app_ABC.css"),
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
    fn builds_a_script_bundle_without_a_css_bundle_or_preloads() {
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

        let handle = bundle_tokens(
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
                ::margaret_asset_bag::script_bundle::ScriptBundle::new(
                    ::margaret_asset_bag::asset_href::AssetHref::Local("assets/solo_ABC.js"),
                    &[],
                )
            }
            .to_string()
        );
    }

    #[test]
    fn builds_a_stylesheet_bundle_for_a_pure_css_entry_point() {
        let metafile = metafile(
            r#"{
                "outputs": {
                    "assets/theme_ABC.css": {
                        "imports": [],
                        "entryPoint": "src/theme.css"
                    }
                }
            }"#,
        );

        let handle = bundle_tokens(
            &metafile,
            &EntrypointOutput {
                css_bundle: None,
                main_output: "assets/theme_ABC.css".to_string(),
            },
        )
        .expect("the bundle handle is built");

        assert_eq!(
            handle.to_string(),
            quote! {
                ::margaret_asset_bag::stylesheet_bundle::StylesheetBundle::new(
                    ::margaret_asset_bag::asset_href::AssetHref::Local("assets/theme_ABC.css"),
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
            bundle_tokens(
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
            bundle_tokens(
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
