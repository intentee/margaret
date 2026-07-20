use std::collections::BTreeMap;
use std::collections::HashMap;

use esbuild_metafile::import::Import;
use esbuild_metafile::input_in_output::InputInOutput;
use esbuild_metafile::output::Output;
use esbuild_metafile::raw_esbuild_metafile::RawEsbuildMetafile;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::provided_singleton::ProvidedSingleton;

use crate::esbuild_metafile_codegen_error::EsbuildMetafileCodegenError;

fn esbuild_metafile_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "esbuild_metafile".to_string(),
        "esbuild_metafile".to_string(),
        "EsbuildMetafile".to_string(),
    ])
}

fn imports_tokens(imports: &[Import]) -> TokenStream {
    let entries = imports.iter().map(|Import { path }| {
        quote! {
            esbuild_metafile::import::Import {
                path: String::from(#path),
            }
        }
    });

    quote! { vec![#(#entries),*] }
}

fn inputs_tokens(inputs: &HashMap<String, InputInOutput>) -> TokenStream {
    let sorted: BTreeMap<&String, &InputInOutput> = inputs.iter().collect();
    let entries: Vec<TokenStream> = sorted
        .into_keys()
        .map(|input_path| {
            quote! {
                (
                    String::from(#input_path),
                    esbuild_metafile::input_in_output::InputInOutput {},
                )
            }
        })
        .collect();

    map_tokens(entries)
}

fn map_tokens(entries: Vec<TokenStream>) -> TokenStream {
    if entries.is_empty() {
        quote! { std::collections::HashMap::new() }
    } else {
        quote! { std::collections::HashMap::from([#(#entries),*]) }
    }
}

fn optional_string_tokens(value: &Option<String>) -> TokenStream {
    match value {
        None => quote! { None },
        Some(value) => quote! { Some(String::from(#value)) },
    }
}

fn output_tokens(
    Output {
        imports,
        css_bundle,
        entry_point,
        inputs,
    }: &Output,
) -> TokenStream {
    let imports = imports_tokens(imports);
    let css_bundle = optional_string_tokens(css_bundle);
    let entry_point = optional_string_tokens(entry_point);
    let inputs = inputs_tokens(inputs);

    quote! {
        esbuild_metafile::output::Output {
            imports: #imports,
            css_bundle: #css_bundle,
            entry_point: #entry_point,
            inputs: #inputs,
        }
    }
}

fn outputs_tokens(outputs: &HashMap<String, Output>) -> TokenStream {
    let sorted: BTreeMap<&String, &Output> = outputs.iter().collect();
    let entries: Vec<TokenStream> = sorted
        .into_iter()
        .map(|(output_path, output)| {
            let output = output_tokens(output);

            quote! { (String::from(#output_path), #output) }
        })
        .collect();

    map_tokens(entries)
}

pub fn esbuild_metafile_provider(
    json: &str,
) -> Result<ProvidedSingleton, EsbuildMetafileCodegenError> {
    let RawEsbuildMetafile { outputs } = serde_json::from_str(json)?;
    let outputs = outputs_tokens(&outputs);

    Ok(ProvidedSingleton {
        construction: quote! {
            esbuild_metafile::esbuild_metafile::EsbuildMetafile::from(
                esbuild_metafile::raw_esbuild_metafile::RawEsbuildMetafile {
                    outputs: #outputs,
                },
            )
        },
        path: esbuild_metafile_path(),
    })
}

#[cfg(test)]
mod tests {
    use super::esbuild_metafile_provider;

    const RICH_METAFILE: &str = r#"{
        "outputs": {
            "dist/main.js": {
                "imports": [{"path": "dist/chunk.js"}],
                "cssBundle": "dist/main.css",
                "entryPoint": "src/main.ts",
                "inputs": {}
            },
            "dist/logo.png": {
                "imports": [],
                "inputs": {"src/logo.png": {}}
            }
        }
    }"#;

    fn collapsed(json: &str) -> String {
        esbuild_metafile_provider(json)
            .expect("the metafile is valid")
            .construction
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn bakes_the_raw_metafile_as_an_infallible_from_conversion() {
        let source = collapsed(RICH_METAFILE);

        assert!(source.contains(
            "esbuild_metafile::esbuild_metafile::EsbuildMetafile::from(esbuild_metafile::raw_esbuild_metafile::RawEsbuildMetafile"
        ));
        assert!(source.contains("esbuild_metafile::output::Output"));
        assert!(
            source.contains("esbuild_metafile::import::Import{path:String::from(\"dist/chunk.js\"),}")
        );
        assert!(source.contains("css_bundle:Some(String::from(\"dist/main.css\"))"));
        assert!(source.contains("entry_point:Some(String::from(\"src/main.ts\"))"));
        assert!(source.contains("css_bundle:None"));
        assert!(source.contains("entry_point:None"));
        assert!(source.contains("esbuild_metafile::input_in_output::InputInOutput{}"));
        assert!(source.contains("std::collections::HashMap::from(["));
        assert!(source.contains("std::collections::HashMap::new()"));
    }

    #[test]
    fn targets_the_canonical_esbuild_metafile_path() {
        let provider = esbuild_metafile_provider(RICH_METAFILE).expect("the metafile is valid");

        assert_eq!(
            provider.path.to_string(),
            "esbuild_metafile::esbuild_metafile::EsbuildMetafile"
        );
    }

    #[test]
    fn sorts_outputs_by_key() {
        let source = collapsed(
            r#"{"outputs": {"dist/z.js": {"imports": [], "entryPoint": "src/z.ts", "inputs": {}}, "dist/a.js": {"imports": [], "entryPoint": "src/a.ts", "inputs": {}}}}"#,
        );

        let first = source.find("dist/a.js").expect("the a output is present");
        let second = source.find("dist/z.js").expect("the z output is present");

        assert!(first < second);
    }

    #[test]
    fn emits_an_empty_map_for_an_empty_metafile() {
        let source = collapsed(r#"{"outputs": {}}"#);

        assert!(source.contains("outputs:std::collections::HashMap::new()"));
    }

    #[test]
    fn rejects_an_invalid_metafile() {
        assert!(esbuild_metafile_provider("not valid json").is_err());
    }
}
