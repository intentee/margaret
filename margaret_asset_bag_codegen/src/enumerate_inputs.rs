use std::collections::BTreeMap;
use std::collections::BTreeSet;

use esbuild_metafile::raw_esbuild_metafile::RawEsbuildMetafile;

use crate::asset_bag_codegen_error::AssetBagCodegenError;
use crate::entrypoint_output::EntrypointOutput;
use crate::enumerated_inputs::EnumeratedInputs;

pub(crate) fn enumerate_inputs(
    raw: &RawEsbuildMetafile,
) -> Result<EnumeratedInputs, AssetBagCodegenError> {
    let mut entrypoints: BTreeMap<String, EntrypointOutput> = BTreeMap::new();
    let mut static_inputs: BTreeSet<String> = BTreeSet::new();

    for (output_path, output) in &raw.outputs {
        match &output.entry_point {
            Some(entry_point) => {
                let replaced = entrypoints.insert(
                    entry_point.clone(),
                    EntrypointOutput {
                        css_bundle: output.css_bundle.clone(),
                        main_output: output_path.clone(),
                    },
                );

                if replaced.is_some() {
                    return Err(AssetBagCodegenError::DuplicateEntrypoint {
                        input: entry_point.clone(),
                    });
                }
            }
            None => {
                for input in output.inputs.keys() {
                    static_inputs.insert(input.clone());
                }
            }
        }
    }

    for entry_point in entrypoints.keys() {
        if static_inputs.contains(entry_point) {
            return Err(AssetBagCodegenError::AmbiguousInput {
                input: entry_point.clone(),
            });
        }
    }

    Ok(EnumeratedInputs {
        entrypoints,
        static_inputs,
    })
}

#[cfg(test)]
mod tests {
    use esbuild_metafile::raw_esbuild_metafile::RawEsbuildMetafile;

    use super::enumerate_inputs;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;

    fn raw(json: &str) -> RawEsbuildMetafile {
        serde_json::from_str(json).expect("the fixture metafile parses")
    }

    #[test]
    fn captures_each_entry_point_main_output_and_css_bundle() {
        let enumerated = enumerate_inputs(&raw(r#"{
                "outputs": {
                    "assets/app_ABC.js": {
                        "imports": [],
                        "cssBundle": "assets/app_ABC.css",
                        "entryPoint": "src/app.ts",
                        "inputs": {}
                    },
                    "assets/app_ABC.css": { "imports": [], "inputs": {} },
                    "assets/logo_ABC.png": {
                        "imports": [],
                        "inputs": { "media/logo.png": {} }
                    }
                }
            }"#))
        .expect("the inputs are enumerated");

        let entrypoint = enumerated
            .entrypoints
            .get("src/app.ts")
            .expect("the entry point is captured");

        assert_eq!(entrypoint.main_output, "assets/app_ABC.js");
        assert_eq!(entrypoint.css_bundle.as_deref(), Some("assets/app_ABC.css"));
        assert!(enumerated.static_inputs.contains("media/logo.png"));
    }

    #[test]
    fn rejects_an_input_that_is_both_an_entry_point_and_a_static_input() {
        assert!(matches!(
            enumerate_inputs(&raw(
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
                }"#,
            )),
            Err(AssetBagCodegenError::AmbiguousInput { input }) if input == "src/app.ts"
        ));
    }

    #[test]
    fn rejects_two_outputs_declaring_the_same_entry_point() {
        assert!(matches!(
            enumerate_inputs(&raw(
                r#"{
                    "outputs": {
                        "assets/app_ABC.js": {
                            "imports": [],
                            "entryPoint": "src/app.ts",
                            "inputs": {}
                        },
                        "assets/app_DEF.js": {
                            "imports": [],
                            "entryPoint": "src/app.ts",
                            "inputs": {}
                        }
                    }
                }"#,
            )),
            Err(AssetBagCodegenError::DuplicateEntrypoint { input }) if input == "src/app.ts"
        ));
    }
}
