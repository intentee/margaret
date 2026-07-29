use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::cache_policy::CachePolicy;
use crate::content_type::content_type;

fn responder_arm(
    tail: &str,
    policy: &CachePolicy,
    assets_directory_name: &str,
    embed_relative: &str,
) -> TokenStream {
    let resolved_content_type = content_type(tail);
    let cache_control = policy.header_value();
    let output_path = format!("{assets_directory_name}/{tail}");

    quote! {
        #tail => ::margaret::framework::http::response::Response::static_bytes(
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
        .header("cache-control", #cache_control),
    }
}

pub(crate) fn render_asset_responder(
    served: &BTreeMap<String, CachePolicy>,
    assets_directory_name: &str,
    embed_relative: &str,
    responder_type: &str,
) -> TokenStream {
    let type_identifier = format_ident!("{responder_type}");
    let mut arms: Vec<TokenStream> = Vec::new();

    for (tail, policy) in served {
        if Path::new(tail).extension() == Some(OsStr::new("map")) {
            continue;
        }

        arms.push(responder_arm(
            tail,
            policy,
            assets_directory_name,
            embed_relative,
        ));
    }

    quote! {
        pub struct #type_identifier;

        impl #type_identifier {
            #[must_use]
            pub fn respond(&self, asset_path: &str) -> ::margaret::framework::http::response::Response {
                match asset_path {
                    #(#arms)*
                    _ => ::margaret::framework::http::response::Response::not_found(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::render_asset_responder;
    use crate::cache_policy::CachePolicy;

    fn rendered(served: &BTreeMap<String, CachePolicy>) -> String {
        render_asset_responder(served, "assets", "..", "AssetResponder").to_string()
    }

    #[test]
    fn renders_one_arm_per_served_tail_with_its_cache_policy() {
        let mut served = BTreeMap::new();
        served.insert("app_A1B2C3D4.js".to_string(), CachePolicy::Immutable);
        served.insert("service_worker.js".to_string(), CachePolicy::Revalidate);

        let source = rendered(&served);

        assert!(source.contains("pub struct AssetResponder"));
        assert!(source.contains("static_bytes"));
        assert!(!source.contains("as_slice"));
        assert!(source.contains("\"app_A1B2C3D4.js\" =>"));
        assert!(source.contains("\"service_worker.js\" =>"));
        assert!(source.contains("\"text/javascript\""));
        assert!(source.contains("\"public, max-age=31536000, immutable\""));
        assert!(source.contains("\"no-cache\""));
        assert!(source.contains("CARGO_MANIFEST_DIR"));
        assert!(source.contains("\"..\""));
        assert!(source.contains("\"assets/app_A1B2C3D4.js\""));
        assert!(source.contains("\"assets/service_worker.js\""));
        assert!(source.contains("not_found"));
    }

    #[test]
    fn omits_source_map_tails_from_the_served_set() {
        let mut served = BTreeMap::new();
        served.insert("app_A1B2C3D4.js".to_string(), CachePolicy::Immutable);
        served.insert("app_A1B2C3D4.js.map".to_string(), CachePolicy::Immutable);

        let source = rendered(&served);

        assert!(source.contains("\"app_A1B2C3D4.js\" =>"));
        assert!(!source.contains(".map"));
    }

    #[test]
    fn renders_only_the_fallback_for_an_empty_served_set() {
        let source = rendered(&BTreeMap::new());

        assert!(source.contains("pub struct AssetResponder"));
        assert!(source.contains("not_found"));
        assert!(!source.contains("static_bytes"));
    }
}
