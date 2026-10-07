use margaret_accepted_clients_codegen::accepted_client_constant::AcceptedClientConstant;
use margaret_accepted_clients_codegen::accepted_client_constant_path::accepted_client_constant_path;
use margaret_accepted_clients_codegen::accepted_client_declaration::AcceptedClientDeclaration;
use margaret_accepted_clients_codegen::accepted_client_item::AcceptedClientItem;
use margaret_accepted_clients_codegen::accepted_client_item_path::accepted_client_item_path;
use margaret_accepted_clients_codegen::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;

fn constructed(
    client: &AcceptedClientDeclaration,
    item: AcceptedClientItem,
    method: &str,
    dependencies: Vec<FrameworkDependency>,
) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies,
            is_async: false,
            method: method.to_string(),
            outcome: ConstructorOutcome::Infallible,
        },
        enablement: FrameworkEnablement::Dependency,
        injection: FrameworkInjectionRole::Unmarked,
        provided: accepted_client_item_path(client.anchor.identifier, item),
    }
}

fn constant(
    client: &AcceptedClientDeclaration,
    constant: AcceptedClientConstant,
) -> FrameworkDependency {
    FrameworkDependency::Constant(accepted_client_constant_path(
        client.anchor.identifier,
        constant,
    ))
}

fn key_set(client: &AcceptedClientDeclaration) -> FrameworkDependency {
    FrameworkDependency::Provider(accepted_client_item_path(
        client.anchor.identifier,
        AcceptedClientItem::IssuerKeySet,
    ))
}

pub(crate) fn accepted_client_framework_providers(
    client: &AcceptedClientDeclaration,
) -> Vec<FrameworkProvider> {
    match client.authentication {
        DeclaredAcceptedAuthentication::PrivateKeyJwt(_) => vec![
            constructed(
                client,
                AcceptedClientItem::IssuerKeySet,
                "awaiting",
                Vec::new(),
            ),
            constructed(
                client,
                AcceptedClientItem::PolledKeySet,
                "published",
                vec![
                    constant(client, AcceptedClientConstant::JwksEndpointIssuer),
                    key_set(client),
                ],
            ),
            constructed(
                client,
                AcceptedClientItem::RegisteredClient,
                "private_key_jwt",
                vec![
                    constant(client, AcceptedClientConstant::AcceptedClient),
                    constant(client, AcceptedClientConstant::ConfidentialPrivileges),
                    key_set(client),
                ],
            ),
        ],
        DeclaredAcceptedAuthentication::Public => vec![constructed(
            client,
            AcceptedClientItem::RegisteredClient,
            "public",
            vec![constant(client, AcceptedClientConstant::AcceptedClient)],
        )],
    }
}
