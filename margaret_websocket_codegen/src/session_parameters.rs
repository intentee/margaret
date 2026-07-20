use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::marker::marker;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::injectable_resolution::InjectableResolution;
use margaret_container::resolve_injectable::resolve_injectable;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;
use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::session_parameter::SessionParameter;
use crate::websocket_codegen_error::WebSocketCodegenError;

pub(crate) fn session_parameters(
    index: &AttributeIndex,
    item: &IndexedItem,
    method: &IndexedMethod,
    session: &str,
    path: &str,
    route_path: &RoutePath,
    bindings: &ContainerBindings,
) -> Result<Vec<SessionParameter>, WebSocketCodegenError> {
    let route_parameter_selector = AttributeSelector::from_marker("route_parameter");
    let mut resolved = Vec::new();

    for ParameterView {
        attributes,
        declared,
        holder,
        position,
    } in parameters(method.signature())
    {
        match marker(attributes, &route_parameter_selector) {
            Some(attribute) => {
                let arguments = AttributeArgs::from_attribute(attribute)?;
                let from = arguments.string("from")?.ok_or_else(|| {
                    WebSocketCodegenError::SessionRouteParameterMissingFrom {
                        session: session.to_string(),
                        parameter: position.to_string(),
                    }
                })?;

                if !route_path.parameters().any(|name| name == from) {
                    return Err(WebSocketCodegenError::SessionRouteParameterNotInPath {
                        session: session.to_string(),
                        from,
                        path: path.to_string(),
                    });
                }

                resolved.push(SessionParameter::Route { from, holder });
            }
            None => match resolve_injectable(index, item, declared, bindings) {
                InjectableResolution::Resolved(dependency) => {
                    resolved.push(SessionParameter::Injectable { dependency, holder });
                }
                InjectableResolution::UnsupportedShape => {
                    return Err(WebSocketCodegenError::UnsupportedSessionParameterShape {
                        session: session.to_string(),
                        parameter: position.to_string(),
                    });
                }
                InjectableResolution::MissingProvider => {
                    return Err(WebSocketCodegenError::MissingSessionProvider {
                        session: session.to_string(),
                        parameter: position.to_string(),
                    });
                }
            },
        }
    }

    Ok(resolved)
}
