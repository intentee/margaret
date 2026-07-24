use std::collections::BTreeMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_service_codegen::jwks_client_endpoint::JwksClientEndpoint;

use crate::codegen_error::CodegenError;

fn serve_roots(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
) -> Result<Vec<CanonicalPath>, CodegenError> {
    let markers = ["service", "scheduled_with_tick_timer"];
    let mut roots = Vec::new();

    for marker in markers {
        let selector = AttributeSelector::from_marker(marker);

        for matched in index.select(&selector) {
            roots.push(matched.item().canonical_path().clone());
        }
    }

    if let Some(endpoint) = JwksClientEndpoint::resolve(index, bindings)? {
        roots.push(endpoint.path);
    }

    Ok(roots)
}

pub(crate) fn serve_arguments(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
    server_console_arguments: &BTreeMap<String, Vec<ConsoleArgument>>,
    views_console_arguments: &[ConsoleArgument],
) -> Result<Vec<ConsoleArgument>, CodegenError> {
    let mut woven: Vec<ConsoleArgument> = Vec::new();

    for arguments in server_console_arguments.values() {
        woven.extend_from_slice(arguments);
    }

    woven.extend_from_slice(views_console_arguments);

    Ok(bindings.serve_arguments(&serve_roots(index, bindings)?, &woven)?)
}
