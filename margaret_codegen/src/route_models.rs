use margaret_active_record_codegen::shape_declaration::ShapeDeclaration;
use margaret_model_codegen::model::Model;
use margaret_request_binding_codegen::route_model::RouteModel;
use margaret_request_binding_codegen::route_model_key::RouteModelKey;

fn route_model_key(model: &Model) -> RouteModelKey {
    match model.indexes.primary_key.fields.as_slice() {
        [_] => RouteModelKey::Single,
        _ => RouteModelKey::Composite,
    }
}

pub(crate) fn route_models(models: &[Model], shapes: &[ShapeDeclaration]) -> Vec<RouteModel> {
    models
        .iter()
        .map(|model| RouteModel {
            loaded: model.path.clone(),
            primary_key: route_model_key(model),
        })
        .chain(shapes.iter().map(|shape| RouteModel {
            loaded: shape.path.clone(),
            primary_key: route_model_key(shape.model),
        }))
        .collect()
}
