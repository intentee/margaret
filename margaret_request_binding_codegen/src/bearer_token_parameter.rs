use quote::ToTokens;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_attribute::IndexedAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_container::container_bindings::ContainerBindings;
use margaret_injection_codegen::optional_parameter::OptionalParameter;
use margaret_syn_type_peeling::generic_argument_pair::GenericArgumentPair;
use margaret_syn_type_peeling::paired_generic_arguments::paired_generic_arguments;
use margaret_syn_type_peeling::single_generic_argument::single_generic_argument;
use margaret_tag_codegen::read_bearer_token_client::read_bearer_token_client;
use margaret_tag_codegen::read_bearer_token_issuer::read_bearer_token_issuer;
use margaret_tag_codegen::tag_expectation::TagExpectation;
use margaret_tag_codegen::tag_pool::TagPool;

use crate::bound_bearer_tokens::BoundBearerTokens;
use crate::plain_type_resolution::PlainTypeResolution;
use crate::request_binding::RequestBinding;
use crate::request_binding_error::RequestBindingError;
use crate::request_injectable::RequestInjectable;

pub(crate) struct BearerTokenParameter<'classified> {
    pub(crate) attribute: &'classified IndexedAttribute,
    pub(crate) container_bindings: &'classified ContainerBindings,
    pub(crate) index: &'classified AttributeIndex,
    pub(crate) item: &'classified IndexedItem,
    pub(crate) position: usize,
    pub(crate) subject: &'classified str,
    pub(crate) tags: &'classified TagPool<'classified>,
}

impl BearerTokenParameter<'_> {
    pub(crate) fn classify(
        &self,
        declared: &Type,
        bound_bearer_tokens: &mut BoundBearerTokens,
    ) -> Result<RequestBinding, RequestBindingError> {
        let OptionalParameter {
            required,
            value_type,
        } = OptionalParameter::from_type(self.index, self.item, declared);
        let carrier_segment = match &value_type {
            Type::Path(carrier) if !required => carrier.path.segments.last(),
            _ => None,
        };
        let carrier = self.index.resolve_item_type(self.item, &value_type);

        match carrier_segment {
            Some(segment) if RequestInjectable::VerifiedJwt.matches(carrier.as_ref(), false) => {
                if let Some(pair) = paired_generic_arguments(segment) {
                    return self.verified_jwt(pair, bound_bearer_tokens);
                }
            }
            Some(segment)
                if RequestInjectable::IntrospectedToken.matches(carrier.as_ref(), false) =>
            {
                if let Some(claims) = single_generic_argument(segment) {
                    return self.introspected_token(claims, bound_bearer_tokens);
                }
            }
            Some(_) | None => {}
        }

        Err(RequestBindingError::BearerTokenTypeMismatch {
            subject: self.subject.to_string(),
            parameter: self.position.to_string(),
            written: declared.to_token_stream().to_string(),
        })
    }

    fn claims(&self, claims: &Type) -> Result<CanonicalPath, RequestBindingError> {
        match PlainTypeResolution::of(self.index, self.item, claims) {
            PlainTypeResolution::NotPlain => {
                Err(RequestBindingError::UnsupportedBearerTokenClaims {
                    subject: self.subject.to_string(),
                    parameter: self.position.to_string(),
                    written: claims.to_token_stream().to_string(),
                })
            }
            PlainTypeResolution::Resolved(claims) => Ok(claims),
            PlainTypeResolution::Unknown => Err(RequestBindingError::UnknownBearerTokenClaims {
                subject: self.subject.to_string(),
                parameter: self.position.to_string(),
                written: claims.to_token_stream().to_string(),
            }),
        }
    }

    fn introspected_token(
        &self,
        claims: &Type,
        bound_bearer_tokens: &mut BoundBearerTokens,
    ) -> Result<RequestBinding, RequestBindingError> {
        let site = self.site();
        let client = read_bearer_token_client(self.attribute.args()?, &site)?;

        self.tags
            .resolve(&client, TagExpectation::OAuthClient, &site)?;

        let claims = self.claims(claims)?;

        bound_bearer_tokens.bind_introspected(self.subject)?;

        let authorization_server =
            self.container_bindings
                .oauth_client(&client)
                .ok_or_else(|| RequestBindingError::UnplannedOAuthClient {
                    subject: self.subject.to_string(),
                    parameter: self.position.to_string(),
                    client: client.to_string(),
                })?;

        Ok(RequestBinding::IntrospectedBearerToken {
            authorization_server,
            claims,
        })
    }

    fn profile(&self, profile: &Type) -> Result<CanonicalPath, RequestBindingError> {
        match PlainTypeResolution::of(self.index, self.item, profile) {
            PlainTypeResolution::NotPlain => {
                Err(RequestBindingError::UnsupportedBearerTokenProfile {
                    subject: self.subject.to_string(),
                    parameter: self.position.to_string(),
                    written: profile.to_token_stream().to_string(),
                })
            }
            PlainTypeResolution::Resolved(profile) => Ok(profile),
            PlainTypeResolution::Unknown => Err(RequestBindingError::UnknownBearerTokenProfile {
                subject: self.subject.to_string(),
                parameter: self.position.to_string(),
                written: profile.to_token_stream().to_string(),
            }),
        }
    }

    fn site(&self) -> String {
        format!("argument #{} of {}", self.position, self.subject)
    }

    fn verified_jwt(
        &self,
        GenericArgumentPair {
            first: claims,
            second: profile,
        }: GenericArgumentPair,
        bound_bearer_tokens: &mut BoundBearerTokens,
    ) -> Result<RequestBinding, RequestBindingError> {
        let site = self.site();
        let issuer = read_bearer_token_issuer(self.attribute.args()?, &site)?;

        self.tags
            .resolve(&issuer, TagExpectation::TrustedIssuer, &site)?;

        let claims = self.claims(claims)?;
        let profile = self.profile(profile)?;

        bound_bearer_tokens.bind_verified_jwt(&issuer, self.subject)?;

        let trusted_issuer = self
            .container_bindings
            .trusted_issuer(&issuer)
            .ok_or_else(|| RequestBindingError::UnplannedTrustedIssuer {
                subject: self.subject.to_string(),
                parameter: self.position.to_string(),
                issuer: issuer.to_string(),
            })?;

        Ok(RequestBinding::BearerToken {
            claims,
            profile,
            trusted_issuer,
        })
    }
}
