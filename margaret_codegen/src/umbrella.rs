use margaret_generated_module::generated_module::GeneratedModule;

use crate::capabilities::Capabilities;

pub(crate) fn umbrella(
    Capabilities {
        has_console,
        has_http,
        serves,
    }: Capabilities,
) -> GeneratedModule {
    let mut source = String::from("#[rustfmt::skip]\npub mod container;\n");

    if has_http {
        source.push_str("#[rustfmt::skip]\npub mod forwarders;\n");
        source.push_str("#[rustfmt::skip]\npub mod http;\n");
        source.push_str("#[rustfmt::skip]\npub mod routes;\n");
    }

    if serves {
        source.push_str("#[rustfmt::skip]\npub mod services;\n");
    }

    if has_console {
        source.push_str("#[rustfmt::skip]\npub mod console;\n");
    }

    GeneratedModule::new("mod", source)
}
