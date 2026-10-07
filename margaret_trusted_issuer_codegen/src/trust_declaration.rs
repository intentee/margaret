use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::tag::Tag;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_https_url::https_url::HttpsUrl;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::declared_trust::DeclaredTrust;
use crate::trust_attribute::TrustAttribute;
use crate::trust_source::TrustSource;
use crate::trusted_issuer_codegen_error::TrustedIssuerCodegenError;

fn trust_source(
    reader: &mut AttributeArgumentsReader,
    attribute: TrustAttribute,
    anchor: &str,
) -> Result<TrustSource, TrustedIssuerCodegenError> {
    match attribute {
        TrustAttribute::ProvidesJwksEndpoint => Ok(TrustSource::JwksEndpoint {
            jwks_uri: reader
                .take_string("jwks_uri")?
                .ok_or_else(|| TrustedIssuerCodegenError::MissingJwksUri {
                    anchor: anchor.to_string(),
                })?
                .parse::<HttpsUrl>()
                .map_err(|source| TrustedIssuerCodegenError::MalformedJwksUri {
                    anchor: anchor.to_string(),
                    source,
                })?,
        }),
        TrustAttribute::TrustsOidcIssuer => Ok(TrustSource::Discovery),
    }
}

pub(crate) struct TrustDeclaration<'index> {
    pub(crate) issuer: IssuerIdentifier,
    pub(crate) source: TrustSource,
    pub(crate) trust: DeclaredTrust<'index>,
}

impl<'index> TrustDeclaration<'index> {
    pub(crate) fn read(
        index: &'index AttributeIndex,
        matched: &MatchedAttribute<'index>,
        attribute: TrustAttribute,
    ) -> Result<Self, TrustedIssuerCodegenError> {
        let framework_attribute = attribute.framework_attribute();
        let anchor = declaration_anchor(index, matched, framework_attribute)?.item;
        let path = anchor.canonical_path().to_string();
        let attribute_name = framework_attribute.name();

        matched.args()?.interpret(|reader| {
            let tag = reader
                .take_positional_path()
                .ok_or_else(|| TrustedIssuerCodegenError::MissingTag {
                    anchor: path.clone(),
                    attribute: attribute_name,
                })
                .and_then(|tag| {
                    Tag::from_path(&tag).ok_or_else(|| TrustedIssuerCodegenError::MalformedTag {
                        anchor: path.clone(),
                        attribute: attribute_name,
                    })
                })?;
            let audience = reader
                .take_string("audience")?
                .ok_or_else(|| TrustedIssuerCodegenError::MissingAudience {
                    anchor: path.clone(),
                    attribute: attribute_name,
                })?
                .parse::<Audience>()
                .map_err(|source| TrustedIssuerCodegenError::MalformedAudience {
                    anchor: path.clone(),
                    attribute: attribute_name,
                    source,
                })?;
            let issuer = reader
                .take_string("issuer")?
                .ok_or_else(|| TrustedIssuerCodegenError::MissingIssuer {
                    anchor: path.clone(),
                    attribute: attribute_name,
                })?
                .parse::<IssuerIdentifier>()
                .map_err(|source| TrustedIssuerCodegenError::MalformedIssuer {
                    anchor: path.clone(),
                    attribute: attribute_name,
                    source,
                })?;

            Ok(Self {
                issuer,
                source: trust_source(reader, attribute, &path)?,
                trust: DeclaredTrust {
                    anchor,
                    audience,
                    tag,
                },
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

    use crate::declared_trusts::DeclaredTrusts;
    use crate::trusted_issuer_codegen_error::TrustedIssuerCodegenError;

    fn rejection(source: &str) -> TrustedIssuerCodegenError {
        let indexed = IndexedSource::new(source);

        DeclaredTrusts::read(&indexed.index)
            .err()
            .expect("the declaration is rejected")
    }

    #[test]
    fn rejects_a_trust_without_a_tag() {
        assert_eq!(
            rejection(
                "#[trusts_oidc_issuer(audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Partner;\n"
            )
            .to_string(),
            "#[trusts_oidc_issuer] on 'crate::Partner' does not name a tag"
        );
    }

    #[test]
    fn rejects_a_tag_that_is_not_a_plain_name() {
        assert_eq!(
            rejection(
                "#[trusts_oidc_issuer(partner::tag, audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Partner;\n"
            )
            .to_string(),
            "#[trusts_oidc_issuer] on 'crate::Partner' names a tag that is not a single plain name"
        );
    }

    #[test]
    fn rejects_a_trust_without_an_audience() {
        assert_eq!(
            rejection(
                "#[trusts_oidc_issuer(partner, issuer = \"https://issuer.example\")]\npub struct Partner;\n"
            )
            .to_string(),
            "#[trusts_oidc_issuer] on 'crate::Partner' does not declare the audience of the trusted tokens"
        );
    }

    #[test]
    fn rejects_an_empty_audience() {
        assert!(matches!(
            rejection(
                "#[trusts_oidc_issuer(partner, audience = \"\", issuer = \"https://issuer.example\")]\npub struct Partner;\n"
            ),
            TrustedIssuerCodegenError::MalformedAudience { anchor, attribute: "trusts_oidc_issuer", .. }
                if anchor == "crate::Partner"
        ));
    }

    #[test]
    fn rejects_a_trust_without_an_issuer() {
        assert_eq!(
            rejection("#[trusts_oidc_issuer(partner, audience = \"a\")]\npub struct Partner;\n")
                .to_string(),
            "#[trusts_oidc_issuer] on 'crate::Partner' does not declare the issuer of the trusted tokens"
        );
    }

    #[test]
    fn rejects_an_issuer_over_plain_http() {
        assert!(
            rejection(
                "#[provides_jwks_endpoint(ci, audience = \"a\", issuer = \"http://ci.example\", jwks_uri = \"https://ci.example/jwks\")]\npub struct Ci;\n"
            )
            .to_string()
            .starts_with("the issuer #[provides_jwks_endpoint] declares on 'crate::Ci' is malformed: ")
        );
    }

    #[test]
    fn rejects_a_jwks_endpoint_trust_without_a_jwks_uri() {
        assert_eq!(
            rejection(
                "#[provides_jwks_endpoint(ci, audience = \"a\", issuer = \"https://ci.example\")]\npub struct Ci;\n"
            )
            .to_string(),
            "#[provides_jwks_endpoint] on 'crate::Ci' does not declare the jwks_uri of the issuer"
        );
    }

    #[test]
    fn rejects_a_plaintext_jwks_uri() {
        assert_eq!(
            rejection(
                "#[provides_jwks_endpoint(ci, audience = \"a\", issuer = \"https://ci.example\", jwks_uri = \"http://ci.example/jwks\")]\npub struct Ci;\n"
            )
            .to_string(),
            "the jwks_uri #[provides_jwks_endpoint] declares on 'crate::Ci' is rejected: the url uses the 'http' scheme instead of https"
        );
    }

    #[test]
    fn rejects_a_jwks_uri_on_a_discovered_trust() {
        assert!(matches!(
            rejection(
                "#[trusts_oidc_issuer(partner, audience = \"a\", issuer = \"https://issuer.example\", jwks_uri = \"https://issuer.example/jwks\")]\npub struct Partner;\n"
            ),
            TrustedIssuerCodegenError::AttributeArguments(
                AttributeArgumentsError::UnrecognizedArgument { argument, .. }
            ) if argument == "jwks_uri"
        ));
    }

    #[test]
    fn rejects_an_audience_that_is_not_a_string() {
        assert!(matches!(
            rejection(
                "#[trusts_oidc_issuer(partner, audience = api, issuer = \"https://issuer.example\")]\npub struct Partner;\n"
            ),
            TrustedIssuerCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "audience"
        ));
    }

    #[test]
    fn rejects_an_issuer_that_is_not_a_string() {
        assert!(matches!(
            rejection(
                "#[trusts_oidc_issuer(partner, audience = \"api\", issuer = 5)]\npub struct Partner;\n"
            ),
            TrustedIssuerCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "issuer"
        ));
    }

    #[test]
    fn rejects_a_jwks_uri_that_is_not_a_string() {
        assert!(matches!(
            rejection(
                "#[provides_jwks_endpoint(ci, audience = \"a\", issuer = \"https://ci.example\", jwks_uri = jwks)]\npub struct Ci;\n"
            ),
            TrustedIssuerCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "jwks_uri"
        ));
    }

    #[test]
    fn rejects_a_trust_anchored_by_a_singleton() {
        assert!(matches!(
            rejection(
                "#[singleton]\n#[trusts_oidc_issuer(partner, audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Partner;\n"
            ),
            TrustedIssuerCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. })
                if path == "crate::Partner"
        ));
    }

    #[test]
    fn reports_unparseable_trust_arguments() {
        assert!(matches!(
            rejection("#[provides_jwks_endpoint(= 5)]\npub struct Ci;\n"),
            TrustedIssuerCodegenError::Index(AttributeError::Arguments(
                AttributeArgumentsError::Malformed { attribute_path, .. }
            )) if attribute_path == "provides_jwks_endpoint"
        ));
    }
}
