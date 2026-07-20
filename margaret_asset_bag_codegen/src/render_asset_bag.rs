use std::collections::BTreeSet;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::raw_esbuild_metafile::RawEsbuildMetafile;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::asset_arm::AssetArm;
use crate::asset_bag_codegen_error::AssetBagCodegenError;
use crate::bundle_handle::bundle_handle;
use crate::enumerate_inputs::enumerate_inputs;
use crate::output_directory::output_directory;
use crate::static_handle::static_handle;

fn module_tokens(directory: &str, arms: &[AssetArm]) -> TokenStream {
    let folder = format!("{directory}/");
    let arm_tokens: Vec<TokenStream> = arms
        .iter()
        .map(|AssetArm { handle, input }| {
            quote! {
                (#input) => { #handle };
            }
        })
        .collect();

    quote! {
        #[derive(::rust_embed::RustEmbed)]
        #[folder = #folder]
        pub struct EmbeddedAssets;

        pub type AssetServer =
            ::margaret_asset_bag_server::asset_server::AssetServer<EmbeddedAssets>;

        macro_rules! asset {
            #(#arm_tokens)*
            ($other:literal) => {
                ::core::compile_error!(::core::concat!("unknown esbuild asset input: ", $other))
            };
        }

        pub(crate) use asset;
    }
}

pub fn render_asset_bag(
    metafile_contents: &str,
) -> Result<Vec<GeneratedModuleTokens>, AssetBagCodegenError> {
    let raw: RawEsbuildMetafile = serde_json::from_str(metafile_contents)?;
    let enumerated = enumerate_inputs(&raw)?;
    let metafile: EsbuildMetafile = raw.into();
    let output_paths: BTreeSet<String> = metafile.get_output_paths().into_iter().collect();
    let directory = output_directory(&output_paths)?;

    let mut arms: Vec<AssetArm> = Vec::new();

    for (input, entrypoint) in &enumerated.entrypoints {
        arms.push(AssetArm {
            handle: bundle_handle(&metafile, entrypoint)?,
            input: input.clone(),
        });
    }

    for input in &enumerated.static_inputs {
        arms.push(AssetArm {
            handle: static_handle(&metafile, input)?,
            input: input.clone(),
        });
    }

    Ok(vec![GeneratedModuleTokens::new(
        "asset_bag",
        module_tokens(&directory, &arms),
    )])
}

#[cfg(test)]
mod tests {
    use super::render_asset_bag;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;

    const FULL_METAFILE: &str = r#"{
        "outputs": {
            "assets/app_ABC.js": {
                "imports": [{ "path": "assets/chunk_ABC.js" }],
                "cssBundle": "assets/app_ABC.css",
                "entryPoint": "src/app.ts"
            },
            "assets/app_ABC.css": { "imports": [] },
            "assets/chunk_ABC.js": { "imports": [] },
            "assets/logo_ABC.png": {
                "imports": [],
                "inputs": { "media/logo.png": {} }
            }
        }
    }"#;

    fn formatted_module(metafile_contents: &str) -> String {
        let mut modules =
            render_asset_bag(metafile_contents).expect("the asset bag module is generated");

        assert_eq!(modules.len(), 1);

        let module = modules.remove(0);

        assert_eq!(module.name(), "asset_bag");

        module
            .format()
            .expect("the generated tokens form a valid Rust file")
            .source()
            .to_string()
    }

    #[test]
    fn generates_a_valid_module_with_an_arm_per_input() {
        let source = formatted_module(FULL_METAFILE);

        assert!(source.contains("#[folder = \"assets/\"]"));
        assert!(source.contains("pub struct EmbeddedAssets"));
        assert!(source.contains("pub type AssetServer"));
        assert!(source.contains("::margaret_asset_bag_server::asset_server::AssetServer"));
        assert!(source.contains("macro_rules! asset"));
        assert!(source.contains("(\"src/app.ts\") =>"));
        assert!(source.contains("(\"media/logo.png\") =>"));
        assert!(source.contains("unknown esbuild asset input: "));
        assert!(source.contains("pub(crate) use asset;"));
    }

    #[test]
    fn rejects_a_metafile_that_is_not_valid_json() {
        assert_eq!(
            render_asset_bag("not json")
                .expect_err("invalid json is rejected")
                .to_string(),
            "failed to parse the esbuild metafile"
        );
    }

    #[test]
    fn rejects_a_metafile_without_outputs() {
        assert_eq!(
            render_asset_bag(r#"{ "outputs": {} }"#)
                .expect_err("a metafile without outputs is rejected")
                .to_string(),
            "the esbuild metafile declares no outputs"
        );
    }

    #[test]
    fn propagates_an_ambiguous_input_error() {
        assert!(matches!(
            render_asset_bag(
                r#"{
                    "outputs": {
                        "assets/app_ABC.js": {
                            "imports": [],
                            "entryPoint": "src/app.ts",
                            "inputs": {}
                        },
                        "assets/other_ABC.js": {
                            "imports": [],
                            "inputs": { "src/app.ts": {} }
                        }
                    }
                }"#
            ),
            Err(AssetBagCodegenError::AmbiguousInput { input }) if input == "src/app.ts"
        ));
    }

    #[test]
    fn propagates_an_unsupported_entry_point_output_error() {
        assert!(matches!(
            render_asset_bag(
                r#"{
                    "outputs": {
                        "assets/mod_ABC.wasm": {
                            "imports": [],
                            "entryPoint": "src/mod.ts"
                        }
                    }
                }"#
            ),
            Err(AssetBagCodegenError::UnsupportedIncludeOutput { output }) if output == "assets/mod_ABC.wasm"
        ));
    }

    #[test]
    fn propagates_an_ambiguous_static_input_error() {
        assert!(matches!(
            render_asset_bag(
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
                }"#
            ),
            Err(AssetBagCodegenError::AmbiguousStaticInput { input, output_count })
                if input == "media/logo.png" && output_count == 2
        ));
    }
}
