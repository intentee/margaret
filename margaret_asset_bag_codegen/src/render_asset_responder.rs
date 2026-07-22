use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::asset_bag_codegen_error::AssetBagCodegenError;
use crate::content_type::content_type;

const IMMUTABLE_CACHE_CONTROL: &str = "public, max-age=31536000, immutable";

fn responder_arm(
    output_path: &str,
    tail: &str,
    embed_relative: &str,
) -> Result<TokenStream, AssetBagCodegenError> {
    let resolved_content_type = content_type(output_path)?;

    Ok(quote! {
        #tail => ::margaret_http::response::Response::bytes(
            200,
            #resolved_content_type,
            ::core::include_bytes!(::core::concat!(
                ::core::env!("CARGO_MANIFEST_DIR"),
                "/",
                #embed_relative,
                "/",
                #output_path
            )).as_slice(),
        )
        .header("cache-control", #IMMUTABLE_CACHE_CONTROL),
    })
}

pub(crate) fn render_asset_responder(
    output_paths: &BTreeSet<String>,
    directory: &str,
    embed_relative: &str,
    responder_type: &str,
) -> Result<TokenStream, AssetBagCodegenError> {
    let prefix = format!("{directory}/");
    let type_identifier = format_ident!("{responder_type}");
    let mut arms: Vec<TokenStream> = Vec::new();

    for output_path in output_paths {
        if output_path.ends_with(".map") {
            continue;
        }

        let tail = output_path.strip_prefix(&prefix).ok_or_else(|| {
            AssetBagCodegenError::OutputOutsideDirectory {
                output: output_path.clone(),
                directory: directory.to_string(),
            }
        })?;

        arms.push(responder_arm(output_path, tail, embed_relative)?);
    }

    Ok(quote! {
        pub struct #type_identifier;

        impl #type_identifier {
            #[must_use]
            pub fn respond(&self, asset_path: &str) -> ::margaret_http::response::Response {
                match asset_path {
                    #(#arms)*
                    _ => ::margaret_http::response::Response::not_found(),
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::render_asset_responder;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;

    fn output_paths(paths: &[&str]) -> BTreeSet<String> {
        paths.iter().map(|path| (*path).to_string()).collect()
    }

    fn rendered(paths: &[&str], directory: &str, embed_relative: &str) -> String {
        render_asset_responder(&output_paths(paths), directory, embed_relative, "AssetResponder")
            .expect("the responder renders")
            .to_string()
    }

    #[test]
    fn renders_one_arm_per_output_keyed_by_the_directory_relative_tail() {
        let source = rendered(
            &["assets/app_A1B2C3D4.js", "assets/logo_I9J0K1L2.png"],
            "assets",
            "..",
        );

        assert!(source.contains("pub struct AssetResponder"));
        assert!(source.contains("\"app_A1B2C3D4.js\" =>"));
        assert!(source.contains("\"logo_I9J0K1L2.png\" =>"));
        assert!(source.contains("\"text/javascript\""));
        assert!(source.contains("\"image/png\""));
        assert!(source.contains("\"public, max-age=31536000, immutable\""));
        assert!(source.contains("CARGO_MANIFEST_DIR"));
        assert!(source.contains("\"..\""));
        assert!(source.contains("\"assets/app_A1B2C3D4.js\""));
        assert!(source.contains("not_found"));
    }

    #[test]
    fn omits_source_map_outputs_from_the_served_set() {
        let source = rendered(&["assets/app_A1B2C3D4.js", "assets/app_A1B2C3D4.js.map"], "assets", "..");

        assert!(source.contains("\"app_A1B2C3D4.js\" =>"));
        assert!(!source.contains(".map"));
    }

    #[test]
    fn rejects_an_output_outside_the_shared_directory() {
        assert!(matches!(
            render_asset_responder(
                &output_paths(&["static/app_A1B2C3D4.js"]),
                "assets",
                "..",
                "AssetResponder",
            ),
            Err(AssetBagCodegenError::OutputOutsideDirectory { output, directory })
                if output == "static/app_A1B2C3D4.js" && directory == "assets"
        ));
    }

    #[test]
    fn propagates_an_unsupported_content_type() {
        assert!(matches!(
            render_asset_responder(
                &output_paths(&["assets/model_HASH.bin"]),
                "assets",
                "..",
                "AssetResponder",
            ),
            Err(AssetBagCodegenError::UnsupportedAssetContentType { output })
                if output == "assets/model_HASH.bin"
        ));
    }
}
