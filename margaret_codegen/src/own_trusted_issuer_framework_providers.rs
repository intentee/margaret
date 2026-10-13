use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_trusted_issuer_codegen::own_trust::OwnTrust;
use margaret_trusted_issuer_codegen::trusted_issuer_constant::TrustedIssuerConstant;
use margaret_trusted_issuer_codegen::trusted_issuer_constant_path::trusted_issuer_constant_path;
use margaret_trusted_issuer_codegen::trusted_issuer_item::TrustedIssuerItem;
use margaret_trusted_issuer_codegen::trusted_issuer_item_path::trusted_issuer_item_path;

use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;

pub(crate) fn own_trusted_issuer_framework_providers(
    own_trusts: &[OwnTrust],
) -> Vec<FrameworkProvider> {
    own_trusts
        .iter()
        .map(|own| FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![
                    FrameworkDependency::Provider(server_secret_store_canonical_path()),
                    FrameworkDependency::Constant(trusted_issuer_constant_path(
                        own.tag,
                        TrustedIssuerConstant::TokenTrust,
                    )),
                ],
                is_async: false,
                method: "own".to_string(),
                outcome: ConstructorOutcome::Infallible,
            },
            enablement: FrameworkEnablement::Declared,
            injection: FrameworkInjectionRole::TrustedIssuer(own.tag.clone()),
            provided: trusted_issuer_item_path(own.tag, TrustedIssuerItem::TrustedIssuer),
        })
        .collect()
}
