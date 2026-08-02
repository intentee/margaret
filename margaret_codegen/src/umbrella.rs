use margaret_generated_module::generated_module::GeneratedModule;

use crate::generated_feature::GeneratedFeature;
use crate::generated_features::GeneratedFeatures;

pub(crate) fn umbrella(features: &GeneratedFeatures) -> GeneratedModule {
    let serves_http = features.serves_http();
    let mut source = String::from("#![forbid(unsafe_code)]\n");

    source.push_str("#[rustfmt::skip]\npub mod container;\n");

    if features.contains(GeneratedFeature::AssetBag) {
        source.push_str("#[rustfmt::skip]\npub mod asset_bag;\n");
    }

    if features.contains(GeneratedFeature::Jwks) {
        source.push_str("#[rustfmt::skip]\npub mod jwks;\n");
    }

    if features.contains(GeneratedFeature::AuthenticatedUsers) && serves_http {
        source.push_str("#[rustfmt::skip]\npub mod authenticated_users;\n");
    }

    if serves_http {
        source.push_str("#[rustfmt::skip]\npub mod forwarders;\n");
        source.push_str("#[rustfmt::skip]\npub mod http;\n");
        source.push_str("#[rustfmt::skip]\npub mod routes;\n");
    }

    if features.contains(GeneratedFeature::Middleware) && serves_http {
        source.push_str("#[rustfmt::skip]\npub mod middleware;\n");
    }

    if features.contains(GeneratedFeature::Websockets) {
        source.push_str("#[rustfmt::skip]\npub mod websocket;\n");
    }

    if features.contains(GeneratedFeature::Models) {
        source.push_str("#[rustfmt::skip]\npub mod schema;\n");
    }

    if features.contains(GeneratedFeature::Views) {
        source.push_str("#[rustfmt::skip]\npub mod views;\n");
    }

    if features.contains(GeneratedFeature::Serves) {
        source.push_str("#[rustfmt::skip]\npub mod serve;\n");
    }

    if features.contains(GeneratedFeature::Console) {
        source.push_str("#[rustfmt::skip]\npub mod run;\n");
    }

    source.push_str("#[rustfmt::skip]\npub use ::margaret::framework;\n");

    GeneratedModule::new("mod", source)
}

#[cfg(test)]
mod tests {
    use crate::generated_features::GeneratedFeatures;

    use super::umbrella;

    fn minimal_features() -> GeneratedFeatures {
        GeneratedFeatures::default()
    }

    #[test]
    fn forwards_the_framework_re_exports_into_the_generated_module() {
        assert!(
            umbrella(&minimal_features())
                .source()
                .contains("pub use ::margaret::framework;")
        );
    }

    #[test]
    fn forbids_unsafe_code_throughout_the_generated_module() {
        assert!(
            umbrella(&minimal_features())
                .source()
                .starts_with("#![forbid(unsafe_code)]")
        );
    }
}
