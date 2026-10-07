use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::tag::Tag;

use crate::trust_attribute::TrustAttribute;
use crate::trust_declaration::TrustDeclaration;
use crate::trusted_issuer_binding::TrustedIssuerBinding;
use crate::trusted_issuer_codegen_error::TrustedIssuerCodegenError;
use crate::trusted_issuer_group::TrustedIssuerGroup;

fn shared_discovery(
    groups: &[TrustedIssuerGroup],
    founded: &TrustedIssuerGroup,
) -> Result<(), TrustedIssuerCodegenError> {
    let TrustedIssuerGroup::Discovered {
        discovery_url,
        issuer,
        ..
    } = founded
    else {
        return Ok(());
    };

    match groups.iter().find(|group| {
        matches!(group, TrustedIssuerGroup::Discovered { discovery_url: existing, .. } if existing == discovery_url)
    }) {
        Some(existing) => Err(TrustedIssuerCodegenError::DistinctIssuersShareDiscovery {
            discovery_url: discovery_url.to_string(),
            first: existing.issuer().as_str().to_string(),
            second: issuer.as_str().to_string(),
        }),
        None => Ok(()),
    }
}

pub struct DeclaredTrusts<'index> {
    pub groups: Vec<TrustedIssuerGroup<'index>>,
}

impl<'index> DeclaredTrusts<'index> {
    /// # Errors
    ///
    /// Returns `TrustedIssuerCodegenError` when a declaration is malformed, when two declarations
    /// trust one issuer for one audience, when an issuer trusted through a jwks endpoint is
    /// trusted by another declaration as well, or when distinct issuers share a discovery
    /// location.
    pub fn read(index: &'index AttributeIndex) -> Result<Self, TrustedIssuerCodegenError> {
        let mut declarations: Vec<TrustDeclaration<'index>> = Vec::new();

        for attribute in TrustAttribute::ALL {
            for matched in index.select_framework_attribute(attribute.framework_attribute()) {
                declarations.push(TrustDeclaration::read(index, &matched, attribute)?);
            }
        }

        declarations.sort_by_key(|declaration| declaration.trust.tag.to_string());

        let mut groups: Vec<TrustedIssuerGroup<'index>> = Vec::new();

        for declaration in declarations {
            if let Some(group) = groups
                .iter_mut()
                .find(|group| group.issuer().as_str() == declaration.issuer.as_str())
            {
                group.admit(declaration)?;
            } else {
                let founded = TrustedIssuerGroup::founded_by(declaration);

                shared_discovery(&groups, &founded)?;
                groups.push(founded);
            }
        }

        Ok(Self { groups })
    }

    #[must_use]
    pub fn binding(&self, tag: &Tag) -> Option<TrustedIssuerBinding<'_, 'index>> {
        self.groups.iter().find_map(|group| {
            group
                .members()
                .find(|trust| trust.tag == *tag)
                .map(|trust| TrustedIssuerBinding { group, trust })
        })
    }

    pub fn bindings(&self) -> impl Iterator<Item = TrustedIssuerBinding<'_, 'index>> {
        self.groups.iter().flat_map(|group| {
            group
                .members()
                .map(move |trust| TrustedIssuerBinding { group, trust })
        })
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use super::DeclaredTrusts;
    use crate::trusted_issuer_group::TrustedIssuerGroup;

    fn rejection(source: &str) -> String {
        let indexed = IndexedSource::new(source);

        DeclaredTrusts::read(&indexed.index)
            .err()
            .expect("the trusts are rejected")
            .to_string()
    }

    #[test]
    fn reads_a_trust_of_a_discovered_issuer() {
        let indexed = IndexedSource::new(
            "#[trusts_oidc_issuer(partner, audience = \"api\", issuer = \"https://partner.example/tenant\")]\npub struct Partner;\n",
        );
        let trusts = DeclaredTrusts::read(&indexed.index).expect("the trust is read");

        assert!(matches!(
            trusts.groups.as_slice(),
            [TrustedIssuerGroup::Discovered {
                discovery_url,
                issuer,
                lead,
                others,
            }] if discovery_url.as_str()
                == "https://partner.example/tenant/.well-known/openid-configuration"
                && issuer.as_str() == "https://partner.example/tenant"
                && lead.audience.as_str() == "api"
                && lead.tag.to_string() == "partner"
                && lead.anchor.canonical_path().to_string() == "crate::Partner"
                && others.is_empty()
        ));
    }

    #[test]
    fn reads_a_trust_of_a_jwks_endpoint_issuer() {
        let indexed = IndexedSource::new(
            "#[provides_jwks_endpoint(ci, audience = \"deploy\", issuer = \"https://ci.example\", jwks_uri = \"https://ci.example/jwks\")]\npub struct Ci;\n",
        );
        let trusts = DeclaredTrusts::read(&indexed.index).expect("the trust is read");

        assert!(matches!(
            trusts.groups.as_slice(),
            [TrustedIssuerGroup::JwksEndpoint {
                issuer,
                jwks_uri,
                trust,
            }] if issuer.as_str() == "https://ci.example"
                && jwks_uri.as_str() == "https://ci.example/jwks"
                && trust.tag.to_string() == "ci"
        ));
    }

    #[test]
    fn groups_the_trusts_of_one_issuer_led_by_the_first_tag() {
        let indexed = IndexedSource::new(
            "#[trusts_oidc_issuer(beta, audience = \"b\", issuer = \"https://issuer.example\")]\npub struct Beta;\n#[trusts_oidc_issuer(alpha, audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Alpha;\n",
        );
        let trusts = DeclaredTrusts::read(&indexed.index).expect("the trusts are read");

        assert_eq!(trusts.groups.len(), 1);
        assert_eq!(
            trusts
                .groups
                .iter()
                .flat_map(TrustedIssuerGroup::members)
                .map(|trust| trust.tag.to_string())
                .collect::<Vec<String>>(),
            ["alpha", "beta"]
        );
    }

    #[test]
    fn rejects_two_trusts_of_one_issuer_for_one_audience() {
        assert_eq!(
            rejection(
                "#[trusts_oidc_issuer(first, audience = \"api\", issuer = \"https://issuer.example\")]\npub struct First;\n#[trusts_oidc_issuer(second, audience = \"api\", issuer = \"https://issuer.example\")]\npub struct Second;\n"
            ),
            "the issuer 'https://issuer.example' is trusted for the audience 'api' by both 'crate::First' and 'crate::Second', so its tokens have no single addressee"
        );
    }

    #[test]
    fn rejects_a_discovered_trust_of_a_jwks_endpoint_issuer() {
        assert_eq!(
            rejection(
                "#[provides_jwks_endpoint(alpha, audience = \"a\", issuer = \"https://issuer.example\", jwks_uri = \"https://issuer.example/jwks\")]\npub struct Alpha;\n#[trusts_oidc_issuer(beta, audience = \"b\", issuer = \"https://issuer.example\")]\npub struct Beta;\n"
            ),
            "the issuer 'https://issuer.example' is trusted through a jwks endpoint by 'crate::Alpha' and by 'crate::Beta' as well, so its key set has no single source"
        );
    }

    #[test]
    fn rejects_a_jwks_endpoint_trust_of_a_discovered_issuer() {
        assert_eq!(
            rejection(
                "#[trusts_oidc_issuer(alpha, audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Alpha;\n#[provides_jwks_endpoint(beta, audience = \"b\", issuer = \"https://issuer.example\", jwks_uri = \"https://issuer.example/jwks\")]\npub struct Beta;\n"
            ),
            "the issuer 'https://issuer.example' is trusted through a jwks endpoint by 'crate::Alpha' and by 'crate::Beta' as well, so its key set has no single source"
        );
    }

    #[test]
    fn rejects_distinct_issuers_that_share_a_discovery_location() {
        assert_eq!(
            rejection(
                "#[trusts_oidc_issuer(alpha, audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Alpha;\n#[trusts_oidc_issuer(beta, audience = \"b\", issuer = \"https://issuer.example/\")]\npub struct Beta;\n"
            ),
            "the issuers 'https://issuer.example' and 'https://issuer.example/' publish their metadata at the same discovery location 'https://issuer.example/.well-known/openid-configuration', so at most one of them can match it"
        );
    }

    #[test]
    fn binds_a_tag_to_its_trust_and_issuer() {
        let indexed = IndexedSource::new(
            "#[trusts_oidc_issuer(alpha, audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Alpha;\n#[trusts_oidc_issuer(beta, audience = \"b\", issuer = \"https://issuer.example\")]\npub struct Beta;\n",
        );
        let trusts = DeclaredTrusts::read(&indexed.index).expect("the trusts are read");
        let beta = syn::parse_str("beta").expect("the tag parses");
        let binding = trusts
            .binding(&margaret_attributes::tag::Tag::from_path(&beta).expect("the tag is plain"))
            .expect("the tag is trusted");

        assert_eq!(binding.trust.audience.as_str(), "b");
        assert_eq!(binding.group.lead().tag.to_string(), "alpha");
    }

    #[test]
    fn binds_no_trust_to_an_undeclared_tag() {
        let indexed = IndexedSource::new(
            "#[trusts_oidc_issuer(alpha, audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Alpha;\n",
        );
        let trusts = DeclaredTrusts::read(&indexed.index).expect("the trust is read");
        let missing = syn::parse_str("missing").expect("the tag parses");

        assert!(
            trusts
                .binding(
                    &margaret_attributes::tag::Tag::from_path(&missing).expect("the tag is plain")
                )
                .is_none()
        );
    }

    #[test]
    fn lists_the_binding_of_every_trust_by_issuer() {
        let indexed = IndexedSource::new(
            "#[trusts_oidc_issuer(gamma, audience = \"g\", issuer = \"https://first.example\")]\npub struct Gamma;\n#[trusts_oidc_issuer(beta, audience = \"b\", issuer = \"https://second.example\")]\npub struct Beta;\n#[trusts_oidc_issuer(alpha, audience = \"a\", issuer = \"https://first.example\")]\npub struct Alpha;\n",
        );
        let trusts = DeclaredTrusts::read(&indexed.index).expect("the trusts are read");

        assert_eq!(
            trusts
                .bindings()
                .map(|binding| format!("{}={}", binding.trust.tag, binding.group.issuer()))
                .collect::<Vec<String>>(),
            [
                "alpha=https://first.example",
                "gamma=https://first.example",
                "beta=https://second.example",
            ]
        );
    }
}
