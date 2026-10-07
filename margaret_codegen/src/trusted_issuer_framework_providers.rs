use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;
use margaret_trusted_issuer_codegen::trusted_issuer_constant::TrustedIssuerConstant;
use margaret_trusted_issuer_codegen::trusted_issuer_constant_path::trusted_issuer_constant_path;
use margaret_trusted_issuer_codegen::trusted_issuer_group::TrustedIssuerGroup;
use margaret_trusted_issuer_codegen::trusted_issuer_item::TrustedIssuerItem;
use margaret_trusted_issuer_codegen::trusted_issuer_item_path::trusted_issuer_item_path;

fn constructed(
    dependencies: Vec<FrameworkDependency>,
    method: &str,
    outcome: ConstructorOutcome,
) -> FrameworkConstruction {
    FrameworkConstruction::Constructor {
        dependencies,
        is_async: false,
        method: method.to_string(),
        outcome,
    }
}

fn awaiting(provided: TrustedIssuerItem, group: &TrustedIssuerGroup) -> FrameworkProvider {
    FrameworkProvider {
        construction: constructed(Vec::new(), "awaiting", ConstructorOutcome::Infallible),
        enablement: FrameworkEnablement::Dependency,
        injection: FrameworkInjectionRole::Unmarked,
        provided: trusted_issuer_item_path(&group.lead().tag, provided),
    }
}

fn group_providers(group: &TrustedIssuerGroup) -> Vec<FrameworkProvider> {
    let lead = &group.lead().tag;
    let key_set = || {
        FrameworkDependency::Provider(trusted_issuer_item_path(
            lead,
            TrustedIssuerItem::IssuerKeySet,
        ))
    };
    let (mut providers, polled_key_set) = match group {
        TrustedIssuerGroup::Discovered { .. } => (
            vec![awaiting(TrustedIssuerItem::IssuerMetadata, group)],
            constructed(
                vec![
                    FrameworkDependency::Constant(trusted_issuer_constant_path(
                        lead,
                        TrustedIssuerConstant::DiscoveredIssuer,
                    )),
                    FrameworkDependency::Provider(trusted_issuer_item_path(
                        lead,
                        TrustedIssuerItem::IssuerMetadata,
                    )),
                    key_set(),
                ],
                "discovered",
                ConstructorOutcome::Infallible,
            ),
        ),
        TrustedIssuerGroup::JwksEndpoint { .. } => (
            Vec::new(),
            constructed(
                vec![
                    FrameworkDependency::Constant(trusted_issuer_constant_path(
                        lead,
                        TrustedIssuerConstant::JwksEndpointIssuer,
                    )),
                    key_set(),
                ],
                "published",
                ConstructorOutcome::Infallible,
            ),
        ),
    };

    providers.push(awaiting(TrustedIssuerItem::IssuerKeySet, group));
    providers.push(FrameworkProvider {
        construction: polled_key_set,
        enablement: FrameworkEnablement::Dependency,
        injection: FrameworkInjectionRole::Unmarked,
        provided: trusted_issuer_item_path(lead, TrustedIssuerItem::PolledKeySet),
    });
    providers.extend(group.members().map(|trust| FrameworkProvider {
        construction: constructed(
            vec![
                key_set(),
                FrameworkDependency::Constant(trusted_issuer_constant_path(
                    &trust.tag,
                    TrustedIssuerConstant::TokenTrust,
                )),
            ],
            "create",
            ConstructorOutcome::Infallible,
        ),
        enablement: FrameworkEnablement::Always,
        injection: FrameworkInjectionRole::TrustedIssuer(trust.tag.clone()),
        provided: trusted_issuer_item_path(&trust.tag, TrustedIssuerItem::TrustedIssuer),
    }));

    providers
}

pub(crate) fn trusted_issuer_framework_providers(
    trusts: &DeclaredTrusts,
) -> Vec<FrameworkProvider> {
    trusts.groups.iter().flat_map(group_providers).collect()
}
