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
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::injectable_resolution::InjectableResolution;
use margaret_container::resolve_injectable::resolve_injectable;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;

use crate::binding_site::BindingSite;
use crate::bound_parameter::BoundParameter;
use crate::form_request_arguments::FormRequestArguments;
use crate::form_request_extraction::FormRequestExtraction;
use crate::request_binding::RequestBinding;
use crate::request_binding_error::RequestBindingError;
use crate::request_binding_policy::RequestBindingPolicy;
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

fn classify_route_parameter(
    index: &AttributeIndex,
    item: &IndexedItem,
    subject: &str,
    from: String,
    declared: &Type,
    binders: &HashMap<CanonicalPath, RouteParameterBinder>,
) -> Result<RequestBinding, RequestBindingError> {
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

fn classify_form_request(
    index: &AttributeIndex,
    item: &IndexedItem,
    attribute: &Attribute,
    declared: &Type,
    subject: &str,
    position: usize,
    policy: &RequestBindingPolicy,
) -> Result<RequestBinding, RequestBindingError> {
    let arguments = AttributeArgs::from_attribute(attribute)?;
    let FormRequestArguments { source } =
        FormRequestArguments::parse(&arguments, subject, position)?;

    if !policy.allows_body_form_requests {
        let unavailable = |written: &str| RequestBindingError::FormRequestBodyUnavailable {
            subject: subject.to_string(),
            parameter: position.to_string(),
            input_source: written.to_string(),
        };

        match source {
            RequestInputSource::Form => return Err(unavailable("Form")),
            RequestInputSource::Json => return Err(unavailable("Json")),
            RequestInputSource::Cookie | RequestInputSource::Query => {}
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

pub fn classify_parameters(
    index: &AttributeIndex,
    item: &IndexedItem,
    signature: &Signature,
    site: &BindingSite,
    binders: &HashMap<CanonicalPath, RouteParameterBinder>,
    container_bindings: Option<&ContainerBindings>,
    policy: &RequestBindingPolicy,
) -> Result<Vec<BoundParameter>, RequestBindingError> {
    let subject = site.subject;
    let server = site.server;
    let route_path = site.route_path;
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
        let is_current_request =
            RequestInjectable::CurrentRequest.matches(resolved.as_ref(), is_reference);
        let is_forwarder = is_forwarder(resolved.as_ref(), is_reference, server);
        let is_peer_spiffe_id =
            RequestInjectable::PeerSpiffeId.matches(resolved.as_ref(), is_reference);
        let is_routes = RequestInjectable::Routes.matches(resolved.as_ref(), is_reference);
        let is_views = RequestInjectable::Views.matches(resolved.as_ref(), is_reference);
        let is_asset_bag = RequestInjectable::AssetBag.matches(resolved.as_ref(), is_reference);

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
            classify_form_request(index, item, attribute, declared, subject, position, policy)?
        } else if let Some(attribute) = route_parameter {
            let arguments = AttributeArgs::from_attribute(attribute)?;
            let RouteParameterArguments { from } =
                RouteParameterArguments::parse(&arguments, subject, position)?;

            classify_route_parameter(index, item, subject, from, declared, binders)?
        } else if is_forwarder {
            if !policy.allows_forwarder {
                return Err(RequestBindingError::ForwarderUnavailable {
                    subject: subject.to_string(),
                    parameter: position.to_string(),
                });
            }

            RequestBinding::Forwarder
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
        } else if let Some(container_bindings) = container_bindings {
            match resolve_injectable(index, item, declared, container_bindings) {
                InjectableResolution::Resolved(dependency) => {
                    RequestBinding::Injectable { dependency }
                }
                InjectableResolution::UnsupportedShape => {
                    return Err(RequestBindingError::UnsupportedParameterShape {
                        subject: subject.to_string(),
                        parameter: position.to_string(),
                    });
                }
                InjectableResolution::MissingProvider => {
                    return Err(RequestBindingError::MissingProvider {
                        subject: subject.to_string(),
                        parameter: position.to_string(),
                    });
                }
            }
        } else {
            return Err(RequestBindingError::UnmarkedParameter {
                subject: subject.to_string(),
                parameter: position.to_string(),
            });
        };

        bound.push(BoundParameter { binding, holder });
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

    Ok(bound)
}
