use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::raw_esbuild_metafile::RawEsbuildMetafile;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::asset_bag_codegen_error::AssetBagCodegenError;
use crate::asset_bag_generation::AssetBagGeneration;
use crate::asset_macro_canonical_suffix::asset_macro_canonical_suffix;
use crate::asset_responder_canonical_suffix::asset_responder_canonical_suffix;
use crate::asset_slot::AssetSlot;
use crate::bundle_tokens::bundle_tokens;
use crate::cache_policy::CachePolicy;
use crate::enumerate_inputs::enumerate_inputs;
use crate::enumerated_inputs::EnumeratedInputs;
use crate::render_asset_responder::render_asset_responder;
use crate::resolution_tokens::resolution_tokens;
use crate::resolve_static_outputs::resolve_static_outputs;
use crate::served_assets::ServedAssets;

fn classify_served_tails(
    served_tails: &BTreeSet<String>,
    assets_directory_name: &str,
    output_paths: &BTreeSet<String>,
) -> BTreeMap<String, CachePolicy> {
    served_tails
        .iter()
        .map(|tail| {
            let output_path = format!("{assets_directory_name}/{tail}");
            let policy = if output_paths.contains(&output_path) {
                CachePolicy::Immutable
            } else {
                CachePolicy::Revalidate
            };

            (tail.clone(), policy)
        })
        .collect()
}

fn asset_macro_tokens(
    metafile: &EsbuildMetafile,
    enumerated: &EnumeratedInputs,
) -> Result<TokenStream, AssetBagCodegenError> {
    let mut inputs: BTreeSet<&String> = BTreeSet::new();

    inputs.extend(enumerated.entrypoints.keys());
    inputs.extend(&enumerated.static_inputs);

    let mut arm_tokens: Vec<TokenStream> = Vec::new();

    for input in inputs {
        let bundle = match enumerated.entrypoints.get(input) {
            Some(entrypoint) => AssetSlot::Present(bundle_tokens(metafile, entrypoint)?),
            None => AssetSlot::Absent,
        };
        let statics = resolve_static_outputs(metafile, input)?;

        if !bundle.is_present() && !statics.image.is_present() && !statics.file.is_present() {
            continue;
        }

        let handle = resolution_tokens(bundle, statics);

        arm_tokens.push(quote! {
            (#input) => { #handle };
        });
    }

    let [_, macro_name] = asset_macro_canonical_suffix();
    let macro_name = format_ident!("{macro_name}");

    Ok(quote! {
        macro_rules! #macro_name {
            #(#arm_tokens)*
            ($other:literal) => {
                ::core::compile_error!(::core::concat!("unknown esbuild asset input: ", $other))
            };
        }

        pub(crate) use #macro_name;
    })
}

fn asset_responder(
    ServedAssets {
        assets_directory_name,
        embed_relative,
        served_tails,
    }: &ServedAssets,
    output_paths: &BTreeSet<String>,
) -> Result<AssetResponder, AssetBagCodegenError> {
    validate_outputs_served(output_paths, assets_directory_name, served_tails)?;

    let [root_module, responder_module, responder_type] = asset_responder_canonical_suffix();
    let responder_module_identifier = format_ident!("{responder_module}");
    let served = classify_served_tails(served_tails, assets_directory_name, output_paths);

    Ok(AssetResponder {
        declaration: quote! { pub mod #responder_module_identifier; },
        module: GeneratedModuleTokens::new(
            format!("{root_module}/{responder_module}"),
            render_asset_responder(
                &served,
                assets_directory_name,
                embed_relative,
                responder_type,
            ),
        ),
    })
}

fn validate_outputs_served(
    output_paths: &BTreeSet<String>,
    assets_directory_name: &str,
    served_tails: &BTreeSet<String>,
) -> Result<(), AssetBagCodegenError> {
    let root_prefix = format!("{assets_directory_name}/");

    for output in output_paths {
        if Path::new(output).extension() == Some(OsStr::new("map")) {
            continue;
        }

        let Some(tail) = output.strip_prefix(&root_prefix) else {
            return Err(AssetBagCodegenError::AssetOutputOutsideRoot {
                output: output.clone(),
                root: assets_directory_name.to_string(),
            });
        };

        if !served_tails.contains(tail) {
            return Err(AssetBagCodegenError::MissingAssetOutput {
                output: output.clone(),
            });
        }
    }

    Ok(())
}

struct AssetResponder {
    declaration: TokenStream,
    module: GeneratedModuleTokens,
}

/// # Errors
///
/// Returns `AssetBagCodegenError` when the metafile is malformed, declares no outputs, or disagrees with the served files.
pub fn render_asset_bag(
    metafile_contents: &str,
    generation: &AssetBagGeneration,
) -> Result<Vec<GeneratedModuleTokens>, AssetBagCodegenError> {
    let raw: RawEsbuildMetafile = serde_json::from_str(metafile_contents)?;
    let enumerated = enumerate_inputs(&raw)?;
    let metafile: EsbuildMetafile = raw.into();
    let output_paths: BTreeSet<String> = metafile.get_output_paths().into_iter().collect();

    if output_paths.is_empty() {
        return Err(AssetBagCodegenError::EmptyMetafile);
    }

    let [root_module, ..] = asset_macro_canonical_suffix();

    let asset_macro = match generation {
        AssetBagGeneration::Macro | AssetBagGeneration::MacroAndResponder(_) => {
            asset_macro_tokens(&metafile, &enumerated)?
        }
        AssetBagGeneration::Responder(_) => TokenStream::new(),
    };

    Ok(match generation {
        AssetBagGeneration::Macro => vec![GeneratedModuleTokens::new(root_module, asset_macro)],
        AssetBagGeneration::MacroAndResponder(served) | AssetBagGeneration::Responder(served) => {
            let AssetResponder {
                declaration,
                module,
            } = asset_responder(served, &output_paths)?;

            vec![
                GeneratedModuleTokens::new(root_module, quote! { #declaration #asset_macro }),
                module,
            ]
        }
    })
}

#[cfg(test)]
mod tests {
    use super::render_asset_bag;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;
    use crate::asset_bag_generation::AssetBagGeneration;
    use crate::served_assets::ServedAssets;

    const FULL_METAFILE: &str = r#"{
        "outputs": {
            "assets/app_ABC.js": {
                "imports": [{ "path": "assets/chunk_ABC.js" }],
                "cssBundle": "assets/app_ABC.css",
                "entryPoint": "resources/ts/app.ts"
            },
            "assets/app_ABC.css": { "imports": [] },
            "assets/chunk_ABC.js": {
                "imports": [],
                "inputs": { "node_modules/react/index.js": {} }
            },
            "assets/logo_ABC.png": {
                "imports": [],
                "inputs": { "resources/media/logo.png": {} }
            },
            "assets/favicon_MWST.svg": {
                "imports": [],
                "inputs": { "resources/media/favicon.svg": {} }
            },
            "assets/favicon_VVSH.js": {
                "imports": [{ "path": "assets/favicon_MWST.svg", "kind": "file-loader" }],
                "entryPoint": "resources/media/favicon.svg",
                "inputs": { "resources/media/favicon.svg": {} }
            },
            "assets/inter_HASH.woff2": {
                "imports": [],
                "inputs": { "resources/fonts/inter.woff2": {} }
            }
        }
    }"#;

    fn formatted(metafile_contents: &str, generation: &AssetBagGeneration, name: &str) -> String {
        let modules = render_asset_bag(metafile_contents, generation)
            .expect("the asset bag module is generated");

        modules
            .into_iter()
            .find(|module| module.name() == name)
            .expect("the requested module is generated")
            .format()
            .expect("the generated tokens form a valid Rust file")
            .source()
            .to_string()
    }

    const FULL_METAFILE_TAILS: &[&str] = &[
        "app_ABC.css",
        "app_ABC.js",
        "chunk_ABC.js",
        "favicon_MWST.svg",
        "favicon_VVSH.js",
        "inter_HASH.woff2",
        "logo_ABC.png",
    ];

    fn served(served_tails: &[&str]) -> ServedAssets {
        ServedAssets {
            assets_directory_name: "assets".to_string(),
            embed_relative: "..".to_string(),
            served_tails: served_tails
                .iter()
                .map(|tail| (*tail).to_string())
                .collect(),
        }
    }

    fn emit(served_tails: &[&str]) -> AssetBagGeneration {
        AssetBagGeneration::MacroAndResponder(served(served_tails))
    }

    #[test]
    fn generates_the_macro_with_an_arm_per_addressable_input() {
        let source = formatted(FULL_METAFILE, &AssetBagGeneration::Macro, "asset_bag");

        assert!(!source.contains("EmbeddedAssets"));
        assert!(!source.contains("RustEmbed"));
        assert!(source.contains("macro_rules! asset"));
        assert!(source.contains("(\"resources/ts/app.ts\") =>"));
        assert!(source.contains("script_stylesheet_bundle::ScriptStylesheetBundle::new"));
        assert!(source.contains("(\"resources/media/logo.png\") =>"));
        assert!(source.contains("(\"resources/media/favicon.svg\") =>"));
        assert!(source.contains("script_bundle::ScriptBundle::new"));
        assert!(source.contains("image_output::ImageOutput::new"));
        assert!(source.contains("(\"resources/fonts/inter.woff2\") =>"));
        assert!(source.contains("file_output::FileOutput::new"));
        assert!(source.contains("unknown esbuild asset input: "));
        assert!(source.contains("pub(crate) use asset;"));
    }

    #[test]
    fn omits_the_responder_submodule_when_only_the_macro_is_generated() {
        let modules = render_asset_bag(FULL_METAFILE, &AssetBagGeneration::Macro)
            .expect("the asset bag module is generated");

        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name(), "asset_bag");

        let root = formatted(FULL_METAFILE, &AssetBagGeneration::Macro, "asset_bag");

        assert!(!root.contains("pub mod asset_responder"));
    }

    #[test]
    fn emits_the_responder_submodule_and_declares_it_from_the_root() {
        let modules = render_asset_bag(FULL_METAFILE, &emit(FULL_METAFILE_TAILS))
            .expect("the asset bag module is generated");

        assert_eq!(modules.len(), 2);
        assert!(modules.iter().any(|module| module.name() == "asset_bag"));
        assert!(
            modules
                .iter()
                .any(|module| module.name() == "asset_bag/asset_responder")
        );

        let root = formatted(FULL_METAFILE, &emit(FULL_METAFILE_TAILS), "asset_bag");
        let responder = formatted(
            FULL_METAFILE,
            &emit(FULL_METAFILE_TAILS),
            "asset_bag/asset_responder",
        );

        assert!(root.contains("pub mod asset_responder;"));
        assert!(responder.contains("pub struct AssetResponder"));
        assert!(responder.contains("\"app_ABC.js\" =>"));
        assert!(responder.contains("\"text/javascript\""));
        assert!(responder.contains("\"logo_ABC.png\" =>"));
        assert!(responder.contains("\"image/png\""));
        assert!(responder.contains("\"favicon_MWST.svg\" =>"));
        assert!(responder.contains("\"image/svg+xml\""));
        assert!(responder.contains("public, max-age=31536000, immutable"));
    }

    #[test]
    fn immutability_follows_metafile_membership_not_a_hash_like_name() {
        let responder = formatted(
            r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#,
            &emit(&["app_ABC.js", "orphan_DEADBEEF.js"]),
            "asset_bag/asset_responder",
        );

        let after_metafile_output = responder
            .split_once("\"app_ABC.js\" =>")
            .expect("the metafile output is served")
            .1;
        let (metafile_output_arm, hash_like_orphan_arm) = after_metafile_output
            .split_once("\"orphan_DEADBEEF.js\" =>")
            .expect("the hash-like orphan is served");

        assert!(metafile_output_arm.contains("public, max-age=31536000, immutable"));
        assert!(!metafile_output_arm.contains("no-cache"));
        assert!(hash_like_orphan_arm.contains("no-cache"));
        assert!(!hash_like_orphan_arm.contains("public, max-age=31536000, immutable"));
    }

    #[test]
    fn rejects_a_metafile_output_missing_from_the_served_files() {
        assert!(matches!(
            render_asset_bag(
                r#"{
                    "outputs": {
                        "assets/app_ABC.js": { "imports": [], "entryPoint": "src/app.ts" },
                        "assets/chunk_ABC.js": { "imports": [] }
                    }
                }"#,
                &emit(&["app_ABC.js"]),
            ),
            Err(AssetBagCodegenError::MissingAssetOutput { output })
                if output == "assets/chunk_ABC.js"
        ));
    }

    #[test]
    fn rejects_a_metafile_output_outside_the_asset_root() {
        assert!(matches!(
            render_asset_bag(
                r#"{"outputs":{"static/x_ABC.js":{"imports":[],"entryPoint":"src/x.ts"}}}"#,
                &emit(&[]),
            ),
            Err(AssetBagCodegenError::AssetOutputOutsideRoot { output, root })
                if output == "static/x_ABC.js" && root == "assets"
        ));
    }

    #[test]
    fn accepts_a_source_map_output_without_a_served_file() {
        let responder = formatted(
            r#"{
                "outputs": {
                    "assets/app_ABC.js": { "imports": [], "entryPoint": "src/app.ts" },
                    "assets/app_ABC.js.map": { "imports": [] }
                }
            }"#,
            &emit(&["app_ABC.js"]),
            "asset_bag/asset_responder",
        );

        assert!(responder.contains("\"app_ABC.js\" =>"));
        assert!(!responder.contains(".map"));
    }

    #[test]
    fn omits_an_arm_for_a_bundled_source_input_without_a_static_asset() {
        let source = formatted(FULL_METAFILE, &AssetBagGeneration::Macro, "asset_bag");

        assert!(!source.contains("node_modules/react/index.js"));
    }

    #[test]
    fn rejects_a_metafile_that_is_not_valid_json() {
        assert_eq!(
            render_asset_bag("not json", &AssetBagGeneration::Macro)
                .expect_err("invalid json is rejected")
                .to_string(),
            "failed to parse the esbuild metafile"
        );
    }

    #[test]
    fn rejects_a_metafile_without_outputs() {
        assert_eq!(
            render_asset_bag(r#"{ "outputs": {} }"#, &AssetBagGeneration::Macro)
                .expect_err("a metafile without outputs is rejected")
                .to_string(),
            "the esbuild metafile declares no outputs"
        );
    }

    #[test]
    fn propagates_an_ambiguous_static_input_error() {
        assert!(matches!(
            render_asset_bag(
                r#"{
                    "outputs": {
                        "assets/a_ABC.png": {
                            "imports": [],
                            "inputs": { "resources/media/logo.png": {} }
                        },
                        "assets/b_DEF.png": {
                            "imports": [],
                            "inputs": { "resources/media/logo.png": {} }
                        }
                    }
                }"#,
                &AssetBagGeneration::Macro,
            ),
            Err(AssetBagCodegenError::AmbiguousStaticInput { input, output_count })
                if input == "resources/media/logo.png" && output_count == 2
        ));
    }

    #[test]
    fn propagates_a_duplicate_entrypoint_error() {
        assert!(matches!(
            render_asset_bag(
                r#"{
                    "outputs": {
                        "assets/a_ABC.js": {
                            "imports": [],
                            "entryPoint": "resources/ts/app.ts"
                        },
                        "assets/b_DEF.js": {
                            "imports": [],
                            "entryPoint": "resources/ts/app.ts"
                        }
                    }
                }"#,
                &AssetBagGeneration::Macro,
            ),
            Err(AssetBagCodegenError::DuplicateEntrypoint { input })
                if input == "resources/ts/app.ts"
        ));
    }

    #[test]
    fn emits_octet_stream_for_an_unrecognized_output() {
        let responder = formatted(
            r#"{
                "outputs": {
                    "assets/model_HASH.bin": {
                        "imports": [],
                        "inputs": { "resources/model.bin": {} }
                    }
                }
            }"#,
            &emit(&["model_HASH.bin"]),
            "asset_bag/asset_responder",
        );

        assert!(responder.contains("\"model_HASH.bin\" =>"));
        assert!(responder.contains("\"application/octet-stream\""));
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
                }"#,
                &AssetBagGeneration::Macro,
            ),
            Err(AssetBagCodegenError::UnsupportedIncludeOutput { output }) if output == "assets/mod_ABC.wasm"
        ));
    }

    #[test]
    fn declares_only_the_responder_when_the_macro_is_not_generated() {
        let generation = AssetBagGeneration::Responder(served(FULL_METAFILE_TAILS));
        let root = formatted(FULL_METAFILE, &generation, "asset_bag");

        assert_eq!(root.trim(), "pub mod asset_responder;");
        assert!(
            formatted(FULL_METAFILE, &generation, "asset_bag/asset_responder")
                .contains("pub struct AssetResponder")
        );
    }
}
