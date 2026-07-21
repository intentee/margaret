use std::collections::HashMap;

use quote::ToTokens;
use syn::Attribute;
use syn::Signature;
use syn::Type;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::marker::marker;
use margaret_container::injectable_resolution::InjectableResolution;
use margaret_container::resolve_injectable::resolve_injectable;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;

use crate::binding_context::BindingContext;
use crate::bound_parameter::BoundParameter;
use crate::form_request_arguments::FormRequestArguments;
use crate::form_request_extraction::FormRequestExtraction;
use crate::request_binding::RequestBinding;
use crate::request_binding_error::RequestBindingError;
use crate::request_injectable::RequestInjectable;
use crate::request_input_source::RequestInputSource;
use crate::route_parameter_arguments::RouteParameterArguments;
use crate::route_parameter_binder::RouteParameterBinder;

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

fn classify_context_specific(
    index: &AttributeIndex,
    item: &IndexedItem,
    declared: &Type,
    resolved: Option<&CanonicalPath>,
    is_reference: bool,
    context: &BindingContext,
    position: usize,
) -> Result<RequestBinding, RequestBindingError> {
    let subject = context.subject();

    match context {
        BindingContext::Responder { server, .. } => {
            if is_forwarder(resolved, is_reference, server) {
                Ok(RequestBinding::Forwarder)
            } else {
                Err(RequestBindingError::UnmarkedParameter {
                    subject: subject.to_string(),
                    parameter: position.to_string(),
                })
            }
        }
        BindingContext::Handshake {
            container_bindings,
            server,
            ..
        } => {
            if is_forwarder(resolved, is_reference, server) {
                return Err(RequestBindingError::ForwarderUnavailable {
                    subject: subject.to_string(),
                    parameter: position.to_string(),
                });
            }

            match resolve_injectable(index, item, declared, container_bindings) {
                InjectableResolution::Resolved(dependency) => {
                    Ok(RequestBinding::Injectable { dependency })
                }
                InjectableResolution::UnsupportedShape => {
                    Err(RequestBindingError::UnsupportedParameterShape {
                        subject: subject.to_string(),
                        parameter: position.to_string(),
                    })
                }
                InjectableResolution::MissingProvider => {
                    Err(RequestBindingError::MissingProvider {
                        subject: subject.to_string(),
                        parameter: position.to_string(),
                    })
                }
            }
        }
        BindingContext::Middleware { .. } => Err(RequestBindingError::UnmarkedMiddlewareParameter {
            subject: subject.to_string(),
            parameter: position.to_string(),
        }),
    }
}

fn classify_form_request(
    index: &AttributeIndex,
    item: &IndexedItem,
    attribute: &Attribute,
    declared: &Type,
    context: &BindingContext,
    position: usize,
) -> Result<RequestBinding, RequestBindingError> {
    let subject = context.subject();
    let arguments = AttributeArgs::from_attribute(attribute)?;
    let FormRequestArguments { source } =
        FormRequestArguments::parse(&arguments, subject, position)?;

    if let BindingContext::Handshake { .. } = context {
        let unavailable = |written: &str| RequestBindingError::FormRequestBodyUnavailable {
            subject: subject.to_string(),
            parameter: position.to_string(),
            input_source: written.to_string(),
        };

        match source {
            RequestInputSource::Form => return Err(unavailable("Form")),
            RequestInputSource::Json => return Err(unavailable("Json")),
            RequestInputSource::Query => {}
        }
    }

    let resolved = index.resolve_item_type(item, declared);
    let is_reference = matches!(declared, Type::Reference(_));
    let extraction = if RequestInjectable::ValidationResult.matches(resolved.as_ref(), is_reference)
    {
        FormRequestExtraction::Result
    } else {
        FormRequestExtraction::Model
    };

    Ok(RequestBinding::FormRequest { source, extraction })
}

fn classify_route_parameter(
    index: &AttributeIndex,
    item: &IndexedItem,
    context: &BindingContext,
    attribute: &Attribute,
    declared: &Type,
    position: usize,
    binders: &HashMap<CanonicalPath, RouteParameterBinder>,
) -> Result<RequestBinding, RequestBindingError> {
    let subject = context.subject();

    if let BindingContext::Middleware { .. } = context {
        return Err(RequestBindingError::RouteParameterUnavailable {
            subject: subject.to_string(),
            parameter: position.to_string(),
        });
    }

    let arguments = AttributeArgs::from_attribute(attribute)?;
    let RouteParameterArguments { from } =
        RouteParameterArguments::parse(&arguments, subject, position)?;
    let resolved = index.resolve_item_type(item, declared);

    if resolved.as_ref() == Some(&string_path()) {
        return Ok(RequestBinding::Raw { path_key: from });
    }

    let written = declared.to_token_stream().to_string();
    let model = resolved.ok_or_else(|| RequestBindingError::MissingRouteParameterBinder {
        subject: subject.to_string(),
        parameter: from.clone(),
        written: written.clone(),
    })?;
    let binder =
        binders
            .get(&model)
            .ok_or_else(|| RequestBindingError::MissingRouteParameterBinder {
                subject: subject.to_string(),
                parameter: from.clone(),
                written,
            })?;

    Ok(RequestBinding::Bound {
        binder_field: binder.field.clone(),
        binder_provider: binder.provider.clone(),
        path_key: from,
    })
}

pub fn classify_parameters(
    index: &AttributeIndex,
    item: &IndexedItem,
    signature: &Signature,
    context: &BindingContext,
    binders: &HashMap<CanonicalPath, RouteParameterBinder>,
) -> Result<Vec<BoundParameter>, RequestBindingError> {
    let subject = context.subject();
    let route_parameter_selector = AttributeSelector::from_marker("route_parameter");
    let form_request_selector = AttributeSelector::from_marker("form_request");
    let mut bound = Vec::new();

    for ParameterView {
        attributes,
        declared,
        holder,
        position,
    } in parameters(signature)
    {
        let route_parameter = marker(attributes, &route_parameter_selector);
        let form_request = marker(attributes, &form_request_selector);
        let resolved = index.resolve_item_type(item, declared);
        let is_reference = matches!(declared, Type::Reference(_));
        let is_asset_bag = RequestInjectable::AssetBag.matches(resolved.as_ref(), is_reference);
        let is_current_request =
            RequestInjectable::CurrentRequest.matches(resolved.as_ref(), is_reference);
        let is_next = RequestInjectable::Next.matches(resolved.as_ref(), is_reference);
        let is_peer_spiffe_id =
            RequestInjectable::PeerSpiffeId.matches(resolved.as_ref(), is_reference);
        let is_routes = RequestInjectable::Routes.matches(resolved.as_ref(), is_reference);
        let is_views = RequestInjectable::Views.matches(resolved.as_ref(), is_reference);

        if route_parameter.is_some() && form_request.is_some() {
            return Err(RequestBindingError::ConflictingArgumentMarkers {
                subject: subject.to_string(),
                parameter: position.to_string(),
            });
        }

        if is_peer_spiffe_id && (route_parameter.is_some() || form_request.is_some()) {
            return Err(RequestBindingError::MarkedPeerSpiffeIdParameter {
                subject: subject.to_string(),
                parameter: position.to_string(),
            });
        }

        let binding = if let Some(attribute) = form_request {
            classify_form_request(index, item, attribute, declared, context, position)?
        } else if let Some(attribute) = route_parameter {
            classify_route_parameter(index, item, context, attribute, declared, position, binders)?
        } else if is_next {
            match context {
                BindingContext::Middleware { .. } => RequestBinding::Next,
                BindingContext::Handshake { .. } | BindingContext::Responder { .. } => {
                    return Err(RequestBindingError::NextOutsideMiddleware {
                        subject: subject.to_string(),
                        parameter: position.to_string(),
                    });
                }
            }
        } else if is_peer_spiffe_id {
            RequestBinding::PeerSpiffeId
        } else if is_routes {
            RequestBinding::Routes
        } else if is_views {
            RequestBinding::Views
        } else if is_asset_bag {
            RequestBinding::AssetBag
        } else if is_current_request {
            RequestBinding::CurrentRequest
        } else {
            classify_context_specific(
                index,
                item,
                declared,
                resolved.as_ref(),
                is_reference,
                context,
                position,
            )?
        };

        bound.push(BoundParameter { binding, holder });
    }

    let next_count = bound
        .iter()
        .filter(|parameter| matches!(parameter.binding, RequestBinding::Next))
        .count();

    if next_count > 1 {
        return Err(RequestBindingError::MultipleNextParameters {
            subject: subject.to_string(),
        });
    }

    let peer_spiffe_id_count = bound
        .iter()
        .filter(|parameter| matches!(parameter.binding, RequestBinding::PeerSpiffeId))
        .count();

    if peer_spiffe_id_count > 1 {
        return Err(RequestBindingError::MultiplePeerSpiffeIdParameters {
            subject: subject.to_string(),
        });
    }

    match context {
        BindingContext::Handshake { route_path, .. }
        | BindingContext::Responder { route_path, .. } => {
            for parameter in &bound {
                if let Some(path_key) = parameter.binding.path_key()
                    && !route_path.parameters().any(|name| name == path_key)
                {
                    return Err(RequestBindingError::RouteParameterNotInPath {
                        subject: subject.to_string(),
                        parameter: path_key.to_string(),
                        path: route_path.pattern().to_string(),
                    });
                }
            }
        }
        BindingContext::Middleware { .. } => {}
    }

    Ok(bound)
}
