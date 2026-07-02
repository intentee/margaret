use quote::ToTokens;
use quote::format_ident;
use syn::Attribute;
use syn::FnArg;
use syn::Pat;
use syn::ReturnType;
use syn::Type;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::resolve_struct::resolve_struct;
use margaret_attributes::resolve_trait::resolve_trait;
use margaret_attributes::type_leaf_ident::type_leaf_ident;

use crate::http_codegen_error::HttpCodegenError;
use crate::registries::Registries;
use crate::responder_output::ResponderOutput;
use crate::responder_signature::ResponderSignature;
use crate::route_parameter::RouteParameter;
use crate::route_parameter_arguments::RouteParameterArguments;
use crate::route_parameter_binding::RouteParameterBinding;

fn classify(
    responder: &str,
    arguments: RouteParameterArguments,
    declared: &Type,
    registries: &Registries,
) -> Result<RouteParameterBinding, HttpCodegenError> {
    let RouteParameterArguments { from } = arguments;

    if is_string(declared) {
        return Ok(RouteParameterBinding::Raw { path_key: from });
    }

    let written = declared.to_token_stream().to_string();
    let model = resolve_struct(declared, registries.struct_resolution).ok_or_else(|| {
        HttpCodegenError::MissingHttpRouteParameterBinder {
            responder: responder.to_string(),
            parameter: from.clone(),
            written: written.clone(),
        }
    })?;
    let binder = registries.binders.get(&model).cloned().ok_or_else(|| {
        HttpCodegenError::MissingHttpRouteParameterBinder {
            responder: responder.to_string(),
            parameter: from.clone(),
            written,
        }
    })?;

    Ok(RouteParameterBinding::Bound {
        binder,
        path_key: from,
    })
}

fn is_string(declared: &Type) -> bool {
    matches!(declared, Type::Path(type_path) if type_path.path.is_ident("String"))
}

fn is_request(declared: &Type) -> bool {
    let Type::Reference(reference) = declared else {
        return false;
    };

    type_leaf_ident(&reference.elem).is_some_and(|ident| ident == "Request")
}

fn find_marker<'attribute>(
    attributes: &'attribute [Attribute],
    selector: &AttributeSelector,
) -> Option<&'attribute Attribute> {
    attributes
        .iter()
        .find(|attribute| selector.matches(attribute.path()))
}

fn find_responder_method(item: &IndexedItem) -> Option<&IndexedMethod> {
    item.method_matching(&AttributeSelector::parse("responder").expect("a valid selector"))
}

fn responder_output(method: &IndexedMethod, registries: &Registries) -> ResponderOutput {
    let ReturnType::Type(_, declared) = &method.signature().output else {
        return ResponderOutput::Plain;
    };

    let Some(intercepted) = resolve_trait(declared, registries.trait_resolution) else {
        return ResponderOutput::Plain;
    };

    match registries.interceptors.get(&intercepted) {
        Some(interceptor) => ResponderOutput::Intercepted {
            interceptor: interceptor.clone(),
        },
        None => ResponderOutput::Plain,
    }
}

pub(crate) fn responder_method(
    item: &IndexedItem,
    responder: &str,
    registries: &Registries,
) -> Result<ResponderSignature, HttpCodegenError> {
    let method =
        find_responder_method(item).ok_or_else(|| HttpCodegenError::MissingResponderMethod {
            responder: responder.to_string(),
        })?;
    let route_parameter_selector =
        AttributeSelector::parse("route_parameter").expect("a valid selector");
    let mut parameters = Vec::new();

    for (position, input) in method.signature().inputs.iter().enumerate() {
        let FnArg::Typed(pattern_type) = input else {
            continue;
        };

        let route_parameter = find_marker(&pattern_type.attrs, &route_parameter_selector);
        let is_current_request = is_request(&pattern_type.ty);

        if route_parameter.is_none() && !is_current_request {
            return Err(HttpCodegenError::UnmarkedResponderParameter {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        }

        let holder = match &*pattern_type.pat {
            Pat::Ident(pattern_ident) => pattern_ident.ident.clone(),
            _ => format_ident!("argument_{position}"),
        };
        let binding = if let Some(attribute) = route_parameter {
            let arguments = AttributeArgs::from_attribute(attribute)?;
            let route_arguments = RouteParameterArguments::parse(&arguments, responder, position)?;

            classify(responder, route_arguments, &pattern_type.ty, registries)?
        } else {
            RouteParameterBinding::CurrentRequest
        };

        parameters.push(RouteParameter { holder, binding });
    }

    Ok(ResponderSignature {
        output: responder_output(method, registries),
        parameters,
    })
}
