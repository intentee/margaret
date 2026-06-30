use std::collections::HashMap;

use quote::quote;
use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::http_codegen_error::HttpCodegenError;

pub(crate) fn site_action_gates(
    index: &AttributeIndex,
) -> Result<HashMap<Path, String>, HttpCodegenError> {
    let selector = AttributeSelector::parse("decides_site_action").expect("a valid selector");
    let mut registry: HashMap<Path, String> = HashMap::new();

    for matched in index.select(&selector) {
        let gate = matched.item().canonical_path().to_string();
        let action = matched
            .args()?
            .positional_path(0)
            .ok_or(HttpCodegenError::MissingSiteActionArgument { gate: gate.clone() })?
            .clone();

        if let Some(existing) = registry.get(&action) {
            return Err(HttpCodegenError::AmbiguousSiteActionGate {
                action: quote! { #action }.to_string(),
                first: existing.clone(),
                second: gate,
            });
        }

        registry.insert(action, gate);
    }

    Ok(registry)
}
