use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::content_type::content_type;

const IMMUTABLE_CACHE_CONTROL: &str = "public, max-age=31536000, immutable";

fn responder_arm(output_path: &str, tail: &str, embed_relative: &str) -> TokenStream {
    let resolved_content_type = content_type(output_path);

    quote! {
        #tail => ::margaret_http::response::Response::static_bytes(
            200,
            #resolved_content_type,
            ::core::include_bytes!(::core::concat!(
                ::core::env!("CARGO_MANIFEST_DIR"),
                "/",
                #embed_relative,
                "/",
                #output_path
            )),
        )
        .header("cache-control", #IMMUTABLE_CACHE_CONTROL),
    }
}

pub(crate) fn render_asset_responder(
    relative_outputs: &BTreeMap<String, String>,
    embed_relative: &str,
    responder_type: &str,
) -> TokenStream {
    let type_identifier = format_ident!("{responder_type}");
    let mut arms: Vec<TokenStream> = Vec::new();

    for (output_path, tail) in relative_outputs {
        if output_path.ends_with(".map") {
            continue;
        }

        arms.push(responder_arm(output_path, tail, embed_relative));
    }

    quote! {
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
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::render_asset_responder;

    fn relative_outputs(paths: &[&str], directory: &str) -> BTreeMap<String, String> {
        let prefix = format!("{directory}/");

        paths
            .iter()
            .map(|path| {
                let tail = path
                    .strip_prefix(&prefix)
                    .expect("the fixture output is under the shared directory");

                ((*path).to_string(), tail.to_string())
            })
            .collect()
    }

    fn rendered(paths: &[&str], directory: &str, embed_relative: &str) -> String {
        render_asset_responder(
            &relative_outputs(paths, directory),
            embed_relative,
            "AssetResponder",
        )
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
        assert!(source.contains("static_bytes"));
        assert!(!source.contains("as_slice"));
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
        let source = rendered(
            &["assets/app_A1B2C3D4.js", "assets/app_A1B2C3D4.js.map"],
            "assets",
            "..",
        );

        assert!(source.contains("\"app_A1B2C3D4.js\" =>"));
        assert!(!source.contains(".map"));
    }
}
