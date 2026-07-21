use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::format_path::format_path;
use margaret_attributes::indexed_item::IndexedItem;

use crate::layer_application::LayerApplication;
use crate::middleware_codegen_error::MiddlewareCodegenError;
use crate::middleware_plan::MiddlewarePlan;

pub fn resolve_layers(
    item: &IndexedItem,
    plans: &[MiddlewarePlan],
    site: &str,
) -> Result<Vec<LayerApplication>, MiddlewareCodegenError> {
    let selector = AttributeSelector::from_marker("middleware");
    let mut layers = Vec::new();

    for matched in AttributeQuery::new(item).find_all(&selector) {
        let arguments = matched.args()?;
        let (Some(tag), None) = (arguments.positional_path(0), arguments.positional(1)) else {
            return Err(MiddlewareCodegenError::MalformedMiddleware {
                site: site.to_string(),
            });
        };
        let mut matching = plans.iter().filter(|plan| plan.selector.matches(tag));
        let Some(plan) = matching.next() else {
            return Err(MiddlewareCodegenError::UnknownMiddleware {
                site: site.to_string(),
                tag: format_path(tag),
            });
        };

        if matching.next().is_some() {
            return Err(MiddlewareCodegenError::AmbiguousMiddleware {
                site: site.to_string(),
                tag: format_path(tag),
            });
        }

        layers.push(LayerApplication {
            field: plan.field.clone(),
            injects_peer_spiffe_id: plan.injects_peer_spiffe_id(),
            injects_routes: plan.injects_routes(),
            injects_views: plan.injects_views(),
            wrapper: plan.wrapper.clone(),
        });
    }

    Ok(layers)
}
