use margaret_generated_module::generated_module::GeneratedModule;

use crate::capabilities::Capabilities;

pub(crate) fn umbrella(
    Capabilities {
        has_asset_bag,
        has_console,
        has_http,
        has_middleware,
        has_models,
        has_views,
        has_websockets,
        serves,
    }: Capabilities,
) -> GeneratedModule {
    let serves_http = has_http || has_websockets;
    let mut source = String::from("#[rustfmt::skip]\npub mod container;\n");

    if has_asset_bag {
        source.push_str("#[rustfmt::skip]\npub mod asset_bag;\n");
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

    if has_views && serves_http {
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
