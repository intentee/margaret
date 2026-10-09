use quote::ToTokens;
use syn::Path;

use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::tag::Tag;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_https_url::https_url::HttpsUrl;
use margaret_https_url::https_url_parsing::HttpsUrlParsing;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_parsing::AudienceParsing;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::declared_issuer_keys::DeclaredIssuerKeys;
use crate::declared_trust::DeclaredTrust;
use crate::issuer_key_sources::ISSUER_KEY_SOURCES;
use crate::trust_source::TrustSource;
use crate::trusted_issuer_codegen_error::TrustedIssuerCodegenError;

fn trust_source(
    index: &AttributeIndex,
    anchor: &IndexedItem,
    variant: &Path,
    reader: &mut AttributeArgumentsReader,
) -> Result<TrustSource, TrustedIssuerCodegenError> {
    let path = anchor.canonical_path().to_string();

    match index
        .resolve_item_path(anchor, variant)
        .as_ref()
        .and_then(|resolved| ISSUER_KEY_SOURCES.variant(resolved))
    {
        None => Err(TrustedIssuerCodegenError::UnknownIssuerKeys {
            anchor: path,
            written: variant.to_token_stream().to_string(),
        }),
        Some(DeclaredIssuerKeys::Discovered) => Ok(TrustSource::Discovery),
        Some(DeclaredIssuerKeys::Published) => Ok(TrustSource::JwksEndpoint {
            jwks_uri: match HttpsUrl::parse(&reader.take_string("jwks_uri")?.ok_or_else(|| {
                TrustedIssuerCodegenError::MissingJwksUri {
                    anchor: path.clone(),
                }
            })?) {
                HttpsUrlParsing::Accepted(jwks_uri) => jwks_uri,
                HttpsUrlParsing::Rejected(rejection) => {
                    return Err(TrustedIssuerCodegenError::MalformedJwksUri {
                        anchor: path,
                        rejection,
                    });
                }
            },
        }),
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
    ) -> Result<Self, TrustedIssuerCodegenError> {
        let anchor =
            declaration_anchor(index, matched, FrameworkAttribute::VerifiesTokensFromIssuer)?.item;
        let path = anchor.canonical_path().to_string();

        matched.args()?.interpret(|reader| {
            let tag = reader
                .take_positional_path()
                .ok_or_else(|| TrustedIssuerCodegenError::MissingTag {
                    anchor: path.clone(),
                })
                .and_then(|tag| {
                    Tag::from_path(&tag).ok_or_else(|| TrustedIssuerCodegenError::MalformedTag {
                        anchor: path.clone(),
                    })
                })?;
            let AudienceParsing::Accepted(audience) =
                Audience::parse(&reader.take_string("audience")?.ok_or_else(|| {
                    TrustedIssuerCodegenError::MissingAudience {
                        anchor: path.clone(),
                    }
                })?)
            else {
                return Err(TrustedIssuerCodegenError::EmptyAudience {
                    anchor: path.clone(),
                });
            };
            let issuer = reader
                .take_string("issuer")?
                .ok_or_else(|| TrustedIssuerCodegenError::MissingIssuer {
                    anchor: path.clone(),
                })?
                .parse::<IssuerIdentifier>()
                .map_err(|source| TrustedIssuerCodegenError::MalformedIssuer {
                    anchor: path.clone(),
                    source,
                })?;
            let source = reader
                .take_variant(ItemNamingArgument::Keys.key(), |variant, keys| {
                    trust_source(index, anchor, variant, keys)
                })?
                .ok_or_else(|| TrustedIssuerCodegenError::MissingIssuerKeys {
                    anchor: path.clone(),
                })?;

            Ok(Self {
                issuer,
                source,
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
                "#[verifies_tokens_from_issuer(audience = \"a\", issuer = \"https://issuer.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\npub struct Partner;\n"
            )
            .to_string(),
            "#[verifies_tokens_from_issuer] on 'crate::Partner' does not name a tag"
        );
    }

    #[test]
    fn rejects_a_tag_that_is_not_a_plain_name() {
        assert_eq!(
            rejection(
                "#[verifies_tokens_from_issuer(partner::tag, audience = \"a\", issuer = \"https://issuer.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\npub struct Partner;\n"
            )
            .to_string(),
            "#[verifies_tokens_from_issuer] on 'crate::Partner' names a tag that is not a single plain name"
        );
    }

    #[test]
    fn rejects_a_trust_without_an_audience() {
        assert_eq!(
            rejection(
                "#[verifies_tokens_from_issuer(partner, issuer = \"https://issuer.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\npub struct Partner;\n"
            )
            .to_string(),
            "#[verifies_tokens_from_issuer] on 'crate::Partner' does not declare the audience of the verified tokens"
        );
    }

    #[test]
    fn rejects_an_empty_audience() {
        assert!(matches!(
            rejection(
                "#[verifies_tokens_from_issuer(partner, audience = \"\", issuer = \"https://issuer.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\npub struct Partner;\n"
            ),
            TrustedIssuerCodegenError::EmptyAudience { anchor }
                if anchor == "crate::Partner"
        ));
    }

    #[test]
    fn rejects_a_trust_without_an_issuer() {
        assert_eq!(
            rejection("#[verifies_tokens_from_issuer(partner, audience = \"a\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\npub struct Partner;\n")
                .to_string(),
            "#[verifies_tokens_from_issuer] on 'crate::Partner' does not declare the issuer of the verified tokens"
        );
    }

    #[test]
    fn rejects_an_issuer_over_plain_http() {
        assert!(
            rejection(
                "#[verifies_tokens_from_issuer(ci, audience = \"a\", issuer = \"http://ci.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = \"https://ci.example/jwks\"))]\npub struct Ci;\n"
            )
            .to_string()
            .starts_with("the issuer #[verifies_tokens_from_issuer] declares on 'crate::Ci' is malformed: ")
        );
    }

    #[test]
    fn rejects_a_trust_without_its_keys() {
        assert!(matches!(
            rejection(
                "#[verifies_tokens_from_issuer(partner, audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Partner;\n"
            ),
            TrustedIssuerCodegenError::MissingIssuerKeys { anchor } if anchor == "crate::Partner"
        ));
    }

    #[test]
    fn rejects_keys_that_are_not_a_variant() {
        assert!(matches!(
            rejection(
                "#[verifies_tokens_from_issuer(partner, audience = \"a\", issuer = \"https://issuer.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Rotated)]\npub struct Partner;\n"
            ),
            TrustedIssuerCodegenError::UnknownIssuerKeys { anchor, written }
                if anchor == "crate::Partner"
                    && written == "margaret :: framework :: trusted_issuer :: issuer_keys :: IssuerKeys :: Rotated"
        ));
    }

    #[test]
    fn rejects_a_jwks_endpoint_trust_without_a_jwks_uri() {
        assert_eq!(
            rejection(
                "#[verifies_tokens_from_issuer(ci, audience = \"a\", issuer = \"https://ci.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published)]\npub struct Ci;\n"
            )
            .to_string(),
            "#[verifies_tokens_from_issuer] on 'crate::Ci' publishes its keys without declaring their jwks_uri"
        );
    }

    #[test]
    fn rejects_a_plaintext_jwks_uri() {
        assert_eq!(
            rejection(
                "#[verifies_tokens_from_issuer(ci, audience = \"a\", issuer = \"https://ci.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = \"http://ci.example/jwks\"))]\npub struct Ci;\n"
            )
            .to_string(),
            "the jwks_uri #[verifies_tokens_from_issuer] declares on 'crate::Ci' is rejected: the url uses the 'http' scheme instead of https"
        );
    }

    #[test]
    fn rejects_a_jwks_uri_on_a_discovered_trust() {
        assert!(matches!(
            rejection(
                "#[verifies_tokens_from_issuer(partner, audience = \"a\", issuer = \"https://issuer.example\", jwks_uri = \"https://issuer.example/jwks\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\npub struct Partner;\n"
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
                "#[verifies_tokens_from_issuer(partner, audience = api, issuer = \"https://issuer.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\npub struct Partner;\n"
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
                "#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = 5, keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\npub struct Partner;\n"
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
                "#[verifies_tokens_from_issuer(ci, audience = \"a\", issuer = \"https://ci.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = jwks))]\npub struct Ci;\n"
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
                "#[singleton]\n#[verifies_tokens_from_issuer(partner, audience = \"a\", issuer = \"https://issuer.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\npub struct Partner;\n"
            ),
            TrustedIssuerCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. })
                if path == "crate::Partner"
        ));
    }

    #[test]
    fn reports_unparseable_trust_arguments() {
        assert!(matches!(
            rejection("#[verifies_tokens_from_issuer(= 5, keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published)]\npub struct Ci;\n"),
            TrustedIssuerCodegenError::Index(AttributeError::Arguments(
                AttributeArgumentsError::Malformed { attribute_path, .. }
            )) if attribute_path == "verifies_tokens_from_issuer"
        ));
    }
}
