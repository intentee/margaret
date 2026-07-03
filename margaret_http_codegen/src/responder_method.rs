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

use crate::form_request_arguments::FormRequestArguments;
use crate::form_request_extraction::FormRequestExtraction;
use crate::http_codegen_error::HttpCodegenError;
use crate::registries::Registries;
use crate::responder_argument::ResponderArgument;
use crate::responder_argument_binding::ResponderArgumentBinding;
use crate::responder_output::ResponderOutput;
use crate::responder_signature::ResponderSignature;
use crate::route_parameter_arguments::RouteParameterArguments;

fn classify(
    responder: &str,
    arguments: RouteParameterArguments,
    declared: &Type,
    registries: &Registries,
) -> Result<ResponderArgumentBinding, HttpCodegenError> {
    let RouteParameterArguments { from } = arguments;

    if is_string(declared) {
        return Ok(ResponderArgumentBinding::Raw { path_key: from });
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

    Ok(ResponderArgumentBinding::Bound {
        binder,
        path_key: from,
    })
}

fn classify_form_request(
    attribute: &Attribute,
    declared: &Type,
    responder: &str,
    position: usize,
) -> Result<ResponderArgumentBinding, HttpCodegenError> {
    let arguments = AttributeArgs::from_attribute(attribute)?;
    let FormRequestArguments { source } =
        FormRequestArguments::parse(&arguments, responder, position)?;
    let extraction = if is_validation_result(declared) {
        FormRequestExtraction::Result
    } else {
        FormRequestExtraction::Model
    };

    Ok(ResponderArgumentBinding::FormRequest { source, extraction })
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

fn is_validation_result(declared: &Type) -> bool {
    type_leaf_ident(declared).is_some_and(|ident| ident == "ValidationResult")
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
    let form_request_selector = AttributeSelector::parse("form_request").expect("a valid selector");
    let mut arguments = Vec::new();

    for (position, input) in method.signature().inputs.iter().enumerate() {
        let FnArg::Typed(pattern_type) = input else {
            continue;
        };

        let route_parameter = find_marker(&pattern_type.attrs, &route_parameter_selector);
        let form_request = find_marker(&pattern_type.attrs, &form_request_selector);
        let is_current_request = is_request(&pattern_type.ty);

        if route_parameter.is_some() && form_request.is_some() {
            return Err(HttpCodegenError::ConflictingArgumentMarkers {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        }

        if route_parameter.is_none() && form_request.is_none() && !is_current_request {
            return Err(HttpCodegenError::UnmarkedResponderParameter {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        }

        let holder = match &*pattern_type.pat {
            Pat::Ident(pattern_ident) => pattern_ident.ident.clone(),
            _ => format_ident!("argument_{position}"),
        };
        let binding = if let Some(attribute) = form_request {
            classify_form_request(attribute, &pattern_type.ty, responder, position)?
        } else if let Some(attribute) = route_parameter {
            let arguments = AttributeArgs::from_attribute(attribute)?;
            let route_arguments = RouteParameterArguments::parse(&arguments, responder, position)?;

            classify(responder, route_arguments, &pattern_type.ty, registries)?
        } else {
            ResponderArgumentBinding::CurrentRequest
        };

        arguments.push(ResponderArgument { holder, binding });
    }

    Ok(ResponderSignature {
        output: responder_output(method, registries),
        arguments,
    })
}
