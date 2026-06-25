use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;
use syn::FnArg;
use syn::Pat;

use crate::http_codegen_error::HttpCodegenError;
use crate::route_parameter::RouteParameter;

pub(crate) fn responder_method(
    index: &AttributeIndex,
    responder_path: &CanonicalPath,
    responder: &str,
) -> Result<Vec<RouteParameter>, HttpCodegenError> {
    let method = find_responder_method(index, responder_path).ok_or_else(|| {
        HttpCodegenError::MissingResponderMethod {
            responder: responder.to_string(),
        }
    })?;
    let route_parameter_selector =
        AttributeSelector::parse("route_parameter").expect("a valid selector");
    let mut parameters = Vec::new();

    for (position, input) in method.signature().inputs.iter().enumerate() {
        let FnArg::Typed(pattern_type) = input else {
            continue;
        };

        let is_route_parameter = pattern_type
            .attrs
            .iter()
            .any(|attribute| route_parameter_selector.matches(attribute.path()));

        if !is_route_parameter {
            return Err(HttpCodegenError::UnmarkedResponderParameter {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        }

        let Pat::Ident(pattern_ident) = &*pattern_type.pat else {
            return Err(HttpCodegenError::RouteParameterNotIdentifier {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        };

        parameters.push(RouteParameter {
            name: pattern_ident.ident.to_string(),
        });
    }

    Ok(parameters)
}

fn find_responder_method<'index>(
    index: &'index AttributeIndex,
    responder_path: &CanonicalPath,
) -> Option<&'index IndexedMethod> {
    let responder_selector = AttributeSelector::parse("responder").expect("a valid selector");

    index
        .select(&responder_selector)
        .into_iter()
        .find_map(|matched| match matched.holder() {
            AttributeHolder::Method(method) if method.self_type_path() == responder_path => {
                Some(method)
            }
            _ => None,
        })
}
