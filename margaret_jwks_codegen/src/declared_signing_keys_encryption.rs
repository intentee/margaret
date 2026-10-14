use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;

use crate::jwks_codegen_error::JwksCodegenError;
use crate::signing_keys_encryption_declaration::SigningKeysEncryptionDeclaration;

pub enum DeclaredSigningKeysEncryption<'index> {
    Absent,
    Declared(SigningKeysEncryptionDeclaration<'index>),
}

impl<'index> DeclaredSigningKeysEncryption<'index> {
    /// # Errors
    ///
    /// Returns `JwksCodegenError` when a declaration is malformed or when more than one struct
    /// declares the signing keys encryption.
    pub fn read(index: &'index AttributeIndex) -> Result<Self, JwksCodegenError> {
        let mut declarations = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::EncryptsSigningKeys) {
            let anchor =
                declaration_anchor(index, &matched, FrameworkAttribute::EncryptsSigningKeys)?;

            declarations.push(SigningKeysEncryptionDeclaration::read(
                &matched,
                anchor.item,
            )?);
        }

        let mut declarations = declarations.into_iter();
        let Some(declared) = declarations.next() else {
            return Ok(Self::Absent);
        };

        match declarations.next() {
            Some(another) => Err(JwksCodegenError::AmbiguousSigningKeysEncryption {
                first: declared.anchor.canonical_path().to_string(),
                second: another.anchor.canonical_path().to_string(),
            }),
            None => Ok(Self::Declared(declared)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

    use super::DeclaredSigningKeysEncryption;
    use crate::jwks_codegen_error::JwksCodegenError;
    use crate::signing_keys_encryption_declaration::SigningKeysEncryptionDeclaration;

    fn read<TOutcome>(
        source: &str,
        outcome: impl FnOnce(Result<DeclaredSigningKeysEncryption, JwksCodegenError>) -> TOutcome,
    ) -> TOutcome {
        outcome(DeclaredSigningKeysEncryption::read(
            &IndexedSource::new(source).index,
        ))
    }

    #[test]
    fn reads_the_environment_variable_of_the_encryption_key() {
        read(
            "use margaret::framework::macros::encrypts_signing_keys;\n#[encrypts_signing_keys(key_from = \"BLOG_SIGNING_KEYS_ENCRYPTION_KEY\")]\npub struct SigningKeysEncryption;\n",
            |read| {
                assert!(matches!(
                    read,
                    Ok(DeclaredSigningKeysEncryption::Declared(SigningKeysEncryptionDeclaration {
                        anchor,
                        key_from,
                    })) if anchor.canonical_path().to_string() == "crate::SigningKeysEncryption"
                        && key_from.as_str() == "BLOG_SIGNING_KEYS_ENCRYPTION_KEY"
                ));
            },
        );
    }

    #[test]
    fn finds_no_encryption_when_none_is_declared() {
        read("pub struct Unrelated;\n", |read| {
            assert_eq!(
                discriminant(&read.expect("the absence is read")),
                discriminant(&DeclaredSigningKeysEncryption::Absent)
            );
        });
    }

    #[test]
    fn rejects_a_declaration_without_a_key_source() {
        read(
            "use margaret::framework::macros::encrypts_signing_keys;\n#[encrypts_signing_keys()]\npub struct SigningKeysEncryption;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(JwksCodegenError::MissingKeySource { anchor })
                        if anchor == "crate::SigningKeysEncryption"
                ));
            },
        );
    }

    #[test]
    fn rejects_a_key_source_that_is_not_a_string_literal() {
        read(
            "use margaret::framework::macros::encrypts_signing_keys;\n#[encrypts_signing_keys(key_from = 5)]\npub struct SigningKeysEncryption;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(JwksCodegenError::AttributeArguments(
                        AttributeArgumentsError::UnexpectedArgument { key, expected, .. }
                    )) if key == "key_from" && expected == "string literal"
                ));
            },
        );
    }

    #[test]
    fn rejects_a_key_source_that_is_not_an_environment_variable_name() {
        read(
            "use margaret::framework::macros::encrypts_signing_keys;\n#[encrypts_signing_keys(key_from = \"signing key\")]\npub struct SigningKeysEncryption;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(JwksCodegenError::MalformedKeySource { name, .. }) if name == "signing key"
                ));
            },
        );
    }

    #[test]
    fn rejects_an_unrecognized_argument_instead_of_silently_ignoring_it() {
        read(
            "use margaret::framework::macros::encrypts_signing_keys;\n#[encrypts_signing_keys(key_from = \"BLOG_SIGNING_KEYS_ENCRYPTION_KEY\", rotate = true)]\npub struct SigningKeysEncryption;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(JwksCodegenError::AttributeArguments(
                        AttributeArgumentsError::UnrecognizedArgument { argument, .. }
                    )) if argument == "rotate"
                ));
            },
        );
    }

    #[test]
    fn rejects_unparseable_arguments_naming_the_attribute_so_the_author_can_find_them() {
        read(
            "use margaret::framework::macros::encrypts_signing_keys;\n#[encrypts_signing_keys(= 5)]\npub struct SigningKeysEncryption;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(JwksCodegenError::Index(AttributeError::Arguments(
                        AttributeArgumentsError::Malformed { attribute_path, .. }
                    ))) if attribute_path == "encrypts_signing_keys"
                ));
            },
        );
    }

    #[test]
    fn rejects_an_encryption_declared_on_a_singleton_since_it_needs_a_plain_struct() {
        read(
            "use margaret::framework::macros::encrypts_signing_keys;\nuse margaret::framework::macros::singleton;\n#[singleton]\n#[encrypts_signing_keys(key_from = \"BLOG_SIGNING_KEYS_ENCRYPTION_KEY\")]\npub struct SigningKeysEncryption;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(JwksCodegenError::Anchor(
                        DeclarationAnchorError::DeclaredAsSingleton { path, .. }
                    )) if path == "crate::SigningKeysEncryption"
                ));
            },
        );
    }

    #[test]
    fn rejects_a_second_encryption_because_the_keys_are_sealed_under_one_key() {
        read(
            "use margaret::framework::macros::encrypts_signing_keys;\n#[encrypts_signing_keys(key_from = \"BLOG_SIGNING_KEYS_ENCRYPTION_KEY\")]\npub struct SigningKeysEncryption;\n\n#[encrypts_signing_keys(key_from = \"AUDIT_SIGNING_KEYS_ENCRYPTION_KEY\")]\npub struct AuditSigningKeysEncryption;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(JwksCodegenError::AmbiguousSigningKeysEncryption { first, second })
                        if first == "crate::AuditSigningKeysEncryption"
                            && second == "crate::SigningKeysEncryption"
                ));
            },
        );
    }
}
