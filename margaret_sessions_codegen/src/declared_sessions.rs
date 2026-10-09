use syn::Path;

use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::tag::Tag;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_parsing::AudienceParsing;
use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

use crate::consumed_sessions_declaration::ConsumedSessionsDeclaration;
use crate::declared_session_cookies::DeclaredSessionCookies;
use crate::issued_sessions_declaration::IssuedSessionsDeclaration;
use crate::session_cookies_variant::SessionCookiesVariant;
use crate::session_cookies_vocabulary::SESSION_COOKIES;
use crate::sessions_codegen_error::SessionsCodegenError;
use crate::sessions_declaration::SessionsDeclaration;

fn sessions_issuer(
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<Tag, SessionsCodegenError> {
    let issuer =
        reader
            .take_path("issuer")?
            .ok_or_else(|| SessionsCodegenError::MissingSessionsIssuer {
                anchor: anchor.to_string(),
            })?;

    Tag::from_path(&issuer).ok_or_else(|| SessionsCodegenError::MalformedSessionsIssuer {
        anchor: anchor.to_string(),
    })
}

fn session_audience(
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
    issuance: &DeclaredTokenIssuance,
    resources: &DeclaredResourceIssuances,
) -> Result<Audience, SessionsCodegenError> {
    let written = reader.take_string("audience")?.ok_or_else(|| {
        SessionsCodegenError::MissingSessionAudience {
            anchor: anchor.to_string(),
        }
    })?;
    let AudienceParsing::Accepted(audience) = Audience::parse(&written) else {
        return Err(SessionsCodegenError::EmptySessionAudience {
            anchor: anchor.to_string(),
        });
    };

    if let DeclaredTokenIssuance::Declared(declaration) = issuance
        && declaration.issuer.as_str() == audience.as_str()
    {
        return Err(SessionsCodegenError::SessionAudienceNamesIssuer {
            anchor: anchor.to_string(),
            audience: audience.to_string(),
        });
    }

    match resources
        .resources()
        .find(|resource| resource.audience == audience)
    {
        Some(resource) => Err(SessionsCodegenError::SessionAudienceOfResource {
            anchor: anchor.to_string(),
            audience: audience.to_string(),
            resource: resource.tag.to_string(),
        }),
        None => Ok(audience),
    }
}

fn session_cookies(
    index: &AttributeIndex,
    item: &IndexedItem,
    written: &Path,
    nested: &mut AttributeArgumentsReader,
) -> Result<DeclaredSessionCookies, SessionsCodegenError> {
    let anchor = item.canonical_path().to_string();
    let variant = index
        .resolve_item_path(item, written)
        .as_ref()
        .and_then(|resolved| SESSION_COOKIES.variant(resolved))
        .ok_or_else(|| SessionsCodegenError::UnknownSessionCookies {
            anchor: anchor.clone(),
            written: format_path(written),
        })?;

    match variant {
        SessionCookiesVariant::HostOnly => Ok(DeclaredSessionCookies::HostOnly),
        SessionCookiesVariant::SharedWithDomain => {
            let name = nested.take_string("domain_from")?.ok_or_else(|| {
                SessionsCodegenError::MissingCookieDomainSource {
                    anchor: anchor.clone(),
                }
            })?;

            EnvironmentVariableName::new(&name)
                .map(|domain_from| DeclaredSessionCookies::SharedWithDomain { domain_from })
                .ok_or(SessionsCodegenError::MalformedCookieDomainSource { anchor, name })
        }
    }
}

fn issued<'index>(
    index: &'index AttributeIndex,
    matched: &MatchedAttribute<'index>,
    issuance: &DeclaredTokenIssuance,
    resources: &DeclaredResourceIssuances,
) -> Result<IssuedSessionsDeclaration<'index>, SessionsCodegenError> {
    let anchor = declaration_anchor(index, matched, FrameworkAttribute::IssuesSessions)?.item;
    let path = anchor.canonical_path().to_string();

    matched.args()?.interpret(|reader| {
        let issuer = sessions_issuer(reader, &path)?;
        let audience = session_audience(reader, &path, issuance, resources)?;
        let cookies = reader
            .take_variant("cookies", |written, nested| {
                session_cookies(index, anchor, written, nested)
            })?
            .ok_or_else(|| SessionsCodegenError::MissingSessionCookies {
                anchor: path.clone(),
            })?;

        Ok(IssuedSessionsDeclaration {
            anchor,
            audience,
            cookies,
            issuer,
        })
    })
}

fn consumed<'index>(
    index: &'index AttributeIndex,
    matched: &MatchedAttribute<'index>,
) -> Result<ConsumedSessionsDeclaration<'index>, SessionsCodegenError> {
    let anchor = declaration_anchor(index, matched, FrameworkAttribute::ConsumesSessions)?.item;
    let path = anchor.canonical_path().to_string();

    matched.args()?.interpret(|reader| {
        let issuer = sessions_issuer(reader, &path)?;
        let domain_name = reader.take_string("cookie_domain_from")?.ok_or_else(|| {
            SessionsCodegenError::MissingConsumedCookieDomainSource {
                anchor: path.clone(),
            }
        })?;
        let cookie_domain_from = EnvironmentVariableName::new(&domain_name).ok_or_else(|| {
            SessionsCodegenError::MalformedConsumedCookieDomainSource {
                anchor: path.clone(),
                name: domain_name,
            }
        })?;
        let refresh_name = reader.take_string("refresh_url_from")?.ok_or_else(|| {
            SessionsCodegenError::MissingRefreshUrlSource {
                anchor: path.clone(),
            }
        })?;
        let refresh_url_from = EnvironmentVariableName::new(&refresh_name).ok_or_else(|| {
            SessionsCodegenError::MalformedRefreshUrlSource {
                anchor: path.clone(),
                name: refresh_name,
            }
        })?;

        if cookie_domain_from == refresh_url_from {
            return Err(SessionsCodegenError::SharedEnvironmentVariable {
                anchor: path.clone(),
                name: refresh_url_from.as_str().to_string(),
            });
        }

        Ok(ConsumedSessionsDeclaration {
            anchor,
            cookie_domain_from,
            issuer,
            refresh_url_from,
        })
    })
}

pub enum DeclaredSessions<'index> {
    Absent,
    Consumed(ConsumedSessionsDeclaration<'index>),
    Issued(IssuedSessionsDeclaration<'index>),
}

impl<'index> DeclaredSessions<'index> {
    /// # Errors
    ///
    /// Returns `SessionsCodegenError` when a session declaration is malformed, its audience names
    /// the issuer or is the audience of a resource, or the crate declares sessions more than once.
    pub fn read(
        index: &'index AttributeIndex,
        issuance: &DeclaredTokenIssuance,
        resources: &DeclaredResourceIssuances,
    ) -> Result<Self, SessionsCodegenError> {
        let mut declarations = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::IssuesSessions) {
            declarations.push(SessionsDeclaration::Issued(issued(
                index, &matched, issuance, resources,
            )?));
        }

        for matched in index.select_framework_attribute(FrameworkAttribute::ConsumesSessions) {
            declarations.push(SessionsDeclaration::Consumed(consumed(index, &matched)?));
        }

        let mut declarations = declarations.into_iter();
        let Some(first) = declarations.next() else {
            return Ok(Self::Absent);
        };

        if let Some(second) = declarations.next() {
            return Err(SessionsCodegenError::AmbiguousSessions {
                first: first.anchor(),
                second: second.anchor(),
            });
        }

        Ok(match first {
            SessionsDeclaration::Consumed(consumed) => Self::Consumed(consumed),
            SessionsDeclaration::Issued(issued) => Self::Issued(issued),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
    use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use super::DeclaredSessions;
    use crate::consumed_sessions_declaration::ConsumedSessionsDeclaration;
    use crate::declared_session_cookies::DeclaredSessionCookies;
    use crate::issued_sessions_declaration::IssuedSessionsDeclaration;
    use crate::sessions_codegen_error::SessionsCodegenError;

    const ISSUANCE: &str = "use margaret::framework::sessions::session_cookies::SessionCookies;\n\n#[issues_tokens(provider, issuer = \"https://issuer.example\")]\npub struct Issuer;\n#[issues_resource_tokens(artifacts, audience = \"artifacts\")]\npub struct Artifacts;\n";

    fn read<TOutcome>(
        declarations: &str,
        inspect: impl FnOnce(Result<DeclaredSessions<'_>, SessionsCodegenError>) -> TOutcome,
    ) -> TOutcome {
        let indexed = IndexedSource::new(&format!("{ISSUANCE}{declarations}"));
        let issuance =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");
        let resources = DeclaredResourceIssuances::read(&indexed.index, &issuance)
            .expect("the resources are read");

        inspect(DeclaredSessions::read(
            &indexed.index,
            &issuance,
            &resources,
        ))
    }

    fn rejection(declarations: &str) -> SessionsCodegenError {
        read(declarations, |read| {
            read.err().expect("the declaration is rejected")
        })
    }

    fn issuing(arguments: &str) -> String {
        format!("#[issues_sessions({arguments})]\npub struct BrowserSessions;\n")
    }

    fn consuming(arguments: &str) -> String {
        format!("#[consumes_sessions({arguments})]\npub struct PartnerSessions;\n")
    }

    #[test]
    fn reads_host_only_sessions_issued_by_the_issuance() {
        read(
            &issuing(
                "issuer = provider, audience = \"browser\", cookies = SessionCookies::HostOnly",
            ),
            |read| {
                assert!(matches!(
                    read,
                    Ok(DeclaredSessions::Issued(IssuedSessionsDeclaration {
                        anchor,
                        audience,
                        cookies: DeclaredSessionCookies::HostOnly,
                        issuer,
                    })) if anchor.canonical_path().to_string() == "crate::BrowserSessions"
                        && audience.as_str() == "browser"
                        && issuer.to_string() == "provider"
                ));
            },
        );
    }

    #[test]
    fn rejects_a_session_audience_naming_the_issuer() {
        assert!(matches!(
            rejection(&issuing(
                "issuer = provider, audience = \"https://issuer.example\", cookies = SessionCookies::HostOnly",
            )),
            SessionsCodegenError::SessionAudienceNamesIssuer { anchor, audience }
                if anchor == "crate::BrowserSessions" && audience == "https://issuer.example"
        ));
    }

    #[test]
    fn reads_sessions_shared_with_the_domain_of_an_environment_variable() {
        read(
            &issuing(
                "issuer = provider, audience = \"browser\", cookies = SessionCookies::SharedWithDomain(domain_from = \"SESSION_COOKIE_DOMAIN\")",
            ),
            |read| {
                assert!(matches!(
                    read,
                    Ok(DeclaredSessions::Issued(IssuedSessionsDeclaration {
                        cookies: DeclaredSessionCookies::SharedWithDomain { domain_from },
                        ..
                    })) if domain_from.as_str() == "SESSION_COOKIE_DOMAIN"
                ));
            },
        );
    }

    #[test]
    fn reads_sessions_consumed_from_a_trusted_issuer() {
        read(
            &consuming(
                "issuer = partner, cookie_domain_from = \"SESSION_COOKIE_DOMAIN\", refresh_url_from = \"SESSION_REFRESH_URL\"",
            ),
            |read| {
                assert!(matches!(
                    read,
                    Ok(DeclaredSessions::Consumed(ConsumedSessionsDeclaration {
                        cookie_domain_from,
                        issuer,
                        refresh_url_from,
                        ..
                    })) if cookie_domain_from.as_str() == "SESSION_COOKIE_DOMAIN"
                        && issuer.to_string() == "partner"
                        && refresh_url_from.as_str() == "SESSION_REFRESH_URL"
                ));
            },
        );
    }

    #[test]
    fn finds_no_sessions_without_a_declaration() {
        read("", |read| {
            assert!(read.is_ok_and(
                |sessions| discriminant(&sessions) == discriminant(&DeclaredSessions::Absent)
            ));
        });
    }

    #[test]
    fn rejects_a_crate_that_both_issues_and_consumes_sessions() {
        assert!(matches!(
            rejection(&format!(
                "{}{}",
                issuing("issuer = provider, audience = \"browser\", cookies = SessionCookies::HostOnly"),
                consuming("issuer = partner, cookie_domain_from = \"SESSION_COOKIE_DOMAIN\", refresh_url_from = \"SESSION_REFRESH_URL\"")
            )),
            SessionsCodegenError::AmbiguousSessions { first, second }
                if first == "crate::BrowserSessions" && second == "crate::PartnerSessions"
        ));
    }

    #[test]
    fn rejects_sessions_without_their_issuer() {
        assert!(matches!(
            rejection(&issuing("audience = \"browser\", cookies = SessionCookies::HostOnly")),
            SessionsCodegenError::MissingSessionsIssuer { anchor } if anchor == "crate::BrowserSessions"
        ));
    }

    #[test]
    fn rejects_an_issuer_that_is_not_a_plain_tag() {
        assert!(matches!(
            rejection(&consuming("issuer = trusted::partner")),
            SessionsCodegenError::MalformedSessionsIssuer { anchor } if anchor == "crate::PartnerSessions"
        ));
    }

    #[test]
    fn rejects_an_issuer_that_is_not_a_path() {
        assert!(matches!(
            rejection(&consuming("issuer = \"partner\"")),
            SessionsCodegenError::AttributeArguments(AttributeArgumentsError::UnexpectedArgument { key, .. })
                if key == "issuer"
        ));
    }

    #[test]
    fn rejects_issued_sessions_without_an_audience() {
        assert!(matches!(
            rejection(&issuing("issuer = provider, cookies = SessionCookies::HostOnly")),
            SessionsCodegenError::MissingSessionAudience { anchor } if anchor == "crate::BrowserSessions"
        ));
    }

    #[test]
    fn rejects_an_audience_that_is_not_a_string() {
        assert!(matches!(
            rejection(&issuing("issuer = provider, audience = browser, cookies = SessionCookies::HostOnly")),
            SessionsCodegenError::AttributeArguments(AttributeArgumentsError::UnexpectedArgument { key, .. })
                if key == "audience"
        ));
    }

    #[test]
    fn rejects_an_empty_audience() {
        assert!(matches!(
            rejection(&issuing("issuer = provider, audience = \"\", cookies = SessionCookies::HostOnly")),
            SessionsCodegenError::EmptySessionAudience { anchor } if anchor == "crate::BrowserSessions"
        ));
    }

    #[test]
    fn rejects_the_audience_of_a_resource() {
        assert!(matches!(
            rejection(&issuing("issuer = provider, audience = \"artifacts\", cookies = SessionCookies::HostOnly")),
            SessionsCodegenError::SessionAudienceOfResource { audience, resource, .. }
                if audience == "artifacts" && resource == "artifacts"
        ));
    }

    #[test]
    fn rejects_issued_sessions_without_a_cookie_policy() {
        assert!(matches!(
            rejection(&issuing("issuer = provider, audience = \"browser\"")),
            SessionsCodegenError::MissingSessionCookies { anchor } if anchor == "crate::BrowserSessions"
        ));
    }

    #[test]
    fn rejects_a_cookie_policy_of_a_foreign_enum() {
        assert!(matches!(
            rejection(&format!(
                "pub enum Policy {{ HostOnly }}\n{}",
                issuing("issuer = provider, audience = \"browser\", cookies = Policy::HostOnly")
            )),
            SessionsCodegenError::UnknownSessionCookies { written, .. } if written == "Policy::HostOnly"
        ));
    }

    #[test]
    fn rejects_shared_cookies_without_their_domain() {
        assert!(matches!(
            rejection(&issuing("issuer = provider, audience = \"browser\", cookies = SessionCookies::SharedWithDomain")),
            SessionsCodegenError::MissingCookieDomainSource { anchor } if anchor == "crate::BrowserSessions"
        ));
    }

    #[test]
    fn rejects_a_cookie_domain_read_from_a_malformed_variable_name() {
        assert!(matches!(
            rejection(&issuing("issuer = provider, audience = \"browser\", cookies = SessionCookies::SharedWithDomain(domain_from = \"1DOMAIN\")")),
            SessionsCodegenError::MalformedCookieDomainSource { name, .. } if name == "1DOMAIN"
        ));
    }

    #[test]
    fn rejects_a_cookie_domain_source_that_is_not_a_string() {
        assert!(matches!(
            rejection(&issuing("issuer = provider, audience = \"browser\", cookies = SessionCookies::SharedWithDomain(domain_from = DOMAIN)")),
            SessionsCodegenError::AttributeArguments(AttributeArgumentsError::UnexpectedArgument { key, .. })
                if key == "domain_from"
        ));
    }

    #[test]
    fn rejects_consumed_sessions_without_their_cookie_domain() {
        assert!(matches!(
            rejection(&consuming("issuer = partner, refresh_url_from = \"SESSION_REFRESH_URL\"")),
            SessionsCodegenError::MissingConsumedCookieDomainSource { anchor } if anchor == "crate::PartnerSessions"
        ));
    }

    #[test]
    fn rejects_a_consumed_cookie_domain_read_from_a_malformed_variable_name() {
        assert!(matches!(
            rejection(&consuming("issuer = partner, cookie_domain_from = \"1DOMAIN\", refresh_url_from = \"SESSION_REFRESH_URL\"")),
            SessionsCodegenError::MalformedConsumedCookieDomainSource { name, .. } if name == "1DOMAIN"
        ));
    }

    #[test]
    fn rejects_consumed_sessions_without_their_refresh_url() {
        assert!(matches!(
            rejection(&consuming("issuer = partner, cookie_domain_from = \"SESSION_COOKIE_DOMAIN\"")),
            SessionsCodegenError::MissingRefreshUrlSource { anchor } if anchor == "crate::PartnerSessions"
        ));
    }

    #[test]
    fn rejects_a_refresh_url_read_from_a_malformed_variable_name() {
        assert!(matches!(
            rejection(&consuming("issuer = partner, cookie_domain_from = \"SESSION_COOKIE_DOMAIN\", refresh_url_from = \"1URL\"")),
            SessionsCodegenError::MalformedRefreshUrlSource { name, .. } if name == "1URL"
        ));
    }

    #[test]
    fn rejects_a_cookie_domain_and_refresh_url_read_from_one_variable() {
        assert!(matches!(
            rejection(&consuming("issuer = partner, cookie_domain_from = \"SESSION\", refresh_url_from = \"SESSION\"")),
            SessionsCodegenError::SharedEnvironmentVariable { name, .. } if name == "SESSION"
        ));
    }

    #[test]
    fn rejects_a_consumed_source_that_is_not_a_string() {
        assert!(matches!(
            rejection(&consuming("issuer = partner, cookie_domain_from = DOMAIN, refresh_url_from = \"SESSION_REFRESH_URL\"")),
            SessionsCodegenError::AttributeArguments(AttributeArgumentsError::UnexpectedArgument { key, .. })
                if key == "cookie_domain_from"
        ));
    }

    #[test]
    fn rejects_a_refresh_url_source_that_is_not_a_string() {
        assert!(matches!(
            rejection(&consuming("issuer = partner, cookie_domain_from = \"SESSION_COOKIE_DOMAIN\", refresh_url_from = URL")),
            SessionsCodegenError::AttributeArguments(AttributeArgumentsError::UnexpectedArgument { key, .. })
                if key == "refresh_url_from"
        ));
    }

    #[test]
    fn rejects_issued_sessions_declared_on_a_singleton() {
        assert!(matches!(
            rejection(&format!(
                "#[singleton]\n{}",
                issuing("issuer = provider, audience = \"browser\", cookies = SessionCookies::HostOnly")
            )),
            SessionsCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. })
                if path == "crate::BrowserSessions"
        ));
    }

    #[test]
    fn rejects_consumed_sessions_declared_on_a_singleton() {
        assert!(matches!(
            rejection(&format!(
                "#[singleton]\n{}",
                consuming("issuer = partner, cookie_domain_from = \"SESSION_COOKIE_DOMAIN\", refresh_url_from = \"SESSION_REFRESH_URL\"")
            )),
            SessionsCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. })
                if path == "crate::PartnerSessions"
        ));
    }

    #[test]
    fn rejects_issued_session_arguments_that_do_not_parse() {
        assert!(matches!(
            rejection(&issuing("= 5")),
            SessionsCodegenError::Index(AttributeError::Arguments(AttributeArgumentsError::Malformed { attribute_path, .. }))
                if attribute_path == "issues_sessions"
        ));
    }

    #[test]
    fn rejects_consumed_session_arguments_that_do_not_parse() {
        assert!(matches!(
            rejection(&consuming("= 5")),
            SessionsCodegenError::Index(AttributeError::Arguments(AttributeArgumentsError::Malformed { attribute_path, .. }))
                if attribute_path == "consumes_sessions"
        ));
    }
}
