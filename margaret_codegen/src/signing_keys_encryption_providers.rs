use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_jwks_codegen::declared_signing_keys_encryption::DeclaredSigningKeysEncryption;
use margaret_jwks_codegen::signing_keys_encryption_declaration::SigningKeysEncryptionDeclaration;

use crate::signing_keys_encryption_key_canonical_path::signing_keys_encryption_key_canonical_path;

pub(crate) fn signing_keys_encryption_providers(
    encryption: &DeclaredSigningKeysEncryption,
) -> Vec<FrameworkProvider> {
    match encryption {
        DeclaredSigningKeysEncryption::Absent => Vec::new(),
        DeclaredSigningKeysEncryption::Declared(SigningKeysEncryptionDeclaration {
            key_from,
            ..
        }) => vec![FrameworkProvider {
            construction: FrameworkConstruction::EnvironmentVariable {
                name: key_from.clone(),
            },
            enablement: FrameworkEnablement::Declared,
            injection: FrameworkInjectionRole::FrameworkState,
            provided: signing_keys_encryption_key_canonical_path(),
        }],
    }
}
