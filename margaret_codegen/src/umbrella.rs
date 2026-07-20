use margaret_generated_module::generated_module::GeneratedModule;

use crate::capabilities::Capabilities;

pub(crate) fn umbrella(
    Capabilities {
        has_console,
        has_http,
        has_views,
        has_websocket,
        serves,
    }: Capabilities,
) -> GeneratedModule {
    let serves_http = has_http || has_websocket;
    let mut source = String::from("#[rustfmt::skip]\npub mod container;\n");

    if serves_http {
        source.push_str("#[rustfmt::skip]\npub mod forwarders;\n");
        source.push_str("#[rustfmt::skip]\npub mod http;\n");
        source.push_str("#[rustfmt::skip]\npub mod routes;\n");
    }

    if has_websocket {
        source.push_str("#[rustfmt::skip]\npub mod websocket;\n");
    }

    if has_views && has_http {
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
