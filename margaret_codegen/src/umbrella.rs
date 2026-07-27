use margaret_generated_module::generated_module::GeneratedModule;

use crate::generated_features::GeneratedFeatures;

pub(crate) fn umbrella(
    GeneratedFeatures {
        has_asset_bag,
        has_authenticated_users,
        has_console,
        has_http,
        has_jwks,
        has_middleware,
        has_models,
        has_views,
        has_websockets,
        serves,
    }: GeneratedFeatures,
) -> GeneratedModule {
    let serves_http = has_http || has_websockets;
    let mut source =
        String::from("#![forbid(unsafe_code)]\n#[rustfmt::skip]\npub use ::margaret::framework;\n");

    source.push_str("#[rustfmt::skip]\npub mod container;\n");

    if has_asset_bag {
        source.push_str("#[rustfmt::skip]\npub mod asset_bag;\n");
    }

    if has_jwks {
        source.push_str("#[rustfmt::skip]\npub mod jwks;\n");
    }

    if has_authenticated_users && serves_http {
        source.push_str("#[rustfmt::skip]\npub mod authenticated_users;\n");
    }

    if serves_http {
        source.push_str("#[rustfmt::skip]\npub mod forwarders;\n");
        source.push_str("#[rustfmt::skip]\npub mod http;\n");
        source.push_str("#[rustfmt::skip]\npub mod routes;\n");
    }

    if has_middleware && serves_http {
        source.push_str("#[rustfmt::skip]\npub mod middleware;\n");
    }

    if has_websockets {
        source.push_str("#[rustfmt::skip]\npub mod websocket;\n");
    }

    if has_models {
        source.push_str("#[rustfmt::skip]\npub mod schema;\n");
    }

    if has_views {
        source.push_str("#[rustfmt::skip]\npub mod views;\n");
    }

    if serves {
        source.push_str("#[rustfmt::skip]\npub mod serve;\n");
    }

    if has_console {
        source.push_str("#[rustfmt::skip]\npub mod run;\n");
    }

    GeneratedModule::new("mod", source)
}

#[cfg(test)]
mod tests {
    use crate::generated_features::GeneratedFeatures;

    use super::umbrella;

    fn minimal_features() -> GeneratedFeatures {
        GeneratedFeatures {
            has_asset_bag: false,
            has_authenticated_users: false,
            has_console: false,
            has_http: false,
            has_jwks: false,
            has_middleware: false,
            has_models: false,
            has_views: false,
            has_websockets: false,
            serves: false,
        }
    }

    #[test]
    fn forwards_the_framework_re_exports_into_the_generated_module() {
        assert!(
            umbrella(minimal_features())
                .source()
                .contains("pub use ::margaret::framework;")
        );
    }

    #[test]
    fn forbids_unsafe_code_throughout_the_generated_module() {
        assert!(
            umbrella(minimal_features())
                .source()
                .starts_with("#![forbid(unsafe_code)]")
        );
    }
}
