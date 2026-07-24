use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_tag_codegen::read_reference_tag::read_reference_tag;

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
        let tag = read_reference_tag(matched.args()?, site)?;
        let Some(plan) = plans.iter().find(|plan| plan.tag == tag) else {
            return Err(MiddlewareCodegenError::UnknownMiddleware {
                site: site.to_string(),
                tag: tag.to_string(),
            });
        };

        layers.push(LayerApplication {
            concrete: plan.concrete.clone(),
            field: plan.field.clone(),
            injects_peer_spiffe_id: plan.injects_peer_spiffe_id(),
            injects_routes: plan.injects_routes(),
            injects_views: plan.injects_views(),
            wrapper: plan.wrapper.clone(),
        });
    }

    Ok(layers)
}
