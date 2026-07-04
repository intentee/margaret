use quote::ToTokens;
use syn::Attribute;
use syn::ReturnType;
use syn::Type;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::resolve_struct::resolve_struct;
use margaret_attributes::resolve_trait::resolve_trait;
use margaret_injection_codegen::leaf_matches::leaf_matches;
use margaret_injection_codegen::marker::marker;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;
use margaret_injection_codegen::reference_leaf_matches::reference_leaf_matches;

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
    let extraction = if leaf_matches(declared, "ValidationResult") {
        FormRequestExtraction::Result
    } else {
        FormRequestExtraction::Model
    };

    Ok(ResponderArgumentBinding::FormRequest { source, extraction })
}

fn is_string(declared: &Type) -> bool {
    matches!(declared, Type::Path(type_path) if type_path.path.is_ident("String"))
}

fn responder_output(method: &IndexedMethod, registries: &Registries) -> ResponderOutput {
    let ReturnType::Type(_, declared) = &method.signature().output else {
        return ResponderOutput::Plain;
    };

    let Some(intercepted) = resolve_trait(declared, registries.trait_resolution) else {
        return ResponderOutput::Plain;
    };

    match registries.interceptors.get(&intercepted) {
        Some(reference) => ResponderOutput::Intercepted {
            interceptor: reference.interceptor.clone(),
            injects_routes: reference.injects_routes,
        },
        None => ResponderOutput::Plain,
    }
}

pub(crate) fn responder_method(
    item: &IndexedItem,
    responder: &str,
    registries: &Registries,
) -> Result<ResponderSignature, HttpCodegenError> {
    let method = process_method(item)?;
    let route_parameter_selector =
        AttributeSelector::parse("route_parameter").expect("a valid selector");
    let form_request_selector = AttributeSelector::parse("form_request").expect("a valid selector");
    let mut arguments = Vec::new();

    for ParameterView {
        attributes,
        declared,
        holder,
        position,
    } in parameters(method.signature())
    {
        let route_parameter = marker(attributes, &route_parameter_selector);
        let form_request = marker(attributes, &form_request_selector);
        let is_current_request = reference_leaf_matches(declared, "Request");
        let is_routes = reference_leaf_matches(declared, "Routes");

        if route_parameter.is_some() && form_request.is_some() {
            return Err(HttpCodegenError::ConflictingArgumentMarkers {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        }

        if route_parameter.is_none() && form_request.is_none() && !is_current_request && !is_routes
        {
            return Err(HttpCodegenError::UnmarkedResponderParameter {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        }

        let binding = if let Some(attribute) = form_request {
            classify_form_request(attribute, declared, responder, position)?
        } else if let Some(attribute) = route_parameter {
            let arguments = AttributeArgs::from_attribute(attribute)?;
            let route_arguments = RouteParameterArguments::parse(&arguments, responder, position)?;

            classify(responder, route_arguments, declared, registries)?
        } else if is_routes {
            ResponderArgumentBinding::Routes
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
