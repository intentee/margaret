use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::http_codegen_error::HttpCodegenError;

pub(crate) fn authenticated_actor_store(
    index: &AttributeIndex,
) -> Result<Option<String>, HttpCodegenError> {
    let selector =
        AttributeSelector::parse("provides_authenticated_actor").expect("a valid selector");
    let mut store: Option<String> = None;

    for matched in index.select(&selector) {
        let path = matched.item().canonical_path().to_string();

        if let Some(existing) = &store {
            return Err(HttpCodegenError::AmbiguousAuthenticatedActorStore {
                first: existing.clone(),
                second: path,
            });
        }

        store = Some(path);
    }

    Ok(store)
}
