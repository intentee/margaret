use margaret_accepted_clients_codegen::accepted_client_constant::AcceptedClientConstant;
use margaret_accepted_clients_codegen::accepted_client_constant_path::accepted_client_constant_path;
use margaret_accepted_clients_codegen::accepted_client_declaration::AcceptedClientDeclaration;
use margaret_accepted_clients_codegen::accepted_client_item::AcceptedClientItem;
use margaret_accepted_clients_codegen::accepted_client_item_path::accepted_client_item_path;
use margaret_accepted_clients_codegen::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use margaret_accepted_clients_codegen::declared_client_keys::DeclaredClientKeys;
use margaret_accepted_clients_codegen::declared_code_grant::DeclaredCodeGrant;
use margaret_accepted_clients_codegen::declared_code_policy::DeclaredCodePolicy;
use margaret_accepted_clients_codegen::declared_confidential_client::DeclaredConfidentialClient;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::url_source::UrlSource;
use margaret_http_codegen::declared_routes::DeclaredRoutes;
use margaret_http_codegen::http_codegen_error::HttpCodegenError;

use crate::authorization_grants_store_canonical_path::authorization_grants_store_canonical_path;
use crate::client_assertions_store_canonical_path::client_assertions_store_canonical_path;
use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;

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

fn item(client: &AcceptedClientDeclaration, item: AcceptedClientItem) -> FrameworkDependency {
    FrameworkDependency::Provider(accepted_client_item_path(client.anchor.identifier, item))
}

fn code_grant_dependencies(
    client: &AcceptedClientDeclaration,
    policy: &DeclaredCodePolicy,
    routes: &DeclaredRoutes,
) -> Result<[FrameworkDependency; 3], HttpCodegenError> {
    let route_urls = policy
        .redirect_routes
        .iter()
        .map(|route| {
            routes
                .redirect_target(route, client.anchor.item.canonical_path())
                .map(UrlSource::Route)
        })
        .collect::<Result<Vec<UrlSource>, HttpCodegenError>>()?;

    Ok([
        constant(client, AcceptedClientConstant::CodeGrantPolicy),
        FrameworkDependency::Urls(
            policy
                .redirect_uris
                .iter()
                .map(|redirect_uri| UrlSource::Declared(redirect_uri.to_string()))
                .chain(route_urls)
                .collect(),
        ),
        FrameworkDependency::Provider(authorization_grants_store_canonical_path()),
    ])
}

fn confidential_dependencies(client: &AcceptedClientDeclaration) -> [FrameworkDependency; 4] {
    [
        constant(client, AcceptedClientConstant::AcceptedClient),
        constant(client, AcceptedClientConstant::ConfidentialPrivileges),
        item(client, AcceptedClientItem::ClientKeySet),
        FrameworkDependency::Provider(client_assertions_store_canonical_path()),
    ]
}

fn registered_client(
    client: &AcceptedClientDeclaration,
    routes: &DeclaredRoutes,
) -> Result<FrameworkProvider, HttpCodegenError> {
    let registered = |method: &str, dependencies: Vec<FrameworkDependency>| {
        constructed(
            client,
            AcceptedClientItem::RegisteredClient,
            method,
            dependencies,
        )
    };

    Ok(match client.authentication {
        DeclaredAcceptedAuthentication::PrivateKeyJwt(_) => match &client.authorization_code {
            DeclaredCodeGrant::Granted(policy) => registered(
                "private_key_jwt_with_code_grant",
                confidential_dependencies(client)
                    .into_iter()
                    .chain(code_grant_dependencies(client, policy, routes)?)
                    .collect(),
            ),
            DeclaredCodeGrant::Withheld => {
                registered("private_key_jwt", confidential_dependencies(client).into())
            }
        },
        DeclaredAcceptedAuthentication::Public => match &client.authorization_code {
            DeclaredCodeGrant::Granted(policy) => registered(
                "public_with_code_grant",
                [constant(client, AcceptedClientConstant::AcceptedClient)]
                    .into_iter()
                    .chain(code_grant_dependencies(client, policy, routes)?)
                    .collect(),
            ),
            DeclaredCodeGrant::Withheld => registered(
                "public",
                vec![constant(client, AcceptedClientConstant::AcceptedClient)],
            ),
        },
    })
}

fn client_key_set_providers(
    client: &AcceptedClientDeclaration,
    keys: &DeclaredClientKeys,
) -> Vec<FrameworkProvider> {
    match keys {
        DeclaredClientKeys::Own => vec![constructed(
            client,
            AcceptedClientItem::ClientKeySet,
            "own",
            vec![FrameworkDependency::Provider(
                server_secret_store_canonical_path(),
            )],
        )],
        DeclaredClientKeys::Published { .. } => vec![
            constructed(
                client,
                AcceptedClientItem::ClientKeySet,
                "published",
                vec![
                    item(client, AcceptedClientItem::IssuerKeySet),
                    constant(client, AcceptedClientConstant::AssertionSigning),
                ],
            ),
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
                    item(client, AcceptedClientItem::IssuerKeySet),
                ],
            ),
        ],
    }
}

pub(crate) fn accepted_client_framework_providers(
    client: &AcceptedClientDeclaration,
    routes: &DeclaredRoutes,
) -> Result<Vec<FrameworkProvider>, HttpCodegenError> {
    let registered = registered_client(client, routes)?;

    Ok(match &client.authentication {
        DeclaredAcceptedAuthentication::PrivateKeyJwt(DeclaredConfidentialClient {
            keys, ..
        }) => client_key_set_providers(client, keys)
            .into_iter()
            .chain([registered])
            .collect(),
        DeclaredAcceptedAuthentication::Public => vec![registered],
    })
}
