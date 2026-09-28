use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_tag_codegen::read_reference_tag::read_reference_tag;
use margaret_tag_codegen::tag_kind::TagKind;

use crate::layer_application::LayerApplication;
use crate::middleware_codegen_error::MiddlewareCodegenError;
use crate::middleware_plans::MiddlewarePlans;

/// # Errors
///
/// Returns `MiddlewareCodegenError::Tag` or `MiddlewareCodegenError::UnplannedMiddlewareHandler`.
pub fn resolve_layers(
    item: &IndexedItem,
    MiddlewarePlans { plans, tags }: &MiddlewarePlans,
    site: &str,
) -> Result<Vec<LayerApplication>, MiddlewareCodegenError> {
    let mut layers = Vec::new();

    for matched in AttributeQuery::new(item).find_all_framework(FrameworkAttribute::Middleware) {
        let tag = read_reference_tag(matched.args()?, site)?;
        let handler = tags.resolve(&tag, TagKind::Middleware, site)?;
        let Some(plan) = plans
            .iter()
            .find(|plan| plan.concrete == *handler.canonical_path())
        else {
            return Err(MiddlewareCodegenError::UnplannedMiddlewareHandler {
                handler: handler.canonical_path().to_string(),
                site: site.to_string(),
            });
        };

        layers.push(LayerApplication {
            concrete: plan.concrete.clone(),
            field: plan.field.clone(),
            injects_peer_spiffe_id: plan.injections.peer_spiffe_id,
            injects_routes: plan.injections.routes,
            injects_views: plan.injections.views,
            wrapper: plan.wrapper.clone(),
        });
    }

    Ok(layers)
}
