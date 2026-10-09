use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::oidc_provider_codegen_error::OidcProviderCodegenError;

#[derive(Debug, Eq, PartialEq)]
pub enum DeclaredUserinfoClaims {
    Absent,
    Declared(CanonicalPath),
}

impl DeclaredUserinfoClaims {
    /// # Errors
    ///
    /// Returns `OidcProviderCodegenError` when `#[provides_userinfo_claims]` carries arguments or
    /// is declared by more than one item.
    pub fn read(index: &AttributeIndex) -> Result<Self, OidcProviderCodegenError> {
        let mut providers = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::ProvidesUserinfoClaims)
        {
            matched
                .args()?
                .interpret(|_| Ok::<(), OidcProviderCodegenError>(()))?;
            providers.push(matched.item().canonical_path().clone());
        }

        let mut providers = providers.into_iter();
        let Some(declared) = providers.next() else {
            return Ok(Self::Absent);
        };

        match providers.next() {
            Some(another) => Err(OidcProviderCodegenError::AmbiguousUserinfoClaimsProvider {
                first: declared.to_string(),
                second: another.to_string(),
            }),
            None => Ok(Self::Declared(declared)),
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use super::DeclaredUserinfoClaims;
    use crate::oidc_provider_codegen_error::OidcProviderCodegenError;

    fn read(source: &str) -> Result<DeclaredUserinfoClaims, OidcProviderCodegenError> {
        DeclaredUserinfoClaims::read(&IndexedSource::new(source).index)
    }

    #[test]
    fn reads_the_declared_claims_provider() {
        assert!(matches!(
            read("#[singleton]\n#[provides_userinfo_claims]\npub struct ProfileClaims;\n"),
            Ok(DeclaredUserinfoClaims::Declared(path)) if path.to_string() == "crate::ProfileClaims"
        ));
    }

    #[test]
    fn finds_no_claims_provider_when_none_is_declared() {
        assert_eq!(
            read("pub struct Unrelated;\n").expect("the declarations are read"),
            DeclaredUserinfoClaims::Absent
        );
    }

    #[test]
    fn rejects_two_claims_providers() {
        assert!(matches!(
            read("#[singleton]\n#[provides_userinfo_claims]\npub struct ProfileClaims;\n#[singleton]\n#[provides_userinfo_claims]\npub struct EmailClaims;\n"),
            Err(OidcProviderCodegenError::AmbiguousUserinfoClaimsProvider { first, second })
                if first == "crate::EmailClaims" && second == "crate::ProfileClaims"
        ));
    }

    #[test]
    fn rejects_an_argument_of_the_claims_provider() {
        assert!(matches!(
            read("#[singleton]\n#[provides_userinfo_claims(scope = Profile)]\npub struct ProfileClaims;\n"),
            Err(OidcProviderCodegenError::AttributeArguments(
                AttributeArgumentsError::UnrecognizedArgument { argument, .. }
            )) if argument == "scope"
        ));
    }

    #[test]
    fn rejects_arguments_that_do_not_parse() {
        assert!(matches!(
            read("#[singleton]\n#[provides_userinfo_claims(= 5)]\npub struct ProfileClaims;\n"),
            Err(OidcProviderCodegenError::Index(AttributeError::Arguments(
                AttributeArgumentsError::Malformed { attribute_path, .. }
            ))) if attribute_path == "provides_userinfo_claims"
        ));
    }
}
