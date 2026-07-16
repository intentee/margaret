use std::collections::HashMap;

use quote::ToTokens;
use syn::Attribute;
use syn::Type;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::marker::marker;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;

use crate::form_request_arguments::FormRequestArguments;
use crate::form_request_extraction::FormRequestExtraction;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_injectable::HttpInjectable;
use crate::responder_argument::ResponderArgument;
use crate::responder_argument_binding::ResponderArgumentBinding;
use crate::responder_selectors::ResponderSelectors;
use crate::route_parameter_arguments::RouteParameterArguments;

fn classify(
    index: &AttributeIndex,
    item: &IndexedItem,
    responder: &str,
    arguments: RouteParameterArguments,
    declared: &Type,
    binders: &HashMap<CanonicalPath, CanonicalPath>,
) -> Result<ResponderArgumentBinding, HttpCodegenError> {
    let RouteParameterArguments { from } = arguments;
    let resolved = index.resolve_item_type(item, declared);

    if resolved.as_ref() == Some(&string_path()) {
        return Ok(ResponderArgumentBinding::Raw { path_key: from });
    }

    let written = declared.to_token_stream().to_string();
    let model = resolved.ok_or_else(|| HttpCodegenError::MissingHttpRouteParameterBinder {
        responder: responder.to_string(),
        parameter: from.clone(),
        written: written.clone(),
    })?;
    let binder = binders.get(&model).cloned().ok_or_else(|| {
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
    index: &AttributeIndex,
    item: &IndexedItem,
    attribute: &Attribute,
    declared: &Type,
    responder: &str,
    position: usize,
) -> Result<ResponderArgumentBinding, HttpCodegenError> {
    let arguments = AttributeArgs::from_attribute(attribute)?;
    let FormRequestArguments { source } =
        FormRequestArguments::parse(&arguments, responder, position)?;
    let resolved = index.resolve_item_type(item, declared);
    let is_reference = matches!(declared, Type::Reference(_));
    let extraction = if HttpInjectable::ValidationResult.matches(resolved.as_ref(), is_reference) {
        FormRequestExtraction::Result
    } else {
        FormRequestExtraction::Model
    };

    Ok(ResponderArgumentBinding::FormRequest { source, extraction })
}

fn forwarder_path(server: &str) -> CanonicalPath {
    CanonicalPath::new(vec![
        "crate".to_string(),
        "margaret".to_string(),
        "forwarders".to_string(),
        server.to_string(),
        "Forwarder".to_string(),
    ])
}

fn is_forwarder(resolved: Option<&CanonicalPath>, is_reference: bool, server: &str) -> bool {
    resolved == Some(&forwarder_path(server)) && !is_reference
}

fn string_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "std".to_string(),
        "string".to_string(),
        "String".to_string(),
    ])
}

pub(crate) fn responder_method(
    index: &AttributeIndex,
    item: &IndexedItem,
    responder: &str,
    server: &str,
    binders: &HashMap<CanonicalPath, CanonicalPath>,
    selectors: &ResponderSelectors,
) -> Result<Vec<ResponderArgument>, HttpCodegenError> {
    let method = process_method(item)?;
    let mut arguments = Vec::new();

    for ParameterView {
        attributes,
        declared,
        holder,
        position,
    } in parameters(method.signature())
    {
        let route_parameter = marker(attributes, &selectors.route_parameter);
        let form_request = marker(attributes, &selectors.form_request);
        let resolved = index.resolve_item_type(item, declared);
        let is_reference = matches!(declared, Type::Reference(_));
        let is_current_request =
            HttpInjectable::CurrentRequest.matches(resolved.as_ref(), is_reference);
        let is_forwarder = is_forwarder(resolved.as_ref(), is_reference, server);
        let is_peer_spiffe_id =
            HttpInjectable::PeerSpiffeId.matches(resolved.as_ref(), is_reference);
        let is_routes = HttpInjectable::Routes.matches(resolved.as_ref(), is_reference);

        if route_parameter.is_some() && form_request.is_some() {
            return Err(HttpCodegenError::ConflictingArgumentMarkers {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        }

        if is_peer_spiffe_id && (route_parameter.is_some() || form_request.is_some()) {
            return Err(HttpCodegenError::MarkedPeerSpiffeIdParameter {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        }

        if route_parameter.is_none()
            && form_request.is_none()
            && !is_current_request
            && !is_forwarder
            && !is_peer_spiffe_id
            && !is_routes
        {
            return Err(HttpCodegenError::UnmarkedResponderParameter {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        }

        let binding = if let Some(attribute) = form_request {
            classify_form_request(index, item, attribute, declared, responder, position)?
        } else if let Some(attribute) = route_parameter {
            let arguments = AttributeArgs::from_attribute(attribute)?;
            let route_arguments = RouteParameterArguments::parse(&arguments, responder, position)?;

            classify(index, item, responder, route_arguments, declared, binders)?
        } else if is_forwarder {
            ResponderArgumentBinding::Forwarder
        } else if is_peer_spiffe_id {
            ResponderArgumentBinding::PeerSpiffeId
        } else if is_routes {
            ResponderArgumentBinding::Routes
        } else {
            ResponderArgumentBinding::CurrentRequest
        };

        arguments.push(ResponderArgument { holder, binding });
    }

    let peer_spiffe_id_count = arguments
        .iter()
        .filter(|argument| matches!(argument.binding, ResponderArgumentBinding::PeerSpiffeId))
        .count();

    if peer_spiffe_id_count > 1 {
        return Err(HttpCodegenError::MultiplePeerSpiffeIdParameters {
            responder: responder.to_string(),
        });
    }

    Ok(arguments)
}
