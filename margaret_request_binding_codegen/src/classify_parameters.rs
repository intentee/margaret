use std::collections::HashMap;
use std::collections::HashSet;

use quote::ToTokens;
use syn::Type;
use syn::TypePath;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_attribute::IndexedAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::name_allocator::NameAllocator;
use margaret_attributes::tag::Tag;
use margaret_container::injectable_resolution::InjectableResolution;
use margaret_container::resolve_injectable::resolve_injectable;
use margaret_injection_codegen::optional_parameter::OptionalParameter;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;
use margaret_syn_type_peeling::single_generic_argument::single_generic_argument;
use margaret_tag_codegen::read_oidc_token_issuer::read_oidc_token_issuer;
use margaret_tag_codegen::tag_kind::TagKind;

use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::authenticated_user_provider::AuthenticatedUserProvider;
use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
use crate::binding_context::BindingContext;
use crate::binding_registries::BindingRegistries;
use crate::bound_parameter::BoundParameter;
use crate::form_request_arguments::FormRequestArguments;
use crate::form_request_extraction::FormRequestExtraction;
use crate::injects_views::injects_views;
use crate::oidc_token_verifier_field::OidcTokenVerifierField;
use crate::request_binding::RequestBinding;
use crate::request_binding_error::RequestBindingError;
use crate::request_injectable::RequestInjectable;
use crate::request_input_source::RequestInputSource;
use crate::route_parameter_arguments::RouteParameterArguments;
use crate::route_parameter_resolution::RouteParameterResolution;
use crate::views_availability::ViewsAvailability;

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

fn body_backed_source(provider: &AuthenticatedUserProvider) -> Option<String> {
    provider.parameters.iter().find_map(|parameter| {
        let RequestBinding::FormRequest { source, .. } = &parameter.binding else {
            return None;
        };

        match source {
            RequestInputSource::Form | RequestInputSource::Json => {
                Some(source.variant().to_string())
            }
            RequestInputSource::Cookie | RequestInputSource::Query => None,
        }
    })
}

fn classify_authenticated_user(
    index: &AttributeIndex,
    item: &IndexedItem,
    declared: &Type,
    context: &BindingContext,
    position: usize,
    providers: &HashMap<CanonicalPath, AuthenticatedUserProvider>,
) -> Result<RequestBinding, RequestBindingError> {
    let subject = context.subject();

    match context {
        BindingContext::AuthenticatedUserProvider { .. } | BindingContext::Middleware { .. } => {
            return Err(RequestBindingError::AuthenticatedUserUnavailable {
                subject: subject.to_string(),
                parameter: position.to_string(),
            });
        }
        BindingContext::Handshake { .. } | BindingContext::Responder { .. } => {}
    }

    let OptionalParameter {
        required,
        value_type,
    } = OptionalParameter::from_type(index, item, declared);

    if matches!(value_type, Type::Reference(_)) {
        return Err(RequestBindingError::AuthenticatedUserByReference {
            subject: subject.to_string(),
            parameter: position.to_string(),
        });
    }

    let model = index.resolve_item_type(item, &value_type).ok_or_else(|| {
        RequestBindingError::UnknownAuthenticatedUserModel {
            subject: subject.to_string(),
            parameter: position.to_string(),
            written: value_type.to_token_stream().to_string(),
        }
    })?;
    let provider = providers.get(&model).ok_or_else(|| {
        RequestBindingError::MissingAuthenticatedUserProvider {
            subject: subject.to_string(),
            parameter: position.to_string(),
            model: model.to_string(),
        }
    })?;

    if let BindingContext::Handshake { .. } = context {
        if let Some(source) = body_backed_source(provider) {
            return Err(RequestBindingError::AuthenticatedUserBodyUnavailable {
                subject: subject.to_string(),
                parameter: position.to_string(),
                provider: provider.application.concrete.to_string(),
                input_source: source,
            });
        }

        if injects_views(&provider.parameters) {
            return Err(RequestBindingError::AuthenticatedUserViewsUnavailable {
                subject: subject.to_string(),
                parameter: position.to_string(),
                provider: provider.application.concrete.to_string(),
            });
        }
    }

    Ok(RequestBinding::AuthenticatedUser {
        application: provider.application.clone(),
        requirement: if required {
            AuthenticatedUserRequirement::Required
        } else {
            AuthenticatedUserRequirement::Optional
        },
    })
}

fn classify_views(
    context: &BindingContext,
    position: usize,
    views: ViewsAvailability,
) -> Result<RequestBinding, RequestBindingError> {
    let subject = context.subject();

    if let BindingContext::Handshake { .. } = context {
        return Err(RequestBindingError::ViewsUnavailableInHandshake {
            subject: subject.to_string(),
            parameter: position.to_string(),
        });
    }

    match views {
        ViewsAvailability::Available => Ok(RequestBinding::Views),
        ViewsAvailability::Unavailable => Err(RequestBindingError::ViewsUnavailable {
            subject: subject.to_string(),
            parameter: position.to_string(),
        }),
    }
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
        BindingContext::AuthenticatedUserProvider { .. } => {
            Err(RequestBindingError::UnmarkedProviderParameter {
                subject: subject.to_string(),
                parameter: position.to_string(),
            })
        }
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
                InjectableResolution::JwksSecretStoreByPath => {
                    Err(RequestBindingError::JwksSecretStoreInjectedByPath {
                        subject: subject.to_string(),
                        parameter: position.to_string(),
                    })
                }
                InjectableResolution::FrameworkOnly => {
                    Err(RequestBindingError::FrameworkOnlyProviderInjected {
                        subject: subject.to_string(),
                        parameter: position.to_string(),
                    })
                }
            }
        }
        BindingContext::Middleware { .. } => {
            Err(RequestBindingError::UnmarkedMiddlewareParameter {
                subject: subject.to_string(),
                parameter: position.to_string(),
            })
        }
    }
}

fn classify_form_request(
    index: &AttributeIndex,
    item: &IndexedItem,
    attribute: &IndexedAttribute,
    declared: &Type,
    context: &BindingContext,
    position: usize,
) -> Result<RequestBinding, RequestBindingError> {
    let subject = context.subject();
    let arguments = attribute.args()?;
    let FormRequestArguments { source } =
        FormRequestArguments::parse(arguments, index, item, subject, position)?;

    if let BindingContext::Handshake { .. } = context {
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

fn classify_route_parameter(
    context: &BindingContext,
    attribute: &IndexedAttribute,
    declared: &Type,
    resolved: Option<&CanonicalPath>,
    position: usize,
    resolutions: &HashMap<CanonicalPath, RouteParameterResolution>,
    bound_route_parameters: &mut HashSet<String>,
) -> Result<RequestBinding, RequestBindingError> {
    let subject = context.subject();

    let route_path = match context {
        BindingContext::AuthenticatedUserProvider { .. } | BindingContext::Middleware { .. } => {
            return Err(RequestBindingError::RouteParameterUnavailable {
                subject: subject.to_string(),
                parameter: position.to_string(),
            });
        }
        BindingContext::Handshake { route_path, .. }
        | BindingContext::Responder { route_path, .. } => route_path,
    };

    let arguments = attribute.args()?;
    let RouteParameterArguments { from } =
        RouteParameterArguments::parse(arguments, subject, position)?;

    if !route_path.parameters().any(|name| name == from) {
        return Err(RequestBindingError::RouteParameterNotInPath {
            subject: subject.to_string(),
            parameter: from,
            path: route_path.pattern().to_string(),
        });
    }

    if !bound_route_parameters.insert(from.clone()) {
        return Err(RequestBindingError::MultipleRouteParameterBindings {
            subject: subject.to_string(),
            parameter: from,
        });
    }

    if matches!(declared, Type::Reference(_)) {
        return Err(RequestBindingError::RouteParameterByReference {
            subject: subject.to_string(),
            parameter: from,
        });
    }

    let missing = || RequestBindingError::MissingRouteParameterResolution {
        subject: subject.to_string(),
        parameter: from.clone(),
        written: declared.to_token_stream().to_string(),
    };

    match resolved.and_then(|value_type| resolutions.get(value_type)) {
        Some(RouteParameterResolution::Value) => {
            Ok(RequestBinding::RouteParameterValue { path_key: from })
        }
        Some(RouteParameterResolution::Binder(binder)) => Ok(RequestBinding::BoundRouteParameter {
            binder_field: binder.field.clone(),
            binder_provider: binder.provider.clone(),
            path_key: from,
        }),
        None => Err(missing()),
    }
}

fn verify_single_inference(
    bound: &[BoundParameter],
    subject: &str,
) -> Result<(), RequestBindingError> {
    let mut inferred: HashSet<&CanonicalPath> = HashSet::new();
    let mut bearer_inferences = 0_usize;

    for parameter in bound {
        let RequestBinding::AuthenticatedUser { application, .. } = &parameter.binding else {
            continue;
        };

        if !inferred.insert(&application.model) {
            return Err(RequestBindingError::MultipleAuthenticatedUserParameters {
                subject: subject.to_string(),
                model: application.model.to_string(),
            });
        }

        if application.challenge == AuthenticatedUserChallenge::Bearer {
            bearer_inferences += 1;
        }
    }

    if bearer_inferences > 1 {
        return Err(RequestBindingError::MultipleBearerAuthenticatedUsers {
            subject: subject.to_string(),
        });
    }

    Ok(())
}

fn verify_single_request_parameters(
    bound: &[BoundParameter],
    subject: &str,
) -> Result<(), RequestBindingError> {
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

    verify_single_inference(bound, subject)
}

fn oidc_token_claims_path(claims: &Type) -> Option<&TypePath> {
    let Type::Path(claims_path) = claims else {
        return None;
    };

    (claims_path.qself.is_none()
        && claims_path
            .path
            .segments
            .iter()
            .all(|segment| segment.arguments.is_none()))
    .then_some(claims_path)
}

fn classify_oidc_token(
    index: &AttributeIndex,
    item: &IndexedItem,
    attribute: &IndexedAttribute,
    declared: &Type,
    context: &BindingContext,
    position: usize,
    bound: &mut BoundOidcTokens,
) -> Result<RequestBinding, RequestBindingError> {
    let subject = context.subject();
    let BindingContext::AuthenticatedUserProvider {
        container_bindings,
        tags,
        ..
    } = context
    else {
        return Err(RequestBindingError::OidcTokenUnavailable {
            subject: subject.to_string(),
            parameter: position.to_string(),
        });
    };
    let site = format!("argument #{position} of {subject}");
    let issuer = read_oidc_token_issuer(attribute.args()?, &site)?;

    tags.resolve(&issuer, TagKind::OidcIssuer, &site)?;

    let written = || declared.to_token_stream().to_string();
    let claims = match declared {
        Type::Path(declared_path)
            if RequestInjectable::OidcTokenVerification
                .matches(index.resolve_item_type(item, declared).as_ref(), false) =>
        {
            declared_path
                .path
                .segments
                .last()
                .and_then(single_generic_argument)
        }
        _ => None,
    }
    .ok_or_else(|| RequestBindingError::OidcTokenTypeMismatch {
        subject: subject.to_string(),
        parameter: position.to_string(),
        written: written(),
    })?;
    let claims_written = || claims.to_token_stream().to_string();
    let claims_path = oidc_token_claims_path(claims).ok_or_else(|| {
        RequestBindingError::UnsupportedOidcTokenClaims {
            subject: subject.to_string(),
            parameter: position.to_string(),
            written: claims_written(),
        }
    })?;
    let claims = index
        .resolve_item_path(item, &claims_path.path)
        .ok_or_else(|| RequestBindingError::UnknownOidcTokenClaims {
            subject: subject.to_string(),
            parameter: position.to_string(),
            written: claims_written(),
        })?;

    if !bound.issuers.insert(issuer.clone()) {
        return Err(RequestBindingError::MultipleOidcTokenParameters {
            subject: subject.to_string(),
            issuer: issuer.to_string(),
        });
    }

    let client = container_bindings.oidc_client(&issuer).ok_or_else(|| {
        RequestBindingError::UnplannedOidcClient {
            subject: subject.to_string(),
            parameter: position.to_string(),
            issuer: issuer.to_string(),
        }
    })?;

    Ok(RequestBinding::OidcToken {
        claims,
        verifier: OidcTokenVerifierField {
            client,
            field: bound
                .verifier_fields
                .allocate("oidc_token_verifier")
                .field()
                .to_string(),
        },
    })
}

fn parameter_marker<'marker>(
    attributes: &'marker [IndexedAttribute],
    subject: &str,
    position: usize,
    is_peer_spiffe_id: bool,
) -> Result<ParameterMarker<'marker>, RequestBindingError> {
    let marker = |attribute: FrameworkAttribute| {
        attributes
            .iter()
            .find(|candidate| candidate.framework_attribute() == Some(attribute))
    };
    let authenticated_user = marker(FrameworkAttribute::AuthenticatedUser);
    let form_request = marker(FrameworkAttribute::FormRequest);
    let oidc_token = marker(FrameworkAttribute::OidcToken);
    let route_parameter = marker(FrameworkAttribute::RouteParameter);

    if route_parameter.is_some() && form_request.is_some() {
        return Err(RequestBindingError::ConflictingArgumentMarkers {
            subject: subject.to_string(),
            parameter: position.to_string(),
        });
    }

    if authenticated_user.is_some() && (route_parameter.is_some() || form_request.is_some()) {
        return Err(RequestBindingError::ConflictingAuthenticatedUserMarkers {
            subject: subject.to_string(),
            parameter: position.to_string(),
        });
    }

    if oidc_token.is_some()
        && (authenticated_user.is_some() || route_parameter.is_some() || form_request.is_some())
    {
        return Err(RequestBindingError::ConflictingOidcTokenMarkers {
            subject: subject.to_string(),
            parameter: position.to_string(),
        });
    }

    if is_peer_spiffe_id
        && (authenticated_user.is_some()
            || route_parameter.is_some()
            || form_request.is_some()
            || oidc_token.is_some())
    {
        return Err(RequestBindingError::MarkedPeerSpiffeIdParameter {
            subject: subject.to_string(),
            parameter: position.to_string(),
        });
    }

    Ok(
        match (
            authenticated_user,
            form_request,
            oidc_token,
            route_parameter,
        ) {
            (Some(_), _, _, _) => ParameterMarker::AuthenticatedUser,
            (None, Some(attribute), _, _) => ParameterMarker::FormRequest(attribute),
            (None, None, Some(attribute), _) => ParameterMarker::OidcToken(attribute),
            (None, None, None, Some(attribute)) => ParameterMarker::RouteParameter(attribute),
            (None, None, None, None) => ParameterMarker::Unmarked,
        },
    )
}

#[derive(Default)]
struct BoundOidcTokens {
    issuers: HashSet<Tag>,
    verifier_fields: NameAllocator,
}

enum ParameterMarker<'marker> {
    AuthenticatedUser,
    FormRequest(&'marker IndexedAttribute),
    OidcToken(&'marker IndexedAttribute),
    RouteParameter(&'marker IndexedAttribute),
    Unmarked,
}

/// # Errors
///
/// Returns `RequestBindingError` propagated from the work it performs.
pub fn classify_parameters(
    index: &AttributeIndex,
    item: &IndexedItem,
    method: &IndexedMethod,
    context: &BindingContext,
    registries: &BindingRegistries,
) -> Result<Vec<BoundParameter>, RequestBindingError> {
    let subject = context.subject();
    let mut bound = Vec::new();
    let mut bound_oidc_tokens = BoundOidcTokens::default();
    let mut bound_route_parameters = HashSet::new();

    for ParameterView {
        attributes,
        declared,
        holder,
        position,
    } in parameters(method)
    {
        let resolved = index.resolve_item_type(item, declared);
        let is_reference = matches!(declared, Type::Reference(_));
        let injectable = RequestInjectable::resolve(resolved.as_ref(), is_reference);
        let is_peer_spiffe_id = matches!(injectable, Some(RequestInjectable::PeerSpiffeId));
        let binding = match parameter_marker(attributes, subject, position, is_peer_spiffe_id)? {
            ParameterMarker::AuthenticatedUser => classify_authenticated_user(
                index,
                item,
                declared,
                context,
                position,
                &registries.authenticated_users,
            )?,
            ParameterMarker::FormRequest(attribute) => {
                classify_form_request(index, item, attribute, declared, context, position)?
            }
            ParameterMarker::OidcToken(attribute) => classify_oidc_token(
                index,
                item,
                attribute,
                declared,
                context,
                position,
                &mut bound_oidc_tokens,
            )?,
            ParameterMarker::RouteParameter(attribute) => classify_route_parameter(
                context,
                attribute,
                declared,
                resolved.as_ref(),
                position,
                &registries.route_parameters,
                &mut bound_route_parameters,
            )?,
            ParameterMarker::Unmarked => {
                if matches!(injectable, Some(RequestInjectable::Next)) {
                    match context {
                        BindingContext::Middleware { .. } => RequestBinding::Next,
                        BindingContext::AuthenticatedUserProvider { .. }
                        | BindingContext::Handshake { .. }
                        | BindingContext::Responder { .. } => {
                            return Err(RequestBindingError::NextOutsideMiddleware {
                                subject: subject.to_string(),
                                parameter: position.to_string(),
                            });
                        }
                    }
                } else if is_peer_spiffe_id {
                    RequestBinding::PeerSpiffeId
                } else if matches!(injectable, Some(RequestInjectable::Routes)) {
                    RequestBinding::Routes
                } else if matches!(injectable, Some(RequestInjectable::Views)) {
                    classify_views(context, position, registries.views)?
                } else if matches!(injectable, Some(RequestInjectable::AssetBag)) {
                    RequestBinding::AssetBag
                } else if matches!(injectable, Some(RequestInjectable::CurrentRequest)) {
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
                }
            }
        };

        bound.push(BoundParameter { binding, holder });
    }

    verify_single_request_parameters(&bound, subject)?;

    Ok(bound)
}
