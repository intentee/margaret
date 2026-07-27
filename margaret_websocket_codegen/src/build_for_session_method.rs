use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;

use crate::websocket_codegen_error::WebSocketCodegenError;

pub(crate) fn build_for_session_method<'index>(
    item: &'index IndexedItem,
    session: &str,
) -> Result<&'index IndexedMethod, WebSocketCodegenError> {
    let selector = AttributeSelector::from_marker("build_for_session");
    let mut found: Vec<&IndexedMethod> = item
        .methods()
        .iter()
        .filter(|method| method.has_attribute(&selector))
        .collect();

    if found.len() > 1 {
        return Err(WebSocketCodegenError::AmbiguousBuildForSession {
            session: session.to_string(),
            methods: found
                .iter()
                .map(|method| method.identifier().to_string())
                .collect::<Vec<String>>()
                .join(", "),
        });
    }

    found
        .pop()
        .ok_or_else(|| WebSocketCodegenError::SessionMissingBuildForSession {
            session: session.to_string(),
        })
}
