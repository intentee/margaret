use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use margaret_accepted_clients_codegen::accepted_client_item::AcceptedClientItem;
use margaret_accepted_clients_codegen::accepted_client_item_path::accepted_client_item_path;
use margaret_accepted_clients_codegen::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
use margaret_accepted_clients_codegen::declared_client_keys::DeclaredClientKeys;
use margaret_accepted_clients_codegen::declared_confidential_client::DeclaredConfidentialClient;
use margaret_accepted_clients_codegen::provider_aggregate::ProviderAggregate;
use margaret_accepted_clients_codegen::render_accepted_clients::render_accepted_clients;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_console_codegen::console_plan::ConsolePlan;
use margaret_console_codegen::render_console::render_console;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::plan_container::plan_container;
use margaret_container::planned_container::PlannedContainer;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::declared_routes::DeclaredRoutes;
use margaret_http_codegen::http_artifacts::HttpArtifacts;
use margaret_http_codegen::http_plan::HttpPlan;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::render_http::render_http;
use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
use margaret_middleware_codegen::render_middleware_wrappers::render_middleware_wrappers;
use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
use margaret_oidc_provider_codegen::oidc_provider_item::OidcProviderItem;
use margaret_oidc_provider_codegen::oidc_provider_item_path::oidc_provider_item_path;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::render_authenticated_user_wrappers::render_authenticated_user_wrappers;
use margaret_request_binding_codegen::views_availability::ViewsAvailability;
use margaret_schema_codegen::render_schema::render_schema;
use margaret_serve_input_codegen::scan::scan;
use margaret_service_codegen::framework_service::FrameworkService;
use margaret_service_codegen::render_services::render_services;
use margaret_service_codegen::service_plan::ServicePlan;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;
use margaret_tag_codegen::subject_token_exchanger_binding::SubjectTokenExchangerBinding;
use margaret_tag_codegen::tag_pool::TagPool;
use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
use margaret_token_issuance_codegen::render_resource_tokens::render_resource_tokens;
use margaret_token_issuance_codegen::render_token_issuance::render_token_issuance;
use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;
use margaret_trusted_issuer_codegen::own_trust::OwnTrust;
use margaret_trusted_issuer_codegen::trusted_issuer_item::TrustedIssuerItem;
use margaret_trusted_issuer_codegen::trusted_issuer_item_path::trusted_issuer_item_path;
use margaret_umbrella_path::umbrella_module_name::UMBRELLA_MODULE_NAME;
use margaret_views_codegen::render_views::render_views;
use margaret_views_codegen::views_artifacts::ViewsArtifacts;
use margaret_views_codegen::views_plan::ViewsPlan;
use margaret_websocket_codegen::render_websocket::render_websocket;
use margaret_websocket_codegen::web_socket_artifacts::WebSocketArtifacts;
use margaret_websocket_codegen::web_socket_plan::WebSocketPlan;

use crate::accepted_client_framework_providers::accepted_client_framework_providers;
use crate::asset_bag_modules::asset_bag_modules;
use crate::asset_responder_canonical_path::asset_responder_canonical_path;
use crate::build_jwks_artifacts::build_jwks_artifacts;
use crate::build_oauth_client_artifacts::build_oauth_client_artifacts;
use crate::build_oidc_provider_artifacts::build_oidc_provider_artifacts;
use crate::build_trusted_issuer_artifacts::build_trusted_issuer_artifacts;
use crate::codegen_error::CodegenError;
use crate::enabled_framework_schemas::enabled_framework_schemas;
use crate::format_pass::format_pass;
use crate::framework_declarations::FrameworkDeclarations;
use crate::framework_modules::FrameworkModules;
use crate::framework_state_providers::framework_state_providers;
use crate::generated_code::GeneratedCode;
use crate::generated_feature::GeneratedFeature;
use crate::generated_features::GeneratedFeatures;
use crate::http_modules::HttpModules;
use crate::issuer_directory_framework_providers::issuer_directory_framework_providers;
use crate::issuer_directory_services::issuer_directory_services;
use crate::jwks_framework_providers::jwks_framework_providers;
use crate::oauth_client_framework_providers::oauth_client_framework_providers;
use crate::oidc_provider_framework_providers::oidc_provider_framework_providers;
use crate::own_trusted_issuer_framework_providers::own_trusted_issuer_framework_providers;
use crate::own_trusts::own_trusts;
use crate::provider_declarations::ProviderDeclarations;
use crate::provider_endpoint_modules::provider_endpoint_modules;
use crate::role_modules::RoleModules;
use crate::role_roots::RoleRoots;
use crate::schema_models::SchemaModels;
use crate::served_modules::ServedModules;
use crate::serving_modules::ServingModules;
use crate::trusted_issuer_framework_providers::trusted_issuer_framework_providers;
use crate::umbrella::umbrella;
use crate::wrapper_modules::WrapperModules;

/// # Errors
///
/// Returns `CodegenError` propagated from the work it performs.
fn framework_providers(
    trusts: &DeclaredTrusts,
    own_trusts: &[OwnTrust],
    oauth_client_bindings: &[OAuthClientBinding],
    accepted_clients: &DeclaredAcceptedClients,
    exchanger_bindings: &[SubjectTokenExchangerBinding],
    routes: &DeclaredRoutes,
    database: &DeclaredPostgresDatabase,
) -> Result<Vec<FrameworkProvider>, CodegenError> {
    let mut framework_providers = vec![FrameworkProvider {
        construction: FrameworkConstruction::Unit,
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: asset_responder_canonical_path(),
    }];
    framework_providers.extend(framework_state_providers(database));
    framework_providers.extend(jwks_framework_providers());
    framework_providers.extend(trusted_issuer_framework_providers(trusts));
    framework_providers.extend(own_trusted_issuer_framework_providers(own_trusts));
    for client in &accepted_clients.clients {
        framework_providers.extend(accepted_client_framework_providers(client, routes)?);
    }
    framework_providers.extend(issuer_directory_framework_providers(
        trusts
            .groups
            .iter()
            .map(|group| {
                trusted_issuer_item_path(&group.lead().tag, TrustedIssuerItem::PolledKeySet)
            })
            .chain(
                accepted_clients
                    .clients
                    .iter()
                    .filter(|client| {
                        matches!(
                            client.authentication,
                            DeclaredAcceptedAuthentication::PrivateKeyJwt(
                                DeclaredConfidentialClient {
                                    keys: DeclaredClientKeys::Published { .. },
                                    ..
                                }
                            )
                        )
                    })
                    .map(|client| {
                        accepted_client_item_path(
                            client.anchor.identifier,
                            AcceptedClientItem::PolledKeySet,
                        )
                    }),
            )
            .collect(),
    ));
    framework_providers.extend(oauth_client_framework_providers(
        oauth_client_bindings,
        routes,
    )?);
    framework_providers.extend(oidc_provider_framework_providers(
        accepted_clients
            .clients
            .iter()
            .map(|client| {
                accepted_client_item_path(
                    client.anchor.identifier,
                    AcceptedClientItem::RegisteredClient,
                )
            })
            .collect(),
        exchanger_bindings,
    ));

    Ok(framework_providers)
}

fn aggregate_consumer(aggregate: ProviderAggregate) -> OidcProviderItem {
    match aggregate {
        ProviderAggregate::AcceptedResources => OidcProviderItem::RevocationEndpoint,
        ProviderAggregate::ProviderSupport => OidcProviderItem::ProviderMetadataHandler,
    }
}

fn framework_modules(
    bindings: &ContainerBindings,
    declarations: &FrameworkDeclarations,
    exchanger_bindings: &[SubjectTokenExchangerBinding],
    features: &mut GeneratedFeatures,
) -> FrameworkModules {
    let jwks = build_jwks_artifacts(bindings);
    let trusted_issuers =
        build_trusted_issuer_artifacts(declarations.trusts, declarations.own_trusts);
    let oauth_clients = build_oauth_client_artifacts(declarations.oauth_client_bindings);
    let accepted_clients = render_accepted_clients(
        declarations.accepted_clients,
        &ProviderAggregate::ALL
            .into_iter()
            .filter(|aggregate| {
                bindings.provides(&oidc_provider_item_path(aggregate_consumer(*aggregate)))
            })
            .collect::<Vec<ProviderAggregate>>(),
    );
    let oidc_provider = build_oidc_provider_artifacts(bindings, exchanger_bindings);

    features.enable_if(GeneratedFeature::Jwks, jwks.enabled);
    features.enable_if(
        GeneratedFeature::TrustedIssuers,
        !trusted_issuers.is_empty(),
    );
    features.enable_if(GeneratedFeature::OAuthClients, !oauth_clients.is_empty());
    features.enable_if(
        GeneratedFeature::AcceptedClients,
        !accepted_clients.is_empty(),
    );
    features.enable_if(GeneratedFeature::OidcProvider, !oidc_provider.is_empty());

    let mut modules = jwks.modules;
    let mut services = jwks.services;

    modules.extend(trusted_issuers);
    modules.extend(oauth_clients);
    modules.extend(accepted_clients);
    modules.extend(oidc_provider);
    services.extend(issuer_directory_services(bindings));

    FrameworkModules { modules, services }
}

fn render_wrapper_modules<'tags>(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
    features: &GeneratedFeatures,
    tags: &'tags TagPool<'tags>,
) -> Result<WrapperModules<'tags>, CodegenError> {
    let mut modules: Vec<GeneratedModuleTokens> = Vec::new();
    let serves_requests = features.contains(GeneratedFeature::Http)
        || features.contains(GeneratedFeature::Websockets);
    let views_availability = if features.contains(GeneratedFeature::Views) {
        ViewsAvailability::Available
    } else {
        ViewsAvailability::Unavailable
    };
    let registries = BindingRegistries::collect(index, views_availability, tags, bindings)?;

    if features.contains(GeneratedFeature::AuthenticatedUsers) && serves_requests {
        modules.extend(render_authenticated_user_wrappers(&registries.providers()));
    }

    let middleware_plans = MiddlewarePlans::collect(index, &registries, tags)?;

    if features.contains(GeneratedFeature::Middleware) && serves_requests {
        modules.extend(render_middleware_wrappers(&middleware_plans.plans));
    }

    Ok(WrapperModules {
        middleware_plans,
        modules,
        registries,
    })
}

fn render_serving_modules(
    index: &AttributeIndex,
    declared_routes: DeclaredRoutes,
    bindings: &ContainerBindings,
    features: &GeneratedFeatures,
    middleware_plans: &MiddlewarePlans,
    registries: &BindingRegistries,
    provider: &ProviderDeclarations,
) -> Result<ServingModules, CodegenError> {
    let websockets = if features.contains(GeneratedFeature::Websockets) {
        render_websocket(
            WebSocketPlan::build(index, bindings, middleware_plans, registries)?,
            bindings,
        )
    } else {
        WebSocketArtifacts {
            modules: Vec::new(),
            retained_roots: Vec::new(),
            servers: BTreeMap::new(),
        }
    };
    let views = if features.contains(GeneratedFeature::Views) {
        render_views(ViewsPlan::build(index)?, bindings)
    } else {
        ViewsArtifacts {
            modules: Vec::new(),
            retained_roots: Vec::new(),
        }
    };
    let http = if features.contains(GeneratedFeature::Http)
        || features.contains(GeneratedFeature::Websockets)
    {
        let plan = HttpPlan::build(
            index,
            declared_routes,
            features.contains(GeneratedFeature::Views),
            &websockets.servers,
            middleware_plans,
            registries,
        )?;
        let provider_endpoints =
            provider_endpoint_modules(&plan.route_locations(), bindings, provider)?;

        HttpModules {
            artifacts: render_http(plan, bindings),
            provider_endpoints,
        }
    } else {
        HttpModules {
            artifacts: HttpArtifacts {
                retained_roots: Vec::new(),
                modules: Vec::new(),
                servers: Vec::new(),
            },
            provider_endpoints: provider_endpoint_modules(&[], bindings, provider)?,
        }
    };

    Ok(ServingModules {
        modules: [
            websockets.modules,
            views.modules,
            http.provider_endpoints.modules,
            http.artifacts.modules,
        ]
        .into_iter()
        .flatten()
        .collect(),
        retained_roots: [
            websockets.retained_roots,
            views.retained_roots,
            http.artifacts.retained_roots,
        ]
        .concat(),
        servers: http
            .artifacts
            .servers
            .into_iter()
            .map(|server| http.provider_endpoints.server.origin_of(server))
            .collect(),
    })
}

fn render_role_modules(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
    features: &GeneratedFeatures,
    framework_services: &[FrameworkService],
    servers: &[HttpServer],
    serving_roots: &[CanonicalPath],
) -> Result<RoleModules, CodegenError> {
    let mut modules: Vec<GeneratedModuleTokens> = Vec::new();

    let service_plan = ServicePlan::build(index, framework_services, bindings, serving_roots)?;
    let service_roots = service_plan.roots().to_vec();

    if features.contains(GeneratedFeature::Serves) {
        modules.push(render_services(
            &service_plan,
            servers,
            features.contains(GeneratedFeature::Views),
            bindings,
        ));
    }

    let console_roots = if features.contains(GeneratedFeature::Console) {
        let plan = ConsolePlan::build(index, bindings)?;
        let console = render_console(
            &plan,
            features.contains(GeneratedFeature::Serves),
            features.contains(GeneratedFeature::Schema),
            servers,
            service_plan.serve_inputs(),
            bindings,
        );
        modules.extend(console.modules);

        console.construction_roots
    } else {
        Vec::new()
    };

    Ok(RoleModules {
        console_roots,
        modules,
        service_roots,
    })
}

fn served_modules(
    index: &AttributeIndex,
    declared_routes: DeclaredRoutes,
    bindings: &ContainerBindings,
    features: &GeneratedFeatures,
    framework_services: &[FrameworkService],
    tags: &TagPool,
    provider: &ProviderDeclarations,
) -> Result<ServedModules, CodegenError> {
    let WrapperModules {
        middleware_plans,
        modules: wrapper_modules,
        registries,
    } = render_wrapper_modules(index, bindings, features, tags)?;
    let ServingModules {
        modules: serving_modules,
        retained_roots: serving_roots,
        servers,
    } = render_serving_modules(
        index,
        declared_routes,
        bindings,
        features,
        &middleware_plans,
        &registries,
        provider,
    )?;
    let RoleModules {
        console_roots,
        modules: role_modules,
        service_roots,
    } = render_role_modules(
        index,
        bindings,
        features,
        framework_services,
        &servers,
        &serving_roots,
    )?;

    Ok(ServedModules {
        modules: [wrapper_modules, serving_modules, role_modules]
            .into_iter()
            .flatten()
            .collect(),
        roots: RoleRoots {
            console: console_roots,
            serving: serving_roots,
            service: service_roots,
        },
    })
}

fn issuance_modules(
    token_issuance: &DeclaredTokenIssuance,
    resource_issuances: &DeclaredResourceIssuances,
    features: &mut GeneratedFeatures,
) -> Vec<GeneratedModuleTokens> {
    let mut modules = Vec::new();

    if let DeclaredTokenIssuance::Declared(declaration) = token_issuance {
        modules.push(render_token_issuance(declaration));
        features.enable(GeneratedFeature::TokenIssuance);
    }

    let resource_token_modules = render_resource_tokens(resource_issuances);

    features.enable_if(
        GeneratedFeature::ResourceTokens,
        !resource_token_modules.is_empty(),
    );
    modules.extend(resource_token_modules);

    modules
}

fn rendered(
    planned_container: PlannedContainer,
    RoleRoots {
        console,
        serving,
        service,
    }: &RoleRoots,
    mut module_tokens: Vec<GeneratedModuleTokens>,
    features: &GeneratedFeatures,
) -> Result<GeneratedCode, CodegenError> {
    let served_roots = service
        .iter()
        .chain(serving)
        .cloned()
        .collect::<BTreeSet<CanonicalPath>>()
        .into_iter()
        .collect::<Vec<CanonicalPath>>();

    planned_container
        .render(&served_roots, console)
        .map_err(CodegenError::from)
        .and_then(|rendered_container| {
            module_tokens.extend(rendered_container.modules);

            format_pass(module_tokens)
                .map_err(CodegenError::from)
                .map(|mut modules| {
                    modules.push(umbrella(features));

                    GeneratedCode::new(modules)
                })
        })
}

/// # Errors
///
/// Returns `CodegenError` propagated from the work it performs.
pub fn build(
    crate_root: &CrateRoot,
    metafile_contents: Option<&str>,
    assets_directory: &Path,
) -> Result<GeneratedCode, CodegenError> {
    let index = AttributeIndexBuilder::new()
        .exclude_root_module(UMBRELLA_MODULE_NAME)
        .index_crate(crate_root)?
        .build();
    let mut features = GeneratedFeatures::from_index(&index);
    let registry = scan(&index)?;
    let database = DeclaredPostgresDatabase::read(&index)?;
    let token_issuance = DeclaredTokenIssuance::read(&index)?;
    let trusts = DeclaredTrusts::read(&index)?;
    let oauth_clients = DeclaredOAuthClients::read(&index)?;
    let resource_issuances = DeclaredResourceIssuances::read(&index, &token_issuance)?;
    let accepted_clients =
        DeclaredAcceptedClients::read(&index, &token_issuance, &resource_issuances)?;
    let declared_routes = DeclaredRoutes::read(&index)?;
    let tags = TagPool::collect(
        &index,
        &trusts,
        &oauth_clients,
        &token_issuance,
        &resource_issuances,
        &accepted_clients,
    )?;
    let oauth_client_bindings =
        tags.oauth_client_bindings(&trusts, &oauth_clients, &token_issuance, &accepted_clients)?;
    let exchanger_bindings =
        tags.subject_token_exchanger_bindings(&index, &trusts, &accepted_clients)?;
    let own_trusts = own_trusts(
        &token_issuance,
        &resource_issuances,
        &tags,
        &oauth_client_bindings,
    );
    let framework_providers = framework_providers(
        &trusts,
        &own_trusts,
        &oauth_client_bindings,
        &accepted_clients,
        &exchanger_bindings,
        &declared_routes,
        &database,
    )?;
    let planned_container = plan_container(
        &index,
        &registry,
        &framework_providers,
        &database,
        &token_issuance,
    )?;
    let bindings = planned_container.bindings();
    let schema_models = SchemaModels::of(&index, &enabled_framework_schemas(bindings))?;
    let mut module_tokens =
        asset_bag_modules(&index, metafile_contents, bindings, assets_directory)?;
    features.enable_if(GeneratedFeature::AssetBag, !module_tokens.is_empty());
    features.enable_if(
        GeneratedFeature::Schema,
        !schema_models.framework.is_empty(),
    );
    features.enable_console(&index);

    if features.contains(GeneratedFeature::Schema) {
        module_tokens.push(render_schema(
            &schema_models.application,
            &schema_models.framework,
        ));
    }

    module_tokens.extend(issuance_modules(
        &token_issuance,
        &resource_issuances,
        &mut features,
    ));
    let FrameworkModules {
        modules: framework_modules,
        services: framework_services,
    } = framework_modules(
        bindings,
        &FrameworkDeclarations {
            accepted_clients: &accepted_clients,
            oauth_client_bindings: &oauth_client_bindings,
            own_trusts: &own_trusts,
            trusts: &trusts,
        },
        &exchanger_bindings,
        &mut features,
    );

    module_tokens.extend(framework_modules);

    let ServedModules { modules, roots } = served_modules(
        &index,
        declared_routes,
        bindings,
        &features,
        &framework_services,
        &tags,
        &ProviderDeclarations {
            accepted: &accepted_clients,
            token_issuance: &token_issuance,
        },
    )?;

    module_tokens.extend(modules);

    rendered(planned_container, &roots, module_tokens, &features)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use margaret_accepted_clients_codegen::accepted_clients_codegen_error::AcceptedClientsCodegenError;
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_attributes_tests::source_crate::SourceCrate;
    use margaret_container::container_error::ContainerError;
    use margaret_database_codegen::database_codegen_error::DatabaseCodegenError;
    use margaret_generated_module::generated_module::GeneratedModule;
    use margaret_http_codegen::http_codegen_error::HttpCodegenError;
    use margaret_tag_codegen::tag_error::TagError;
    use margaret_umbrella_path::umbrella_module_name::UMBRELLA_MODULE_NAME;

    use super::build;
    use crate::codegen_error::CodegenError;
    use crate::generated_code::GeneratedCode;

    const WEB_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]
struct Page;

impl Page {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    const PLAIN_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[console_command(name = \"inspect\")]
struct Config;

impl Config {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<CommandOutcome> {}
}
";

    const COMMAND_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[console_command(name = \"greet\")]
struct Greet;

impl Greet {
    #[process]
    fn run(&self) -> anyhow::Result<CommandOutcome> {}
}
";

    const DATABASE_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/articles\", server = \"public\")]
struct ListArticles {
    database: std::sync::Arc<margaret::framework::database::database::Database>,
}

impl ListArticles {
    #[constructor]
    fn create(database: std::sync::Arc<margaret::framework::database::database::Database>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[postgres_database(url_from = \"APPLICATION_DATABASE_URL\")]
struct ApplicationDatabase;
";

    const MODELS_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[model(table = \"widgets\")]
struct Widget {
    #[column(primary_key, name = \"id\")]
    id: uuid::Uuid,
}
";

    const WEBSOCKET_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[singleton]
struct SystemClock;

impl SystemClock {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}
}

#[websocket_session(path = \"/room/{name}\", server = \"public\")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(clock: std::sync::Arc<SystemClock>, #[route_parameter(from = \"name\")] name: String) -> anyhow::Result<Self> {}
}

#[websocket_message(request, method = \"chat\", response = margaret::framework::websocket::web_socket_response::WebSocketResponse::Single)]
struct Chat;

#[singleton]
struct Chatter;

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
";

    const INVALID_WEBSOCKET_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[websocket_session(path = \"/x\", server = \"public\")]
struct Room;
";

    fn bearer_route(tag: &str) -> String {
        format!(
            "mod {tag}_caller {{\n    use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;\n    use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;\n    use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;\n\n    pub struct Claims;\n\n    pub struct Caller;\n\n    #[singleton]\n    #[infers_authenticated_user(user_model = Caller)]\n    pub struct CallerProvider;\n\n    impl CallerProvider {{\n        #[infer_from_request]\n        fn infer(&self, #[bearer_token(issuer = {tag})] token: Option<VerifiedJwt<Claims, IdTokenProfile>>) -> anyhow::Result<AuthenticatedUserOutcome<Caller>> {{}}\n    }}\n\n    #[singleton]\n    #[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/{tag}\", server = \"public\")]\n    pub struct CallerPage;\n\n    impl CallerPage {{\n        #[process]\n        fn respond(&self, #[authenticated_user] caller: Caller) -> anyhow::Result<Response> {{}}\n    }}\n}}\n"
        )
    }

    fn generate(lib_source: &str) -> Result<GeneratedCode, CodegenError> {
        let source_crate = SourceCrate::new(lib_source);

        build(
            &CrateRoot::new("crate", source_crate.source_directory()),
            None,
            &source_crate.root().join("assets"),
        )
    }

    fn generate_from_source(source: &Path) -> GeneratedCode {
        build(
            &CrateRoot::new("crate", source),
            None,
            &source.join("assets"),
        )
        .expect("the crate generates")
    }

    fn module<'code>(code: &'code GeneratedCode, name: &str) -> &'code str {
        code.modules()
            .iter()
            .find(|module| module.name() == name)
            .expect("the requested module is generated")
            .source()
    }

    fn has_module(code: &GeneratedCode, name: &str) -> bool {
        code.modules().iter().any(|module| module.name() == name)
    }

    fn concatenated(code: &GeneratedCode) -> String {
        code.modules()
            .iter()
            .map(GeneratedModule::source)
            .collect::<Vec<&str>>()
            .join("\n")
    }

    #[test]
    fn generates_container_and_server_when_responders_exist() {
        let code = generate(WEB_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod container;"));
        assert!(module(&code, "mod").contains("pub mod http;"));
        assert!(module(&code, "mod").contains("pub mod routes;"));
        assert!(module(&code, "mod").contains("pub mod run;"));
        assert!(concatenated(&code).contains("container: &super::super::container::Container"));
        assert!(concatenated(&code).contains("fn server"));
        assert!(!concatenated(&code).contains("async fn server"));
        assert!(module(&code, "routes").contains("pub struct Routes"));
        assert!(module(&code, "container").contains("struct Container"));
        assert!(module(&code, "run").contains("\"serve\""));
    }

    #[test]
    fn generates_a_console_for_commands_without_http() {
        let code = generate(COMMAND_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod run;"));
        assert!(!module(&code, "mod").contains("pub mod http;"));
        assert!(!has_module(&code, "http"));
        assert!(module(&code, "run").contains("\"greet\""));
    }

    #[test]
    fn omits_the_asset_macro_of_a_crate_that_never_imports_it() {
        let source_crate = SourceCrate::new(PLAIN_CRATE);

        let code = build(
            &CrateRoot::new("crate", source_crate.source_directory()),
            Some(r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#),
            &source_crate.root().join("assets"),
        )
        .expect("the build succeeds");

        assert!(!module(&code, "mod").contains("pub mod asset_bag;"));
        assert!(!has_module(&code, "asset_bag"));
    }

    const ASSET_MACRO_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

mod views {
    use crate::margaret::asset_bag::asset;
}

#[singleton]
#[console_command(name = \"inspect\")]
struct Config;

impl Config {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<CommandOutcome> {}
}
";

    #[test]
    fn generates_the_asset_macro_a_module_imports() {
        let source_crate = SourceCrate::new(ASSET_MACRO_CRATE);

        let code = build(
            &CrateRoot::new("crate", source_crate.source_directory()),
            Some(r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#),
            &source_crate.root().join("assets"),
        )
        .expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod asset_bag;"));
        assert!(module(&code, "asset_bag").contains("macro_rules! asset"));
        assert!(!module(&code, "asset_bag").contains("pub mod asset_responder"));
        assert!(!has_module(&code, "asset_bag/asset_responder"));
    }

    const ASSET_RESPONDER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/assets\", server = \"public\")]
struct AssetRoute {
    responder: std::sync::Arc<crate::margaret::asset_bag::asset_responder::AssetResponder>,
}

impl AssetRoute {
    #[constructor]
    fn create(
        responder: std::sync::Arc<crate::margaret::asset_bag::asset_responder::AssetResponder>,
    ) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn generates_the_asset_responder_when_a_constructor_injects_it() {
        let source_crate = SourceCrate::new(ASSET_RESPONDER_CRATE);
        let assets = source_crate.root().join("assets");
        fs::create_dir(&assets).expect("the assets directory exists");
        fs::write(assets.join("app_ABC.js"), "console.log(1)")
            .expect("the fingerprinted asset exists");
        fs::write(
            assets.join("service_worker.js"),
            "self.addEventListener('install', () => {})",
        )
        .expect("the un-fingerprinted asset exists");

        let code = build(
            &CrateRoot::new("crate", source_crate.source_directory()),
            Some(r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#),
            &assets,
        )
        .expect("the build succeeds");

        let responder = module(&code, "asset_bag/asset_responder");

        assert!(module(&code, "asset_bag").contains("pub mod asset_responder;"));
        assert!(responder.contains("pub struct AssetResponder"));
        assert!(responder.contains("\"app_ABC.js\" =>"));
        assert!(responder.contains("public, max-age=31536000, immutable"));
        assert!(responder.contains("\"service_worker.js\" =>"));
        assert!(responder.contains("\"no-cache\""));
        assert!(
            module(&code, "container/build/serve")
                .contains("asset_bag::asset_responder::AssetResponder")
        );
    }

    #[test]
    fn generates_the_asset_macro_beside_the_injected_responder() {
        let source_crate = SourceCrate::new(&format!(
            "{ASSET_RESPONDER_CRATE}\nmod views {{\n    use crate::margaret::asset_bag::asset;\n}}\n"
        ));
        let assets = source_crate.root().join("assets");
        fs::create_dir(&assets).expect("the assets directory exists");
        fs::write(assets.join("app_ABC.js"), "console.log(1)")
            .expect("the fingerprinted asset exists");

        let code = build(
            &CrateRoot::new("crate", source_crate.source_directory()),
            Some(r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#),
            &assets,
        )
        .expect("the build succeeds");
        let asset_bag = module(&code, "asset_bag");

        assert!(asset_bag.contains("pub mod asset_responder;"));
        assert!(asset_bag.contains("macro_rules! asset"));
        assert!(has_module(&code, "asset_bag/asset_responder"));
    }

    #[test]
    fn reports_a_missing_asset_directory_when_the_responder_is_injected() {
        let source_crate = SourceCrate::new(ASSET_RESPONDER_CRATE);

        let message = build(
            &CrateRoot::new("crate", source_crate.source_directory()),
            Some(r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#),
            &source_crate.root().join("assets"),
        )
        .expect_err("a missing asset directory is rejected")
        .to_string();

        assert!(message.contains("failed to read the asset directory"));
    }

    #[test]
    fn reports_an_injected_asset_responder_without_a_metafile() {
        let message = generate(ASSET_RESPONDER_CRATE)
            .expect_err("an injected asset responder without a metafile is rejected")
            .to_string();

        assert!(message.contains("no esbuild metafile was found in the manifest directory"));
    }

    #[test]
    fn omits_the_asset_bag_module_without_a_metafile() {
        let code = generate(PLAIN_CRATE).expect("the build succeeds");

        assert!(!module(&code, "mod").contains("pub mod asset_bag;"));
        assert!(!has_module(&code, "asset_bag"));
    }

    #[test]
    fn reports_an_imported_asset_macro_without_a_metafile() {
        assert_eq!(
            generate(ASSET_MACRO_CRATE)
                .expect_err("an imported asset macro without a metafile is rejected")
                .to_string(),
            "a module imports the asset macro, but no esbuild metafile was found in the manifest directory"
        );
    }

    #[test]
    fn propagates_an_asset_bag_failure() {
        let source_crate = SourceCrate::new(ASSET_MACRO_CRATE);

        let message = build(
            &CrateRoot::new("crate", source_crate.source_directory()),
            Some(r#"{ "outputs": {} }"#),
            &source_crate.root().join("assets"),
        )
        .expect_err("an invalid metafile is rejected")
        .to_string();

        assert!(message.contains("failed to generate the asset bag"));
    }

    const JWKS_ROLLER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/.well-known/jwks.json\", server = \"internal\")]
struct GetJwks {
    handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>,
}

impl GetJwks {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[postgres_database(url_from = \"APPLICATION_DATABASE_URL\")]
struct ApplicationDatabase;
";

    const JWKS_MULTI_CLIENT_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;


#[verifies_tokens_from_issuer(auth, audience = \"api\", issuer = \"https://auth.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = \"https://auth.example/jwks\"))]
struct AuthJwksEndpoint;



#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = \"https://partner.example/jwks\"))]
struct PartnerJwksEndpoint;



#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/verify\", server = \"public\")]
struct GetVerify;

impl GetVerify {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    const JWKS_SERVER_STORE_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;


#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.fixture\")]
struct Issuer;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Post, path = \"/mint\", server = \"internal\")]
struct PostMint {
    minter: std::sync::Arc<crate::margaret::jwks::MintAccessTokenHandler>,
    store: std::sync::Arc<crate::margaret::jwks::JwksSecretStore>,
}

impl PostMint {
    #[constructor]
    fn create(
        minter: std::sync::Arc<crate::margaret::jwks::MintAccessTokenHandler>,
        store: std::sync::Arc<crate::margaret::jwks::JwksSecretStore>,
    ) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[postgres_database(url_from = \"APPLICATION_DATABASE_URL\")]
struct ApplicationDatabase;
";

    const SPIFFE_HTTP_CLIENT_CRATE: &str = "\
use reqwest::Client;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

#[singleton]
struct OutboundCaller {
    client: Client,
}

impl OutboundCaller {
    #[constructor]
    fn create(#[spiffe_http_client] client: Client) -> anyhow::Result<Self> {}
}

#[service]
struct Worker {
    caller: Arc<OutboundCaller>,
}

impl Worker {
    #[constructor]
    fn create(caller: Arc<OutboundCaller>) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self, token: CancellationToken) -> anyhow::Result<()> {}
}
";

    #[test]
    fn provisions_a_spiffe_http_client_for_a_client_only_service() {
        let code = generate(SPIFFE_HTTP_CLIENT_CRATE).expect("the build succeeds");

        let serve: String = module(&code, "serve").split_whitespace().collect();
        assert!(serve.contains(
            "letspiffe_bundle=matchmargaret::framework::spiffe_svid_client::svid_client_bundle::SvidClientBundle::new(margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams{"
        ));
        assert!(serve.contains("letspiffe_client_readiness=spiffe_bundle.client_readiness();"));
        assert!(serve.contains(
            "letspiffe_http_client=matchspiffe_bundle.reqwest_client(){Ok(client)=>client,Err(error)=>{returnmargaret::framework::console::report_failure::report_failure(error);}};"
        ));
        assert!(serve.contains(
            "ifletErr(error)=manager.register_bundle(spiffe_bundle).await{returnmargaret::framework::console::report_failure::report_failure(error);}"
        ));
        assert!(serve.contains("letserve_input_0=spiffe_http_client.clone();"));
        assert!(serve.contains(
            "manager.register_service(margaret::framework::spiffe_svid_client::readiness_gated_service::ReadinessGatedService::new(spiffe_client_readiness.clone(),Worker{inner:container.worker(),},),);"
        ));
        assert!(serve.contains(
            "margaret::framework::service::run::run(manager,cancellation_token,trzcina::ServiceShutdownOptions::default(),).await"
        ));
        assert!(!serve.contains("run_all"));
        assert!(!serve.contains("spiffe_identity_manager"));
        assert!(!serve.contains("wait_until_ready"));
        assert!(!serve.contains("bundle_services"));
        assert!(!serve.contains("spiffe_server_config"));

        let construction: String = module(&code, "container/build/serve_arguments")
            .split_whitespace()
            .collect();
        assert!(construction.contains("pubargument0:reqwest::Client,"));

        let run: String = module(&code, "run").split_whitespace().collect();
        assert!(run.contains(
            r#"clap::Arg::new("spiffe-trust-domain").long("spiffe-trust-domain").required(true)"#
        ));
        assert!(run.contains(
            r#"clap::Arg::new("spire-agent-addr").long("spire-agent-addr").required(true)"#
        ));
    }

    const SPIFFE_HTTP_SERVER_CLIENT_CRATE: &str = "\
use reqwest::Client;
use std::sync::Arc;

#[singleton]
struct OutboundCaller {
    client: Client,
}

impl OutboundCaller {
    #[constructor]
    fn create(#[spiffe_http_client] client: Client) -> anyhow::Result<Self> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/call\", server = \"public\")]
struct CallRoute {
    caller: Arc<OutboundCaller>,
}

impl CallRoute {
    #[constructor]
    fn create(caller: Arc<OutboundCaller>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn registers_the_client_bundle_alongside_a_plain_http_server() {
        let code = generate(SPIFFE_HTTP_SERVER_CLIENT_CRATE).expect("the build succeeds");

        let serve: String = module(&code, "serve").split_whitespace().collect();
        assert!(serve.contains(
            "letspiffe_bundle=matchmargaret::framework::spiffe_svid_client::svid_client_bundle::SvidClientBundle::new(margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams{"
        ));
        assert!(serve.contains(
            "ifletErr(error)=manager.register_bundle(spiffe_bundle).await{returnmargaret::framework::console::report_failure::report_failure(error);}"
        ));
        assert!(serve.contains(
            "letserver_services=margaret::framework::service::serve_application::serve_application(matches,servers,)?;"
        ));
        assert!(serve.contains(
            "forserver_serviceinserver_services{manager.register_service(margaret::framework::spiffe_svid_client::readiness_gated_service::ReadinessGatedService::new(spiffe_client_readiness.clone(),server_service,),);}"
        ));
        assert!(serve.contains(
            "margaret::framework::service::run::run(manager,cancellation_token,trzcina::ServiceShutdownOptions::default(),).await"
        ));
        assert!(!serve.contains("run_all"));
        assert!(!serve.contains("bundle_services"));
        assert!(!serve.contains("spiffe_server_config"));
    }

    const SPIFFE_BOTH_CRATE: &str = "\
use reqwest::Client;
use spiffe::spiffe_id::SpiffeId;
use std::sync::Arc;

#[singleton]
struct OutboundCaller {
    client: Client,
}

impl OutboundCaller {
    #[constructor]
    fn create(#[spiffe_http_client] client: Client) -> anyhow::Result<Self> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/identity\", server = \"internal\")]
struct GetIdentity {
    caller: Arc<OutboundCaller>,
}

impl GetIdentity {
    #[constructor]
    fn create(caller: Arc<OutboundCaller>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self, peer: &SpiffeId) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn shares_one_svid_bundle_when_a_server_is_pinned_and_a_client_is_injected() {
        let code = generate(SPIFFE_BOTH_CRATE).expect("the build succeeds");

        let serve: String = module(&code, "serve").split_whitespace().collect();
        assert!(serve.contains(
            "letspiffe_bundle=matchmargaret::framework::spiffe_svid_bundle::svid_bundle::SvidBundle::new(margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams{"
        ));
        assert!(serve.contains(
            "letspiffe_server_config=::std::sync::Arc::new(spiffe_bundle.server_config());"
        ));
        assert!(serve.contains("letspiffe_client_readiness=spiffe_bundle.client_readiness();"));
        assert!(serve.contains(
            "letspiffe_http_client=matchspiffe_bundle.reqwest_client(){Ok(client)=>client,Err(error)=>{returnmargaret::framework::console::report_failure::report_failure(error);}};"
        ));
        assert!(serve.contains(
            "ifletErr(error)=manager.register_bundle(spiffe_bundle).await{returnmargaret::framework::console::report_failure::report_failure(error);}"
        ));
        assert!(serve.contains(
            "transport:matchmatches.get_one::<margaret::framework::service::transport_choice::TransportChoice"
        ));
        assert!(serve.contains("{Some(value)=>value.config(spiffe_server_config),"));
        assert!(serve.contains(
            "forserver_serviceinserver_services{manager.register_service(margaret::framework::spiffe_svid_client::readiness_gated_service::ReadinessGatedService::new(spiffe_client_readiness.clone(),server_service,),);}"
        ));
        assert!(serve.contains(
            "margaret::framework::service::run::run(manager,cancellation_token,trzcina::ServiceShutdownOptions::default(),).await"
        ));
        assert!(serve.matches("SvidBundle::new").count() == 1);
        assert!(!serve.contains("run_all"));
        assert!(!serve.contains("bundle_services"));
    }

    #[test]
    fn rejects_a_spiffe_http_client_marker_with_arguments() {
        let error = generate(
            "use reqwest::Client;\n\n#[singleton]\nstruct Bad {\n    client: Client,\n}\n\nimpl Bad {\n    #[constructor]\n    fn create(#[spiffe_http_client(extra)] client: Client) -> anyhow::Result<Self> {}\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(error.contains("the injected client takes no arguments"));
    }

    #[test]
    fn rejects_a_parameter_that_is_both_a_spiffe_http_client_and_a_console_argument() {
        let error = generate(
            "#[singleton]\nstruct Bad {\n    endpoint: String,\n}\n\nimpl Bad {\n    #[constructor]\n    fn create(#[spiffe_http_client] #[console_argument(from = \"endpoint\")] endpoint: String) -> anyhow::Result<Self> {}\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(error.contains("a constructor parameter is fed by exactly one serve input"));
    }

    #[test]
    fn rejects_a_console_command_that_injects_the_spiffe_http_client() {
        let error = generate(
            "use reqwest::Client;\nuse std::sync::Arc;\n\n#[singleton]\nstruct OutboundCaller {\n    client: Client,\n}\n\nimpl OutboundCaller {\n    #[constructor]\n    fn create(#[spiffe_http_client] client: Client) -> anyhow::Result<Self> {}\n}\n\n#[singleton]\n#[console_command(name = \"call\")]\nstruct Call {\n    caller: Arc<OutboundCaller>,\n}\n\nimpl Call {\n    #[constructor]\n    fn create(caller: Arc<OutboundCaller>) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(error.contains("injects the #[spiffe_http_client]"));
    }

    #[test]
    fn allows_a_serve_input_named_after_the_spiffe_http_client() {
        let code = generate(
            "use reqwest::Client;\nuse std::sync::Arc;\nuse tokio_util::sync::CancellationToken;\n\n#[singleton]\nstruct IdentityClient {\n    client: Client,\n}\n\nimpl IdentityClient {\n    #[constructor]\n    fn create(#[spiffe_http_client] client: Client) -> anyhow::Result<Self> {}\n}\n\n#[singleton]\nstruct Labeled {\n    label: String,\n}\n\nimpl Labeled {\n    #[constructor]\n    fn create(#[console_argument(from = \"spiffe_http_client\")] label: String) -> anyhow::Result<Self> {}\n}\n\n#[service]\nstruct Worker {\n    client: Arc<IdentityClient>,\n    labeled: Arc<Labeled>,\n}\n\nimpl Worker {\n    #[constructor]\n    fn create(client: Arc<IdentityClient>, labeled: Arc<Labeled>) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn run(&self, token: CancellationToken) -> anyhow::Result<()> {}\n}\n",
        )
        .expect("the framework binding and a console argument of the same name coexist");

        let run: String = module(&code, "run").split_whitespace().collect();
        assert!(run.contains(r#"clap::Arg::new("spiffe_http_client").long("spiffe_http_client")"#));
    }

    #[test]
    fn rejects_a_singleton_that_injects_the_jwks_roller_it_could_restart() {
        assert_eq!(
            generate(&format!(
                "{JWKS_ROLLER_CRATE}#[singleton]\n#[console_command(name = \"roll\")]\nstruct Roll {{\n    roller: std::sync::Arc<crate::margaret::jwks::JwksRoller>,\n}}\n\nimpl Roll {{\n    #[constructor]\n    fn create(roller: std::sync::Arc<crate::margaret::jwks::JwksRoller>) -> anyhow::Result<Self> {{}}\n\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {{}}\n}}\n"
            ))
            .expect_err("only the framework runs the jwks roller")
            .to_string(),
            "failed to generate the dependency container: parameter 'roller' of singleton 'crate::Roll' injects 'crate::margaret::jwks::JwksRoller', which only the framework may inject"
        );
    }

    #[test]
    fn generates_the_jwks_roller_when_the_handler_is_injected() {
        let code = generate(JWKS_ROLLER_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod jwks;"));
        assert!(
            module(&code, "jwks").contains(
                "pub use margaret::framework::jwks_roller_server::jwks_roller::JwksRoller;"
            )
        );
        assert!(module(&code, "jwks").contains(
            "pub use margaret::framework::jwks_roller_server::public_jwks_handler::PublicJwksHandler;"
        ));

        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();
        assert!(construction.contains("crate::margaret::jwks::JwksRoller::create("));
        assert!(construction.contains(
            "::std::sync::Arc::new(margaret::framework::jwks_keygen::generated_rsa_signing_keys::GeneratedRsaSigningKeys,)"
        ));
        assert!(construction.contains(".public_jwks_handler()"));
        assert!(construction.contains(
            "crate::margaret::jwks::JwksRoller::create(::std::sync::Arc::<margaret::framework::signing_keys_database::database_signing_keys::DatabaseSigningKeys,>::clone(&framework_signing_keys_database_database_signing_keys_database_signing_keys,),"
        ));

        let serve: String = module(&code, "serve").split_whitespace().collect();
        assert!(serve.contains("impltrzcina::ServiceforMargaretJwksJwksRoller"));
        assert!(
            serve.contains(
                "self.inner.run(cancellation_token).await?;::std::result::Result::Ok(())"
            )
        );
        assert!(!serve.contains("impltrzcina::TickerforMargaretJwksJwksRoller"));
    }

    #[test]
    fn generates_a_trusted_issuer_per_published_key_set() {
        let code = generate(&format!(
            "{JWKS_MULTI_CLIENT_CRATE}{}{}",
            bearer_route("auth"),
            bearer_route("partner")
        ))
        .expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(module(&code, "mod").contains("pub mod trusted_issuers;"));
        assert!(module(&code, "trusted_issuers").contains("pub mod auth;"));
        assert!(module(&code, "trusted_issuers").contains("pub mod partner;"));
        assert!(module(&code, "trusted_issuers/auth").contains(
            "pub use margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer;"
        ));
        assert!(
            module(&code, "trusted_issuers/auth/jwks_endpoint_issuer")
                .contains("jwks_uri: \"https://auth.example/jwks\",")
        );
        assert!(construction.contains(
            "crate::margaret::trusted_issuers::auth::PolledKeySet::published(crate::margaret::trusted_issuers::auth::jwks_endpoint_issuer::JWKS_ENDPOINT_ISSUER,::std::sync::Arc::<crate::margaret::trusted_issuers::auth::IssuerKeySet,>::clone(&margaret_trusted_issuers_auth_issuer_key_set),)"
        ));
        assert!(construction.contains(
            "crate::margaret::trusted_issuers::auth::TrustedIssuer::polled(::std::sync::Arc::<crate::margaret::trusted_issuers::auth::IssuerKeySet,>::clone(&margaret_trusted_issuers_auth_issuer_key_set),crate::margaret::trusted_issuers::auth::token_trust::TOKEN_TRUST,)"
        ));
        assert!(!concatenated(&code).contains("PublicJwksHandler"));
    }

    #[test]
    fn polls_every_trusted_issuer_through_one_issuer_directory() {
        let code = generate(&format!(
            "{JWKS_MULTI_CLIENT_CRATE}{}{}",
            bearer_route("auth"),
            bearer_route("partner")
        ))
        .expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();
        let serve: String = module(&code, "serve").split_whitespace().collect();

        assert!(construction.contains(
            "letframework_issuer_directory_issuer_directory_issuer_directory=::std::sync::Arc::new(margaret::framework::issuer_directory::issuer_directory::IssuerDirectory::create(::std::sync::Arc::<margaret::framework::issuer_request::issuer_request_client::IssuerRequestClient,>::clone(&framework_issuer_request_issuer_request_client_issuer_request_client,),::std::vec::Vec::from([::std::sync::Arc::<crate::margaret::trusted_issuers::auth::PolledKeySet,>::clone(&margaret_trusted_issuers_auth_polled_key_set),::std::sync::Arc::<crate::margaret::trusted_issuers::partner::PolledKeySet,>::clone(&margaret_trusted_issuers_partner_polled_key_set),]),),);"
        ));
        assert!(construction.contains(
            "margaret::framework::issuer_request::issuer_request_client::IssuerRequestClient::create().map_err(margaret::framework::anyhow::Error::from),)?;"
        ));
        assert_eq!(serve.matches("impltrzcina::Servicefor").count(), 1);
        assert!(serve.contains(
            "impltrzcina::ServiceforFrameworkIssuerDirectoryIssuerDirectoryIssuerDirectory"
        ));
    }

    #[test]
    fn generates_the_server_secret_store_and_mint_handler_when_injected() {
        let code = generate(JWKS_SERVER_STORE_CRATE).expect("the build succeeds");

        assert!(
            module(&code, "jwks").contains(
                "pub use margaret::framework::jwks_roller_server::jwks_roller::JwksRoller;"
            )
        );
        assert!(module(&code, "jwks").contains(
            "pub use margaret::framework::jwks_secret_store::jwks_secret_store::JwksSecretStore;"
        ));
        assert!(module(&code, "jwks").contains(
            "pub use margaret::framework::access_token_minter::mint_access_token_handler::MintAccessTokenHandler;"
        ));

        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();
        assert!(construction.contains("crate::margaret::jwks::JwksSecretStore::create("));
        assert!(construction.contains("crate::margaret::token_issuance::TOKEN_ISSUANCE,"));
        assert!(construction.contains("crate::margaret::jwks::MintAccessTokenHandler::create("));
    }

    const OIDC_ISSUERS_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

mod first {

    #[verifies_tokens_from_issuer(first, audience = \"api\", issuer = \"https://first.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]
    pub struct Issuer;

}

mod second {

    #[verifies_tokens_from_issuer(second, audience = \"api\", issuer = \"https://second.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]
    pub struct Issuer;

}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]
struct Page;

impl Page {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn generates_a_trusted_issuer_per_trusted_oidc_issuer() {
        let code = generate(&format!(
            "{OIDC_ISSUERS_CRATE}{}{}",
            bearer_route("first"),
            bearer_route("second")
        ))
        .expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(module(&code, "trusted_issuers").contains("pub mod first;"));
        assert!(module(&code, "trusted_issuers").contains("pub mod second;"));
        assert!(construction.contains(
            "letmargaret_trusted_issuers_first_issuer_metadata=::std::sync::Arc::new(crate::margaret::trusted_issuers::first::IssuerMetadata::awaiting(),);"
        ));
        assert!(construction.contains(
            "crate::margaret::trusted_issuers::first::PolledKeySet::discovered(crate::margaret::trusted_issuers::first::discovered_issuer::DISCOVERED_ISSUER,::std::sync::Arc::<crate::margaret::trusted_issuers::first::IssuerMetadata,>::clone(&margaret_trusted_issuers_first_issuer_metadata),::std::sync::Arc::<crate::margaret::trusted_issuers::first::IssuerKeySet,>::clone(&margaret_trusted_issuers_first_issuer_key_set),)"
        ));
        assert!(construction.contains(
            "crate::margaret::trusted_issuers::second::TrustedIssuer::polled(::std::sync::Arc::<crate::margaret::trusted_issuers::second::IssuerKeySet,>::clone(&margaret_trusted_issuers_second_issuer_key_set),crate::margaret::trusted_issuers::second::token_trust::TOKEN_TRUST,)"
        ));
    }

    const OAUTH_CLIENT_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]
struct PartnerIssuer;


#[acts_as_oauth_client(partner_client, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::ClientSecretBasic(client_secret_from = \"PARTNER_CLIENT_SECRET\"), client_id = \"partner\", issuer = partner)]
struct PartnerClient;

#[singleton]
struct Uploader {
    credentials: std::sync::Arc<crate::margaret::oauth_clients::partner_client::ClientCredentials>,
}

impl Uploader {
    #[constructor]
    fn create(
        credentials: std::sync::Arc<crate::margaret::oauth_clients::partner_client::ClientCredentials>,
    ) -> anyhow::Result<Self> {}
}

struct Claims;

struct ArtifactUploader;

#[singleton]
#[infers_authenticated_user(user_model = ArtifactUploader)]
struct ArtifactUploaderProvider;

impl ArtifactUploaderProvider {
    #[infer_from_request]
    fn infer(
        &self,
        #[bearer_token(client = partner_client)] token: Option<
            margaret::framework::token_introspection::introspected_token::IntrospectedToken<Claims>,
        >,
    ) -> anyhow::Result<AuthenticatedUserOutcome<ArtifactUploader>> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]
struct Page {
    uploader: std::sync::Arc<crate::Uploader>,
}

impl Page {
    #[constructor]
    fn create(uploader: std::sync::Arc<crate::Uploader>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self, #[authenticated_user] artifact_uploader: ArtifactUploader) -> anyhow::Result<Response> {}
}
";

    const SIGN_IN_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;


#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.fixture\")]
struct Issuer;

#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]
struct PartnerIssuer;


#[acts_as_oauth_client(partner_client, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"partner\", issuer = partner, sign_in(redirect_route = SignInCallback, scopes = [\"profile\"]))]
struct PartnerClient;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/sign-in/callback\", server = \"public\")]
struct SignInCallback;

impl SignInCallback {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/sign-in\", server = \"public\")]
struct SignIn {
    flow: std::sync::Arc<crate::margaret::oauth_clients::partner_client::SignInFlow>,
}

impl SignIn {
    #[constructor]
    fn create(
        flow: std::sync::Arc<crate::margaret::oauth_clients::partner_client::SignInFlow>,
    ) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[postgres_database(url_from = \"APPLICATION_DATABASE_URL\")]
struct ApplicationDatabase;
";

    const OIDC_PROVIDER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;

#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.fixture\")]
struct Issuer;
#[issues_resource_tokens(artifacts, audience = \"artifacts\")]
struct ArtifactsResource;
#[issues_resource_tokens(reports, audience = \"reports\")]
struct ReportsResource;

#[verifies_tokens_from_issuer(ci, audience = \"api\", issuer = \"https://ci.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]
struct CiIssuer;


#[singleton]
#[exchanges_tokens_from(issuer = ci)]
struct CiExchanger;

impl ExchangesSubjectTokens for CiExchanger {}

#[admits_oauth_client(
    portal_app,
    authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt(introspection, keys = margaret::framework::accepted_clients::client_keys::ClientKeys::Published(jwks_uri = \"https://portal.fixture/jwks.json\", signing = margaret::framework::jose_parameters::jws_algorithm::JwsAlgorithm::Es256)),
    authorization_code(consent = margaret::framework::accepted_clients::consent_policy::ConsentPolicy::Prompted, id_token_signing = margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning::Rsa, redirect_uris = [\"https://portal.fixture/callback\"], scopes = [\"openid\"], refresh_token),
    client_id = \"portal\",
    resources = [artifacts],
    token_exchange(scopes = [\"deploy\"]),
)]
struct PortalClient;

#[admits_oauth_client(spa_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"spa\", resources = [reports])]
struct SpaClient;

#[admits_oauth_client(service_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt(keys = margaret::framework::accepted_clients::client_keys::ClientKeys::Own), client_id = \"service\", resources = [reports])]
struct ServiceClient;

#[acts_as_oauth_client(service_client, admitted_as = service_app)]
struct OwnServiceClient;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/caller\", server = \"public\")]
struct Caller;

impl Caller {
    #[constructor]
    fn create(credentials: std::sync::Arc<crate::margaret::oauth_clients::service_client::ClientCredentials>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[admits_oauth_client(kiosk_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, authorization_code(consent = margaret::framework::accepted_clients::consent_policy::ConsentPolicy::Implicit, id_token_signing = margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning::EllipticCurve, redirect_uris = [\"https://kiosk.fixture/callback\"], scopes = [\"openid\"]), client_id = \"kiosk\", resources = [reports])]
struct KioskClient;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/authorize\", server = \"public\")]
struct GetAuthorize;

impl GetAuthorize {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::AuthorizationEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Post, path = \"/authorize\", server = \"public\")]
struct PostAuthorize;

impl PostAuthorize {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::AuthorizationEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/.well-known/openid-configuration\", server = \"public\")]
struct GetDiscovery;

impl GetDiscovery {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::ProviderMetadataHandler>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Post, path = \"/introspect\", server = \"public\")]
struct PostIntrospect;

impl PostIntrospect {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::IntrospectionEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Post, path = \"/revoke\", server = \"public\")]
struct PostRevoke;

impl PostRevoke {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::RevocationEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Post, path = \"/token\", server = \"public\")]
struct PostToken;

impl PostToken {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::TokenEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/userinfo\", server = \"public\")]
struct GetUserinfo;

impl GetUserinfo {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::UserinfoEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/jwks.json\", server = \"public\")]
struct GetJwks;

impl GetJwks {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Post, path = \"/consent\", server = \"public\")]
struct PostConsent;

impl PostConsent {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::ConsentEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}


#[postgres_database(url_from = \"APPLICATION_DATABASE_URL\")]
struct ApplicationDatabase;
";

    #[test]
    fn generates_the_endpoints_of_an_oidc_provider() {
        let code = generate(OIDC_PROVIDER_CRATE).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(!has_module(&code, "trusted_issuers/artifacts"));
        assert!(!has_module(&code, "trusted_issuers/reports"));
        assert!(module(&code, "mod").contains("pub mod oidc_provider;"));
        assert!(module(&code, "mod").contains("pub mod token_issuance;"));
        assert!(module(&code, "token_issuance").contains("issuer: \"https://issuer.fixture\","));
        assert_eq!(
            module(
                &code,
                "oidc_provider/provider_endpoints/authorization_endpoint_url"
            ),
            "pub const AUTHORIZATION_ENDPOINT_URL: &str = \"https://issuer.fixture/authorize\";\n"
        );
        assert!(
            module(&code, "oidc_provider/provider_endpoints")
                .split_whitespace()
                .collect::<String>()
                .contains("authorization:margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Served(authorization_endpoint_url::AUTHORIZATION_ENDPOINT_URL,),")
        );
        assert!(construction.contains(
            "crate::margaret::oidc_provider::AuthorizationEndpoint::create(::std::sync::Arc::<crate::margaret::oidc_provider::AcceptedClients,>::clone(&margaret_oidc_provider_accepted_clients),crate::margaret::oidc_provider::provider_endpoints::authorization_endpoint_url::AUTHORIZATION_ENDPOINT_URL,crate::margaret::token_issuance::TOKEN_ISSUANCE,)"
        ));
        assert!(module(&code, "oidc_provider/subject_token_exchangers").contains("pub mod ci;"));
        assert!(construction.contains(
            "crate::margaret::oidc_provider::AcceptedClients::create(::std::vec::Vec::from([::std::sync::Arc::<crate::margaret::accepted_clients::clients::kiosk_client::RegisteredClient,>::clone(&margaret_accepted_clients_clients_kiosk_client_registered_client,),::std::sync::Arc::<crate::margaret::accepted_clients::clients::portal_client::RegisteredClient,>::clone(&margaret_accepted_clients_clients_portal_client_registered_client,),::std::sync::Arc::<crate::margaret::accepted_clients::clients::service_client::RegisteredClient,>::clone(&margaret_accepted_clients_clients_service_client_registered_client,),::std::sync::Arc::<crate::margaret::accepted_clients::clients::spa_client::RegisteredClient,>::clone(&margaret_accepted_clients_clients_spa_client_registered_client),]),crate::margaret::token_issuance::TOKEN_ISSUANCE,)"
        ));
        assert!(construction.contains(
            "crate::margaret::oidc_provider::IntrospectionEndpoint::create(::std::sync::Arc::<crate::margaret::oidc_provider::AcceptedClients,>::clone(&margaret_oidc_provider_accepted_clients),::std::sync::Arc::<crate::margaret::jwks::JwksSecretStore,>::clone(&margaret_jwks_jwks_secret_store),)"
        ));
        assert!(construction.contains(
            "crate::margaret::oidc_provider::subject_token_exchangers::ci::SubjectTokenExchanger::create(::std::sync::Arc::<crate::margaret::trusted_issuers::ci::TrustedIssuer,>::clone(&margaret_trusted_issuers_ci_trusted_issuer),::std::sync::Arc::<crate::CiExchanger>::clone(&ci_exchanger),)"
        ));
        assert!(construction.contains(
            "crate::margaret::oidc_provider::ConsentEndpoint::create(::std::sync::Arc::<margaret::framework::authorization_grants_database::database_authorization_grants::DatabaseAuthorizationGrants,>::clone(&framework_authorization_grants_database_database_authorization_grants_database_authorization_grants,),crate::margaret::token_issuance::TOKEN_ISSUANCE,)"
        ));
        assert!(construction.contains(
            "crate::margaret::oidc_provider::RevocationEndpoint::create(::std::sync::Arc::<crate::margaret::oidc_provider::AcceptedClients,>::clone(&margaret_oidc_provider_accepted_clients),::std::sync::Arc::<crate::margaret::jwks::JwksSecretStore,>::clone(&margaret_jwks_jwks_secret_store),::std::sync::Arc::<margaret::framework::authorization_grants_database::database_authorization_grants::DatabaseAuthorizationGrants,>::clone(&framework_authorization_grants_database_database_authorization_grants_database_authorization_grants,),crate::margaret::accepted_clients::accepted_resources::ACCEPTED_RESOURCES,)"
        ));
        assert!(
            module(&code, "serve")
                .split_whitespace()
                .collect::<String>()
                .contains("letserve_input_0=matchmargaret::framework::environment_variable::read_required::read_required::<margaret::framework::database::database_url::DatabaseUrl,>(\"APPLICATION_DATABASE_URL\")")
        );
        assert!(!module(&code, "serve").contains("serve_input_1"));
        assert!(module(&code, "schema").contains("framework_tables"));
        assert!(module(&code, "schema").contains("\"pending_authorizations\""));
        assert!(module(&code, "schema").contains("\"client_assertions\""));
        assert!(module(&code, "schema").contains("\"signing_key_sets\""));
        assert!(
            module(&code, "serve")
                .split_whitespace()
                .collect::<String>()
                .contains(
                    "letorigin_public:::std::sync::Arc<str>=::std::sync::Arc::from(crate::margaret::oidc_provider::provider_endpoints::PROVIDER_ENDPOINTS.issuer_origin,);letroutes=::std::sync::Arc::new(super::routes::Routes::from_origins(::std::sync::Arc::clone(&origin_public)),);"
                )
        );
        assert!(!module(&code, "run").contains("public-url"));
    }

    #[test]
    fn registers_each_accepted_client_by_how_it_authenticates() {
        let code = generate(OIDC_PROVIDER_CRATE).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(module(&code, "mod").contains("pub mod accepted_clients;"));
        assert!(construction.contains(
            "crate::margaret::accepted_clients::clients::portal_client::RegisteredClient::private_key_jwt_with_code_grant(crate::margaret::accepted_clients::clients::portal_client::accepted_client::ACCEPTED_CLIENT,crate::margaret::accepted_clients::clients::portal_client::confidential_privileges::CONFIDENTIAL_PRIVILEGES,::std::sync::Arc::<crate::margaret::accepted_clients::clients::portal_client::ClientKeySet,>::clone(&margaret_accepted_clients_clients_portal_client_client_key_set),::std::sync::Arc::<margaret::framework::client_assertions_database::database_client_assertions::DatabaseClientAssertions,>::clone(&framework_client_assertions_database_database_client_assertions_database_client_assertions,),crate::margaret::accepted_clients::clients::portal_client::code_grant_policy::CODE_GRANT_POLICY,::std::vec::Vec::from([::std::string::String::from(\"https://portal.fixture/callback\"),]),::std::sync::Arc::<margaret::framework::authorization_grants_database::database_authorization_grants::DatabaseAuthorizationGrants,>::clone(&framework_authorization_grants_database_database_authorization_grants_database_authorization_grants,),)"
        ));
        assert!(construction.contains(
            "crate::margaret::accepted_clients::clients::service_client::RegisteredClient::private_key_jwt(crate::margaret::accepted_clients::clients::service_client::accepted_client::ACCEPTED_CLIENT,crate::margaret::accepted_clients::clients::service_client::confidential_privileges::CONFIDENTIAL_PRIVILEGES,::std::sync::Arc::<crate::margaret::accepted_clients::clients::service_client::ClientKeySet,>::clone(&margaret_accepted_clients_clients_service_client_client_key_set),::std::sync::Arc::<margaret::framework::client_assertions_database::database_client_assertions::DatabaseClientAssertions,>::clone(&framework_client_assertions_database_database_client_assertions_database_client_assertions,),)"
        ));
        assert!(construction.contains(
            "crate::margaret::accepted_clients::clients::kiosk_client::RegisteredClient::public_with_code_grant(crate::margaret::accepted_clients::clients::kiosk_client::accepted_client::ACCEPTED_CLIENT,crate::margaret::accepted_clients::clients::kiosk_client::code_grant_policy::CODE_GRANT_POLICY,::std::vec::Vec::from([::std::string::String::from(\"https://kiosk.fixture/callback\"),]),::std::sync::Arc::<margaret::framework::authorization_grants_database::database_authorization_grants::DatabaseAuthorizationGrants,>::clone(&framework_authorization_grants_database_database_authorization_grants_database_authorization_grants,),)"
        ));
        assert!(!module(&code, "serve").contains("serve_input_1"));
        assert!(construction.contains(
            "crate::margaret::accepted_clients::clients::spa_client::RegisteredClient::public(crate::margaret::accepted_clients::clients::spa_client::accepted_client::ACCEPTED_CLIENT,)"
        ));
    }

    #[test]
    fn verifies_each_confidential_client_by_the_keys_it_declares() {
        let construction: String = module(
            &generate(OIDC_PROVIDER_CRATE).expect("the build succeeds"),
            "container/build/serve",
        )
        .split_whitespace()
        .collect();

        assert!(construction.contains(
            "crate::margaret::accepted_clients::clients::portal_client::ClientKeySet::published(::std::sync::Arc::<crate::margaret::accepted_clients::clients::portal_client::IssuerKeySet,>::clone(&margaret_accepted_clients_clients_portal_client_issuer_key_set),crate::margaret::accepted_clients::clients::portal_client::assertion_signing::ASSERTION_SIGNING,)"
        ));
        assert!(construction.contains(
            "crate::margaret::accepted_clients::clients::service_client::ClientKeySet::own(::std::sync::Arc::<crate::margaret::jwks::JwksSecretStore,>::clone(&margaret_jwks_jwks_secret_store),)"
        ));
    }

    #[test]
    fn polls_the_keys_of_confidential_clients_beside_trusted_issuers() {
        let construction: String = module(
            &generate(OIDC_PROVIDER_CRATE).expect("the build succeeds"),
            "container/build/serve",
        )
        .split_whitespace()
        .collect();

        assert!(construction.contains(
            "crate::margaret::accepted_clients::clients::portal_client::PolledKeySet::published(crate::margaret::accepted_clients::clients::portal_client::jwks_endpoint_issuer::JWKS_ENDPOINT_ISSUER,::std::sync::Arc::<crate::margaret::accepted_clients::clients::portal_client::IssuerKeySet,>::clone(&margaret_accepted_clients_clients_portal_client_issuer_key_set),)"
        ));
        assert!(construction.contains(
            "margaret::framework::issuer_directory::issuer_directory::IssuerDirectory::create(::std::sync::Arc::<margaret::framework::issuer_request::issuer_request_client::IssuerRequestClient,>::clone(&framework_issuer_request_issuer_request_client_issuer_request_client,),::std::vec::Vec::from([::std::sync::Arc::<crate::margaret::trusted_issuers::ci::PolledKeySet,>::clone(&margaret_trusted_issuers_ci_polled_key_set),::std::sync::Arc::<crate::margaret::accepted_clients::clients::portal_client::PolledKeySet,>::clone(&margaret_accepted_clients_clients_portal_client_polled_key_set),]),)"
        ));
    }

    #[test]
    fn describes_the_provider_by_the_aggregates_of_its_accepted_clients() {
        let construction: String = module(
            &generate(OIDC_PROVIDER_CRATE).expect("the build succeeds"),
            "container/build/serve",
        )
        .split_whitespace()
        .collect();

        assert!(construction.contains(
            "crate::margaret::oidc_provider::ProviderMetadataHandler::create(crate::margaret::accepted_clients::provider_support::PROVIDER_SUPPORT,crate::margaret::oidc_provider::provider_endpoints::PROVIDER_ENDPOINTS,crate::margaret::token_issuance::TOKEN_ISSUANCE,)"
        ));
        assert!(construction.contains(
            "crate::margaret::accepted_clients::accepted_resources::ACCEPTED_RESOURCES,)"
        ));
    }

    #[test]
    fn rejects_an_accepted_client_of_an_application_that_issues_no_tokens() {
        let error = generate(
            &OIDC_PROVIDER_CRATE
                .replace(
                    "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.fixture\")]\nstruct Issuer;\n",
                    "",
                )
                .replace(
                    "#[issues_resource_tokens(artifacts, audience = \"artifacts\")]\nstruct ArtifactsResource;\n#[issues_resource_tokens(reports, audience = \"reports\")]\nstruct ReportsResource;\n",
                    "",
                ),
        )
        .expect_err("an accepted client needs a token issuance");

        assert!(matches!(
            error,
            CodegenError::AcceptedClients {
                source: AcceptedClientsCodegenError::AcceptedWithoutTokenIssuance { anchor }
            } if anchor == "crate::KioskClient"
        ));
    }

    #[test]
    fn injects_provider_handlers_imported_through_use_statements() {
        let imported = OIDC_PROVIDER_CRATE
            .replace(
                "use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;\n",
                "use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;\nuse std::sync::Arc;\n\nuse crate::margaret::jwks::PublicJwksHandler;\nuse crate::margaret::oidc_provider;\n",
            )
            .replace("std::sync::Arc<crate::margaret::oidc_provider::", "Arc<oidc_provider::")
            .replace("std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>", "Arc<PublicJwksHandler>");
        let expected = generate(OIDC_PROVIDER_CRATE).expect("the build succeeds");
        let code = generate(&imported).expect("the build succeeds");

        assert!(!imported.contains("crate::margaret::oidc_provider::"));
        assert_eq!(
            module(&code, "oidc_provider/provider_endpoints"),
            module(&expected, "oidc_provider/provider_endpoints")
        );
        assert_eq!(
            module(&code, "container/build/serve"),
            module(&expected, "container/build/serve")
        );
    }

    #[test]
    fn rejects_an_oidc_provider_without_a_discovery_route() {
        let error = generate(&OIDC_PROVIDER_CRATE.replace(
            "crate::margaret::oidc_provider::ProviderMetadataHandler",
            "crate::margaret::oidc_provider::TokenEndpoint",
        ))
        .expect_err("the provider metadata needs a route");

        assert_eq!(
            error.to_string(),
            "failed to derive the openid connect provider endpoints: no route serves the discovery document"
        );
    }

    #[test]
    fn rejects_an_oidc_provider_of_an_application_without_routes() {
        let error = generate(
            "\
#[rustfmt::skip]
pub mod margaret;

#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.fixture\")]
struct Issuer;

#[issues_resource_tokens(artifacts, audience = \"artifacts\")]
struct ArtifactsResource;

#[admits_oauth_client(spa_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"spa\", resources = [artifacts])]
struct SpaClient;
",
        )
        .expect_err("an admitted client needs the provider routes");

        assert_eq!(
            error.to_string(),
            "failed to derive the openid connect provider endpoints: no route serves the discovery document"
        );
    }

    #[test]
    fn rejects_a_provider_route_of_an_application_that_admits_no_client() {
        let error = generate(
            "\
#[rustfmt::skip]
pub mod margaret;

#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.fixture\")]
struct Issuer;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/.well-known/openid-configuration\", server = \"public\")]
struct GetDiscovery;

impl GetDiscovery {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::ProviderMetadataHandler>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
",
        )
        .expect_err("a provider without admitted clients serves no discovery document");

        assert_eq!(
            error.to_string(),
            "failed to derive the openid connect provider endpoints: the discovery document is routed, but the provider admits no oauth client"
        );
    }

    #[test]
    fn generates_the_sign_in_flow_of_an_oauth_client() {
        let code = generate(SIGN_IN_CRATE).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(construction.contains(
            "crate::margaret::oauth_clients::partner_client::SignInFlow::create(::std::sync::Arc::<crate::margaret::oauth_clients::partner_client::AuthorizationServerClient,>::clone(&margaret_oauth_clients_partner_client_authorization_server_client,),::std::sync::Arc::<crate::margaret::jwks::JwksRoller,>::clone(&margaret_jwks_jwks_roller),serve_input_1,crate::margaret::oauth_clients::partner_client::sign_in_scopes::SIGN_IN_SCOPES,).map_err(margaret::framework::anyhow::Error::from)"
        ));
        assert!(
            module(&code, "serve")
                .split_whitespace()
                .collect::<String>()
                .contains(
                    "letserve_input_1=margaret::framework::http::build_url::build_url(&origin_public,&[margaret::framework::http::url_segment::UrlSegment::Literal(\"/sign-in/callback\",),],);"
                )
        );
    }

    #[test]
    fn rejects_the_sign_in_flow_of_a_client_that_declares_no_sign_in() {
        assert_eq!(
            generate(&SIGN_IN_CRATE.replace(
                ", sign_in(redirect_route = SignInCallback, scopes = [\"profile\"])",
                "",
            ))
            .expect_err("the client does not sign in")
            .to_string(),
            "failed to generate the dependency container: parameter 'flow' of singleton 'crate::SignIn' depends on 'crate :: margaret :: oauth_clients :: partner_client :: SignInFlow', which no singleton provides"
        );
    }

    #[test]
    fn rejects_a_sign_in_that_redirects_to_a_route_of_another_method() {
        assert_eq!(
            generate(&SIGN_IN_CRATE.replace(
                "RouteMethod::Get, path = \"/sign-in/callback\"",
                "RouteMethod::Post, path = \"/sign-in/callback\"",
            ))
            .expect_err("the redirect route does not respond to GET")
            .to_string(),
            "failed to generate the http server: the oauth client 'crate::PartnerClient' redirects to 'crate::SignInCallback', which does not respond to RouteMethod::Get"
        );
    }

    #[test]
    fn rejects_a_confidential_client_that_redirects_to_a_route_of_another_method() {
        assert_eq!(
            generate(&format!(
                "{OIDC_PROVIDER_CRATE}{}",
                OWN_SIGN_IN.replace(
                    "RouteMethod::Get, path = \"/blog/callback\"",
                    "RouteMethod::Post, path = \"/blog/callback\"",
                )
            ))
            .expect_err("the redirect route does not respond to GET")
            .to_string(),
            "failed to generate the http server: the oauth client 'crate::BlogApp' redirects to 'crate::GetBlogCallback', which does not respond to RouteMethod::Get"
        );
    }

    #[test]
    fn rejects_a_public_client_that_redirects_to_a_route_of_another_method() {
        assert_eq!(
            generate(&OIDC_PROVIDER_CRATE.replace(
                "redirect_uris = [\"https://kiosk.fixture/callback\"]",
                "redirect_routes = [PostAuthorize]",
            ))
            .expect_err("the redirect route does not respond to GET")
            .to_string(),
            "failed to generate the http server: the oauth client 'crate::KioskClient' redirects to 'crate::PostAuthorize', which does not respond to RouteMethod::Get"
        );
    }

    #[test]
    fn propagates_an_http_route_failure() {
        assert_eq!(
            generate("#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n#[singleton]\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n")
                .expect_err("two responders answer the same route")
                .to_string(),
            "failed to generate the http server: responder 'crate::Second' registers 'Get /x' on server 'public', which is already registered by responder 'crate::First'"
        );
    }

    #[test]
    fn propagates_a_resource_issuance_failure() {
        assert_eq!(
            generate("#[rustfmt::skip]\npub mod margaret;\n\n#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.fixture\")]\nstruct Issuer;\n#[issues_resource_tokens(reports, audience = \"\")]\nstruct ReportsResource;\n")
                .expect_err("the resource audience is empty")
                .to_string(),
            "failed to read the token issuance: #[issues_resource_tokens] on 'crate::ReportsResource' declares an empty audience"
        );
    }

    #[test]
    fn signs_in_an_application_that_issues_no_tokens() {
        let code = generate(&SIGN_IN_CRATE.replace(
            "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.fixture\")]\nstruct Issuer;\n",
            "",
        ))
        .expect("the sign-in flow signs its transactions with the rolled keys");

        assert!(!has_module(&code, "token_issuance"));
        assert!(module(&code, "container/build/serve").contains("SignInFlow::create("));
    }

    #[test]
    fn generates_the_runtime_of_an_oauth_client_of_a_discovered_issuer() {
        let code = generate(OAUTH_CLIENT_CRATE).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(module(&code, "mod").contains("pub mod oauth_clients;"));
        assert!(module(&code, "oauth_clients").contains("pub mod partner_client;"));
        assert!(module(&code, "oauth_clients/partner_client").contains(
            "pub use margaret::framework::client_credentials::client_credentials::ClientCredentials;"
        ));
        assert!(construction.contains(
            "crate::margaret::oauth_clients::partner_client::AuthorizationServerClient::with_client_secret_basic(::std::sync::Arc::<margaret::framework::issuer_request::issuer_request_client::IssuerRequestClient,>::clone(&framework_issuer_request_issuer_request_client_issuer_request_client,),::std::sync::Arc::<crate::margaret::trusted_issuers::partner::IssuerMetadata,>::clone(&margaret_trusted_issuers_partner_issuer_metadata),::std::sync::Arc::<crate::margaret::trusted_issuers::partner::TrustedIssuer,>::clone(&margaret_trusted_issuers_partner_trusted_issuer),crate::margaret::oauth_clients::partner_client::client_id::CLIENT_ID,serve_input_0"
        ));
        assert!(construction.contains(
            "crate::margaret::oauth_clients::partner_client::ClientCredentials::create("
        ));
        assert!(!construction.contains("TokenExchange::create("));

        let server: String = module(&code, "http/server_public")
            .split_whitespace()
            .collect();

        assert!(server.contains(
            "margaret_oauth_clients_partner_client_authorization_server_client:container.margaret_oauth_clients_partner_client_authorization_server_client(),"
        ));
        assert!(server.contains(
            "margaret::framework::identity::require_bearer_authenticated_user::require_bearer_authenticated_user("
        ));
    }

    #[test]
    fn generates_the_client_of_the_own_provider_from_its_admitted_client() {
        let code = generate(OIDC_PROVIDER_CRATE).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert_eq!(
            module(&code, "oauth_clients/service_client/client_id"),
            "pub const CLIENT_ID: &str = \"service\";\n"
        );
        assert!(construction.contains(
            "crate::margaret::oidc_provider::IssuerMetadata::of_provider(crate::margaret::oidc_provider::provider_endpoints::PROVIDER_ENDPOINTS,)"
        ));
        assert!(construction.contains(
            "crate::margaret::trusted_issuers::provider::TrustedIssuer::own(::std::sync::Arc::<crate::margaret::jwks::JwksSecretStore,>::clone(&margaret_jwks_jwks_secret_store),crate::margaret::trusted_issuers::provider::token_trust::TOKEN_TRUST,)"
        ));
        assert!(construction.contains(
            "crate::margaret::oauth_clients::service_client::AuthorizationServerClient::with_private_key_jwt(::std::sync::Arc::<margaret::framework::issuer_request::issuer_request_client::IssuerRequestClient,>::clone(&framework_issuer_request_issuer_request_client_issuer_request_client,),::std::sync::Arc::<crate::margaret::oidc_provider::IssuerMetadata,>::clone(&margaret_oidc_provider_issuer_metadata),::std::sync::Arc::<crate::margaret::trusted_issuers::provider::TrustedIssuer,>::clone(&margaret_trusted_issuers_provider_trusted_issuer),crate::margaret::oauth_clients::service_client::client_id::CLIENT_ID,::std::sync::Arc::<crate::margaret::jwks::JwksRoller,>::clone(&margaret_jwks_jwks_roller),)"
        ));
    }

    const OWN_SIGN_IN: &str = "#[admits_oauth_client(blog_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt(keys = margaret::framework::accepted_clients::client_keys::ClientKeys::Own), authorization_code(consent = margaret::framework::accepted_clients::consent_policy::ConsentPolicy::Implicit, id_token_signing = margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning::Rsa, redirect_routes = [GetBlogCallback], scopes = [\"openid\", \"profile\"]), client_id = \"blog\", resources = [reports])]\nstruct BlogApp;\n#[singleton]\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/blog/callback\", server = \"public\")]\nstruct GetBlogCallback {\n    flow: std::sync::Arc<crate::margaret::oauth_clients::blog::SignInFlow>,\n}\nimpl GetBlogCallback {\n    #[constructor]\n    fn create(flow: std::sync::Arc<crate::margaret::oauth_clients::blog::SignInFlow>) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n#[acts_as_oauth_client(blog, admitted_as = blog_app)]\nstruct BlogClient;\n";

    #[test]
    fn signs_the_own_client_in_through_the_route_its_admitted_client_redirects_to() {
        let code =
            generate(&format!("{OIDC_PROVIDER_CRATE}{OWN_SIGN_IN}")).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(
            module(&code, "serve")
                .split_whitespace()
                .collect::<String>()
                .contains(
                    "letserve_input_1=margaret::framework::http::build_url::build_url(&origin_public,&[margaret::framework::http::url_segment::UrlSegment::Literal(\"/blog/callback\")],);"
                )
        );
        assert!(!module(&code, "serve").contains("serve_input_2"));
        assert!(construction.contains(
            "crate::margaret::accepted_clients::clients::blog_app::code_grant_policy::CODE_GRANT_POLICY,::std::vec::Vec::from([serve_input_1.clone()]),"
        ));
        assert!(construction.contains(
            "crate::margaret::oauth_clients::blog::SignInFlow::create(::std::sync::Arc::<crate::margaret::oauth_clients::blog::AuthorizationServerClient,>::clone(&margaret_oauth_clients_blog_authorization_server_client),::std::sync::Arc::<crate::margaret::jwks::JwksRoller,>::clone(&margaret_jwks_jwks_roller),serve_input_1,crate::margaret::oauth_clients::blog::sign_in_scopes::SIGN_IN_SCOPES,)"
        ));
        assert_eq!(
            module(&code, "oauth_clients/blog/sign_in_scopes"),
            "pub const SIGN_IN_SCOPES: &[&str] = &[\"openid\", \"profile\"];\n"
        );
    }

    #[test]
    fn reads_the_client_secret_from_the_declared_environment_variable() {
        let code: String = concatenated(&generate(OAUTH_CLIENT_CRATE).expect("the build succeeds"))
            .split_whitespace()
            .collect();

        assert!(code.contains(
            "margaret::framework::environment_variable::read_required::read_required::<margaret::framework::oauth_vocabulary::client_secret::ClientSecret,>(\"PARTNER_CLIENT_SECRET\")"
        ));
    }

    #[test]
    fn rejects_an_oauth_client_of_an_issuer_without_discovery() {
        assert!(matches!(
            generate(
                "#[rustfmt::skip]\npub mod margaret;\n\n#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = \"https://partner.example/jwks\"))]\nstruct Endpoint;\n\n#[acts_as_oauth_client(partner_client, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"partner\", issuer = partner)]\nstruct PartnerClient;\n"
            ),
            Err(CodegenError::Tag {
                source: TagError::OAuthClientIssuerNotDiscovered { ref issuer, .. }
            }) if issuer == "partner"
        ));
    }

    #[test]
    fn rejects_a_subject_token_exchanger_of_an_undeclared_issuer() {
        assert!(matches!(
            generate(
                "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[exchanges_tokens_from(issuer = ci)]\nstruct CiExchanger;\n"
            ),
            Err(CodegenError::Tag {
                source: TagError::UnknownTag { ref tag, .. }
            }) if tag == "ci"
        ));
    }

    const OIDC_BEARER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;

#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]
struct Issuer;


struct Claims;

struct Runner;

#[singleton]
#[infers_authenticated_user(user_model = Runner)]
struct RunnerProvider;

impl RunnerProvider {
    #[infer_from_request]
    fn infer(&self, #[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Claims, IdTokenProfile>>) -> anyhow::Result<AuthenticatedUserOutcome<Runner>> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/runner\", server = \"public\")]
struct RunnerPage;

impl RunnerPage {
    #[process]
    fn respond(&self, #[authenticated_user] runner: Runner) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn hands_the_wrapper_of_a_bearer_route_the_trusted_issuer_of_its_oidc_issuer() {
        let code = generate(OIDC_BEARER_CRATE).expect("the build succeeds");
        let server: String = module(&code, "http/server_public")
            .split_whitespace()
            .collect();

        assert!(server.contains(
            "margaret_trusted_issuers_partner_trusted_issuer:container.margaret_trusted_issuers_partner_trusted_issuer(),"
        ));
        assert!(server.contains(
            "margaret::framework::identity::require_bearer_authenticated_user::require_bearer_authenticated_user("
        ));
    }

    #[test]
    fn rejects_the_trusted_issuer_injected_by_path() {
        let error = generate(
            "\
#[rustfmt::skip]
pub mod margaret;


#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]
struct Issuer;


#[singleton]
struct Consumer {
    trusted_issuer: std::sync::Arc<crate::margaret::trusted_issuers::partner::TrustedIssuer>,
}

impl Consumer {
    #[constructor]
    fn create(trusted_issuer: std::sync::Arc<crate::margaret::trusted_issuers::partner::TrustedIssuer>) -> anyhow::Result<Self> {}
}
",
        )
        .expect_err("the trusted issuer is framework-only")
        .to_string();

        assert!(error.contains("which only the framework may inject"));
    }

    const OWN_BEARER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;

#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.fixture\")]
struct Issuer;

#[issues_resource_tokens(attachments, audience = \"attachments\")]
struct AttachmentsResource;

#[postgres_database(url_from = \"APPLICATION_DATABASE_URL\")]
struct ApplicationDatabase;

struct Claims;

struct Holder;

#[singleton]
#[infers_authenticated_user(user_model = Holder)]
struct HolderProvider;

impl HolderProvider {
    #[infer_from_request]
    fn infer(
        &self,
        #[bearer_token(issuer = provider)] session: Option<VerifiedJwt<Claims, AccessTokenProfile>>,
        #[bearer_token(resource = attachments)] upload: Option<VerifiedJwt<Claims, AccessTokenProfile>>,
    ) -> anyhow::Result<AuthenticatedUserOutcome<Holder>> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/holder\", server = \"public\")]
struct HolderPage;

impl HolderPage {
    #[process]
    fn respond(&self, #[authenticated_user] holder: Holder) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn verifies_own_bearer_tokens_with_the_own_keys() {
        let code = generate(OWN_BEARER_CRATE).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(construction.contains(
            "crate::margaret::trusted_issuers::provider::TrustedIssuer::own(::std::sync::Arc::<crate::margaret::jwks::JwksSecretStore,>::clone(&margaret_jwks_jwks_secret_store),crate::margaret::trusted_issuers::provider::token_trust::TOKEN_TRUST,)"
        ));
        assert!(construction.contains(
            "crate::margaret::trusted_issuers::attachments::TrustedIssuer::own(::std::sync::Arc::<crate::margaret::jwks::JwksSecretStore,>::clone(&margaret_jwks_jwks_secret_store),crate::margaret::trusted_issuers::attachments::token_trust::TOKEN_TRUST,)"
        ));
        assert_eq!(
            module(&code, "trusted_issuers/attachments/token_trust"),
            "pub const TOKEN_TRUST: margaret::framework::token_trust::token_trust::TokenTrust = margaret::framework::token_trust::token_trust::TokenTrust {\n    audience: \"attachments\",\n    issuer: \"https://issuer.fixture\",\n};\n"
        );
        assert_eq!(
            module(&code, "resource_tokens/attachments"),
            "pub const AUDIENCE: &str = \"attachments\";\n"
        );
    }

    #[test]
    fn rejects_a_resource_issuance_nothing_uses() {
        assert_eq!(
            generate(&OWN_BEARER_CRATE.replace(
                "#[bearer_token(resource = attachments)] upload: Option<VerifiedJwt<Claims, AccessTokenProfile>>,",
                "",
            ))
            .expect_err("nothing uses the attachments resource")
            .to_string(),
            "failed to collect the tags: the resource 'attachments' declared by 'crate::AttachmentsResource' is never used: no admitted client is granted it and no bearer token is addressed to it"
        );
    }

    const JWKS_BEARER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;

#[verifies_tokens_from_issuer(auth, audience = \"api\", issuer = \"https://auth.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = \"https://auth.example/jwks\"))]
struct AuthJwksEndpoint;



struct Claims;

struct Holder;

#[singleton]
#[infers_authenticated_user(user_model = Holder)]
struct HolderProvider;

impl HolderProvider {
    #[infer_from_request]
    fn infer(&self, #[bearer_token(issuer = auth)] token: Option<VerifiedJwt<Claims, AccessTokenProfile>>) -> anyhow::Result<AuthenticatedUserOutcome<Holder>> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/holder\", server = \"public\")]
struct HolderPage;

impl HolderPage {
    #[process]
    fn respond(&self, #[authenticated_user] holder: Holder) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn hands_the_wrapper_of_a_bearer_route_the_trusted_issuer_of_its_jwks_endpoint() {
        let code = generate(JWKS_BEARER_CRATE).expect("the build succeeds");
        let server: String = module(&code, "http/server_public")
            .split_whitespace()
            .collect();

        assert!(server.contains(
            "margaret_trusted_issuers_auth_trusted_issuer:container.margaret_trusted_issuers_auth_trusted_issuer(),"
        ));
    }

    const JWKS_NAME_COLLISION_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct JwksRoller;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/.well-known/jwks.json\", server = \"internal\")]
struct GetJwks {
    handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>,
    roller: std::sync::Arc<crate::JwksRoller>,
}

impl GetJwks {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>, roller: std::sync::Arc<crate::JwksRoller>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[postgres_database(url_from = \"APPLICATION_DATABASE_URL\")]
struct ApplicationDatabase;
";

    #[test]
    fn framework_jwks_names_do_not_collide_with_a_user_component_of_the_same_name() {
        let code = generate(JWKS_NAME_COLLISION_CRATE)
            .expect("a user component named JwksRoller coexists");

        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(construction.contains("letjwks_roller=::std::sync::Arc::new(crate::JwksRoller);"));
        assert!(construction.contains(
            "letmargaret_jwks_jwks_roller=margaret::framework::construct_singleton::construct_singleton(\"crate::margaret::jwks::JwksRoller\","
        ));
    }

    const JWKS_FIELD_COLLISION_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

pub mod margaret_jwks {
    #[singleton]
    pub struct JwksRoller;
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/.well-known/jwks.json\", server = \"internal\")]
struct GetJwks {
    handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>,
    roller: std::sync::Arc<crate::margaret_jwks::JwksRoller>,
}

impl GetJwks {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>, roller: std::sync::Arc<crate::margaret_jwks::JwksRoller>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[postgres_database(url_from = \"APPLICATION_DATABASE_URL\")]
struct ApplicationDatabase;
";

    #[test]
    fn framework_jwks_field_is_disambiguated_from_a_colliding_user_component() {
        let code = generate(JWKS_FIELD_COLLISION_CRATE)
            .expect("a user component flattening to a framework field coexists");

        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(construction.contains(
            "letmargaret_jwks_jwks_roller=::std::sync::Arc::new(crate::margaret_jwks::JwksRoller,);"
        ));
        assert!(construction.contains(
            "letmargaret_jwks_jwks_roller_2=margaret::framework::construct_singleton::construct_singleton(\"crate::margaret::jwks::JwksRoller\","
        ));
    }

    #[test]
    fn omits_the_jwks_module_when_neither_is_used() {
        let code = generate(WEB_CRATE).expect("the build succeeds");

        assert!(!module(&code, "mod").contains("pub mod jwks;"));
        assert!(!has_module(&code, "jwks"));
        assert!(!concatenated(&code).contains("JwksRoller"));
        assert!(!concatenated(&code).contains("JwksClient"));
    }

    #[test]
    fn rejects_a_bearer_token_referencing_an_unknown_issuer() {
        let message = generate(&JWKS_BEARER_CRATE.replace(
            "#[bearer_token(issuer = auth)]",
            "#[bearer_token(issuer = missing)]",
        ))
        .expect_err("a bearer token without a matching trusted issuer is rejected")
        .to_string();

        assert!(message.contains(
            "references the tag 'missing', which no token issuance or trusted issuer declares"
        ));
    }

    const JWKS_DUPLICATE_ENDPOINT_TAG_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;


#[verifies_tokens_from_issuer(auth, audience = \"api\", issuer = \"https://first.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = \"https://first.example/jwks\"))]
struct FirstEndpoint;



#[verifies_tokens_from_issuer(auth, audience = \"api\", issuer = \"https://second.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = \"https://second.example/jwks\"))]
struct SecondEndpoint;


";

    #[test]
    fn propagates_a_duplicate_jwks_endpoint_tag() {
        let message = generate(JWKS_DUPLICATE_ENDPOINT_TAG_CRATE)
            .expect_err("a duplicate jwks endpoint tag is rejected")
            .to_string();

        assert!(message.contains("declared more than once"));
    }

    #[test]
    fn generates_no_server_without_responders() {
        let code = generate(PLAIN_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod container;"));
        assert!(!module(&code, "mod").contains("pub mod http;"));
        assert!(!module(&code, "mod").contains("pub mod routes;"));
        assert!(!has_module(&code, "http"));
        assert!(!has_module(&code, "routes"));
        assert!(module(&code, "container").contains("struct Container"));
    }

    #[test]
    fn rejects_a_singleton_nothing_consumes() {
        assert_eq!(
            generate(
                "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create() -> anyhow::Result<Self> {}\n}\n"
            )
            .expect_err("nothing consumes the singleton")
            .to_string(),
            "failed to generate the dependency container: the singleton 'crate::Config' is never used: no route, service, ticker, console command, websocket session or middleware reaches it"
        );
    }

    #[test]
    fn generates_the_websocket_module_for_websocket_sessions() {
        let code = generate(WEBSOCKET_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod websocket;"));
        assert!(module(&code, "mod").contains("pub mod http;"));
        assert!(has_module(&code, "websocket"));
        assert!(concatenated(&code).contains("WebSocketRequestMessage"));
        assert!(concatenated(&code).contains("public_routes"));
    }

    #[test]
    fn propagates_a_web_socket_codegen_error() {
        let error = generate(INVALID_WEBSOCKET_CRATE).expect_err("the invalid session is rejected");

        assert!(
            error
                .to_string()
                .contains("failed to generate the websockets")
        );
    }

    const MIDDLEWARE_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]
#[middleware(logged)]
struct Page;

impl Page {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[handles_middleware_attribute(attribute = logged)]
struct RequestLog;

impl RequestLog {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}

    #[process]
    fn process(&self, request: &Request, next: Next) -> anyhow::Result<ResponseContinuation> {}
}
";

    #[test]
    fn generates_the_middleware_module_for_attached_middleware() {
        let code = generate(MIDDLEWARE_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod middleware;"));
        assert!(module(&code, "middleware").contains("pub use request_log::RequestLog;"));
        assert!(!module(&code, "middleware").contains("pub mod request_log;"));
        assert!(module(&code, "middleware/request_log").contains("pub struct RequestLog"));
        assert!(concatenated(&code).contains("super::super::middleware::RequestLog"));
    }

    const AUTHENTICATED_USER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[infer_from_request]
    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/profile\", server = \"public\")]
struct GetProfile;

impl GetProfile {
    #[process]
    fn respond(&self, #[authenticated_user] user: User) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn generates_the_authenticated_users_module_for_a_declared_provider() {
        let code = generate(AUTHENTICATED_USER_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod authenticated_users;"));
        assert!(
            module(&code, "authenticated_users")
                .contains("pub use session_user_provider::SessionUserProvider;")
        );
        assert!(!module(&code, "authenticated_users").contains("pub mod session_user_provider;"));
        assert!(
            module(&code, "authenticated_users/session_user_provider")
                .contains("pub struct SessionUserProvider")
        );
        assert!(
            concatenated(&code).contains("super::super::authenticated_users::SessionUserProvider")
        );
    }

    #[test]
    fn omits_the_authenticated_users_module_without_a_provider() {
        let code = generate(WEB_CRATE).expect("the build succeeds");

        assert!(!module(&code, "mod").contains("pub mod authenticated_users;"));
        assert!(!has_module(&code, "authenticated_users"));
    }

    #[test]
    fn rejects_an_authenticated_user_provider_no_served_request_uses() {
        assert_eq!(
            generate(
                "#[rustfmt::skip]\npub mod margaret;\n\nuse margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;\n\nstruct User;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
            )
            .expect_err("no served request authenticates its user")
            .to_string(),
            "failed to generate the dependency container: the singleton 'crate::SessionUserProvider' is never used: no route, service, ticker, console command, websocket session or middleware reaches it"
        );
    }

    #[test]
    fn propagates_a_request_binding_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[infers_authenticated_user]\nstruct Bad;\n",
        )
        .expect_err("the invalid provider is rejected")
        .to_string();

        assert!(message.contains("failed to bind the request parameters"));
    }

    #[test]
    fn omits_the_middleware_module_without_attached_middleware() {
        let code = generate(WEB_CRATE).expect("the build succeeds");

        assert!(!module(&code, "mod").contains("pub mod middleware;"));
        assert!(!has_module(&code, "middleware"));
    }

    const WEBSOCKET_MIDDLEWARE_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[websocket_session(path = \"/room\", server = \"public\")]
#[middleware(logged)]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}

#[websocket_message(request, method = \"chat\", response = margaret::framework::websocket::web_socket_response::WebSocketResponse::Single)]
struct Chat;

#[singleton]
struct Chatter;

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}

#[singleton]
#[handles_middleware_attribute(attribute = logged)]
struct RequestLog;

impl RequestLog {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}

    #[process]
    fn process(&self, request: &Request, next: Next) -> anyhow::Result<ResponseContinuation> {}
}
";

    #[test]
    fn generates_the_middleware_module_for_a_websocket_only_server() {
        let code = generate(WEBSOCKET_MIDDLEWARE_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod websocket;"));
        assert!(module(&code, "mod").contains("pub mod middleware;"));
        assert!(module(&code, "middleware").contains("pub use request_log::RequestLog;"));
        assert!(!module(&code, "middleware").contains("pub mod request_log;"));
        assert!(module(&code, "middleware/request_log").contains("pub struct RequestLog"));
        assert!(concatenated(&code).contains("super::middleware::RequestLog"));
        assert!(!concatenated(&code).contains("GatedWebSocketUpgrade"));
    }

    #[test]
    fn propagates_a_middleware_codegen_error() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[handles_middleware_attribute(attribute = guard)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn process(&self, flag: bool) -> anyhow::Result<ResponseContinuation> {}\n}\n",
        )
        .expect_err("the invalid middleware handler is rejected")
        .to_string();

        assert!(message.contains("failed to generate the middleware"));
    }

    #[test]
    fn removes_the_server_when_responders_are_removed() {
        let web_crate = SourceCrate::new(WEB_CRATE);
        let plain_crate = SourceCrate::new(PLAIN_CRATE);
        let generated = web_crate.root().join(UMBRELLA_MODULE_NAME);

        generate_from_source(&web_crate.source_directory())
            .write_to(&generated)
            .expect("the first sources are written");

        assert!(generated.join("http.rs").exists());

        generate_from_source(&plain_crate.source_directory())
            .write_to(&generated)
            .expect("the second sources are written");

        let umbrella_source =
            fs::read_to_string(generated.join("mod.rs")).expect("the umbrella exists");

        assert!(!generated.join("http.rs").exists());
        assert!(!generated.join("routes.rs").exists());
        assert!(!generated.join("serve.rs").exists());
        assert!(!umbrella_source.contains("pub mod http;"));
        assert!(!umbrella_source.contains("pub mod routes;"));
        assert!(!umbrella_source.contains("pub mod serve;"));
    }

    #[test]
    fn is_idempotent() {
        let source_crate = SourceCrate::new(WEB_CRATE);
        let source = source_crate.source_directory();

        let first = generate_from_source(&source);
        let second = generate_from_source(&source);

        assert_eq!(module(&first, "mod"), module(&second, "mod"));
        assert_eq!(module(&first, "http"), module(&second, "http"));
        assert_eq!(module(&first, "routes"), module(&second, "routes"));
        assert_eq!(module(&first, "container"), module(&second, "container"));
    }

    #[test]
    fn propagates_an_index_failure() {
        let message = generate("use other::*;\n")
            .expect_err("the build fails")
            .to_string();

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn propagates_a_container_failure() {
        let message =
            generate("#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nenum Bad {}\n")
                .expect_err("the build fails")
                .to_string();

        assert!(message.contains("failed to generate the dependency container"));
    }

    const RECEIVER_CONSTRUCTOR_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(&self) -> anyhow::Result<Self> {}
}
";

    #[test]
    fn reports_a_constructor_with_a_receiver_parameter() {
        let message = generate(RECEIVER_CONSTRUCTOR_CRATE)
            .expect_err("a constructor with a receiver parameter is rejected")
            .to_string();

        assert!(message.contains("has an unsupported type"));
    }

    #[test]
    fn propagates_an_http_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[responds_to_http(path = \"/x\")]\nstruct Bad;\n\nimpl Bad {\n    #[constructor]\n    fn create() -> anyhow::Result<Self> {}\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("missing the 'method'"));
    }

    #[test]
    fn propagates_a_console_failure() {
        let message =
            generate("#[rustfmt::skip]\npub mod margaret;\n\n#[console_command]\nstruct Bad;\n")
                .expect_err("the build fails")
                .to_string();

        assert!(message.contains("failed to generate the console"));
    }

    #[test]
    fn propagates_a_services_failure() {
        let message = generate("#[rustfmt::skip]\npub mod margaret;\n\n#[service]\nstruct Bad;\n")
            .expect_err("the build fails")
            .to_string();

        assert!(message.contains("failed to generate the services"));
    }

    const POSITIONAL_OUTSIDE_COMMAND_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct Bad;

impl Bad {
    #[constructor]
    fn create(#[console_argument(positional)] label: String) -> anyhow::Result<Self> {}
}
";

    #[test]
    fn propagates_a_serve_input_failure() {
        let message = generate(POSITIONAL_OUTSIDE_COMMAND_CRATE)
            .expect_err("the positional argument outside a command is rejected")
            .to_string();

        assert!(message.contains("failed to read the serve inputs"));
    }

    const CONFLICTING_SERVE_INPUT_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[service]
#[console_command(name = \"worker\")]
struct Worker;

impl Worker {
    #[constructor]
    fn create(#[console_argument(positional)] label: String) -> anyhow::Result<Self> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]
struct Page;

impl Page {
    #[constructor]
    fn create(#[console_argument(from = \"label\")] label: String) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn propagates_a_conflicting_serve_input_failure() {
        let message = generate(CONFLICTING_SERVE_INPUT_CRATE)
            .expect_err("the conflicting serve input is rejected")
            .to_string();

        assert!(message.contains("a shared serve input must be declared identically everywhere"));
    }

    const VIEWS_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[renders_view(name = \"card\")]
struct Card;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/card\", server = \"public\")]
struct GetCard;

impl GetCard {
    #[process]
    fn respond(&self, views: &crate::margaret::views::Views) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn generates_the_views_module_when_views_and_http_exist() {
        let code = generate(VIEWS_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod views;"));
        assert!(module(&code, "views").contains("pub struct Views"));
        assert!(module(&code, "views").contains("card"));
        assert!(module(&code, "views/build").contains("pub fn build"));
        assert!(concatenated(&code).contains("super::views::build::build(container)"));
    }

    const WEBSOCKET_ONLY_VIEW_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[singleton]
#[renders_view(name = \"banner\")]
struct Banner;

#[websocket_session(path = \"/room\", server = \"public\")]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}

#[websocket_message(request, method = \"chat\", response = margaret::framework::websocket::web_socket_response::WebSocketResponse::Single)]
struct Chat;

#[singleton]
struct Chatter;

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
";

    #[test]
    fn rejects_a_view_of_a_crate_that_serves_no_responders() {
        assert_eq!(
            generate(WEBSOCKET_ONLY_VIEW_CRATE)
                .expect_err("no responder renders the view")
                .to_string(),
            "failed to generate the dependency container: the singleton 'crate::Banner' is never used: no route, service, ticker, console command, websocket session or middleware reaches it"
        );
    }

    #[test]
    fn propagates_a_views_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[renders_view(name = \"CardLayout\")]\nstruct Card;\n\n#[singleton]\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to generate the views"));
    }

    #[test]
    fn propagates_a_dependency_cycle_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\nuse std::sync::Arc;\n\n#[singleton]\nstruct A;\n\nimpl A {\n    #[constructor]\n    fn create(b: Arc<B>) -> anyhow::Result<Self> {}\n}\n\n#[singleton]\nstruct B;\n\nimpl B {\n    #[constructor]\n    fn create(a: Arc<A>) -> anyhow::Result<Self> {}\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("dependency cycle"));
    }

    #[test]
    fn awaits_an_async_constructor_in_the_container() {
        let code = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nstruct Pool;\n\nimpl Pool {\n    #[constructor]\n    async fn create() -> anyhow::Result<Self> {}\n}\n\n#[singleton]\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]\nstruct Page {\n    pool: std::sync::Arc<Pool>,\n}\n\nimpl Page {\n    #[constructor]\n    fn create(pool: std::sync::Arc<Pool>) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        )
        .expect("the async build succeeds");

        let serve: String = module(&code, "serve").split_whitespace().collect();

        assert!(module(&code, "container/build/serve").contains("Pool::create().await"));
        assert!(concatenated(&code).contains("fn server_public"));
        assert!(!concatenated(&code).contains("async fn server_public"));
        assert!(serve.contains("super::http::server_public::server_public(container,"));
    }

    const MULTI_SERVER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/\", server = \"public\")]
struct Index;

impl Index {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/metrics\", server = \"internal\")]
struct Metrics;

impl Metrics {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn generates_a_server_function_and_address_argument_per_active_server() {
        let code = generate(MULTI_SERVER_CRATE).expect("the build succeeds");

        let http = concatenated(&code);
        assert!(http.contains("fn server_public"));
        assert!(http.contains("fn server_internal"));
        assert!(!http.contains("async fn server_public"));
        assert!(!http.contains("async fn server_internal"));

        let serve: String = module(&code, "serve").split_whitespace().collect();
        assert!(serve.contains("super::http::server_public::server_public(container,"));
        assert!(serve.contains("super::http::server_internal::server_internal(container,"));
        assert!(serve.contains(r#"address_argument:"public-addr""#));
        assert!(serve.contains(r#"address_argument:"internal-addr""#));

        let run = module(&code, "run");
        assert!(run.contains(r#"clap::Arg::new("public-addr")"#));
        assert!(run.contains(r#"clap::Arg::new("internal-addr")"#));
    }

    #[test]
    fn rejects_a_non_string_server() {
        let error = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/\", server = crate::Ghost)]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        )
        .expect_err("the build fails");

        assert!(matches!(
            error,
            CodegenError::Http {
                source: HttpCodegenError::AttributeArguments {
                    source: AttributeArgumentsError::UnexpectedArgument { ref key, ref expected, .. }
                }
            } if key == "server" && expected == "string literal"
        ));
    }

    #[test]
    fn supports_same_struct_name_route_handlers_in_different_modules() {
        let code = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\nmod routes {\n#[singleton]\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/a\", server = \"public\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n}\n\nmod endpoints {\n#[singleton]\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/b\", server = \"internal\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n}\n",
        )
        .expect("two `Page` handlers in different modules coexist");

        let http = concatenated(&code);

        assert!(!http.contains("enum RouteName"));
        assert!(http.contains("\"/a\""));
        assert!(http.contains("\"/b\""));
        assert!(http.contains("crate::routes::Page"));
        assert!(http.contains("crate::endpoints::Page"));
    }

    const FRAMEWORK_NAMED_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct Build;

#[singleton]
struct Container;

#[singleton]
struct Routes;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, name = \"origin\", path = \"/o\", server = \"routes\")]
struct Origin {
    build: std::sync::Arc<Build>,
    container: std::sync::Arc<Container>,
    routes: std::sync::Arc<Routes>,
}

impl Origin {
    #[constructor]
    fn create(build: std::sync::Arc<Build>, container: std::sync::Arc<Container>, routes: std::sync::Arc<Routes>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, name = \"new\", path = \"/n/{id}\", server = \"routes\")]
struct New;

impl New {
    #[process]
    fn respond(&self, #[route_parameter(from = \"id\")] id: String) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn generates_without_reserving_names_for_components_named_after_framework_identifiers() {
        let code = generate(FRAMEWORK_NAMED_CRATE).expect("the build succeeds");

        let build: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();
        let routes: String = module(&code, "routes").split_whitespace().collect();
        let http: String = concatenated(&code).split_whitespace().collect();

        assert!(build.contains("pubfnserve("));
        assert!(build.contains("letbuild=::std::sync::Arc::new(crate::Build);"));
        assert!(build.contains("letcontainer=::std::sync::Arc::new(crate::Container);"));
        assert!(build.contains("letroutes=::std::sync::Arc::new(crate::Routes);"));
        assert!(!build.contains("build_2"));
        assert!(!build.contains("container_2"));
        assert!(!build.contains("routes_2"));

        assert!(routes.contains("pubstructRoutes{pubroutes:servers::routes::Routes,}"));
        assert!(module(&code, "routes/servers/routes").contains("pub struct Routes"));

        assert!(http.contains("container:&super::super::container::Container"));
        assert!(!http.contains("usesuper::container::Container"));
    }

    #[test]
    fn generates_the_schema_module_and_command_when_models_exist() {
        let code = generate(MODELS_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod schema;"));
        assert!(module(&code, "mod").contains("pub mod run;"));
        assert!(!has_module(&code, "http"));
        assert!(module(&code, "schema").contains("pub fn schema"));
        assert!(module(&code, "schema").contains("\"widgets\""));
        assert!(module(&code, "schema").contains("ColumnType::Uuid"));

        let command: String = module(&code, "run").split_whitespace().collect();
        let run: String = module(&code, "run").split_whitespace().collect();

        assert!(run.contains("pubfnrun"));
        assert!(!run.contains("pubasyncfnrun"));
        assert!(command.contains("\"schema\""));
        assert!(run.contains("super::schema::schema()"));
    }

    #[test]
    fn generates_only_the_framework_tables_of_the_enabled_stores() {
        let code = generate(JWKS_ROLLER_CRATE).expect("the build succeeds");
        let run: String = module(&code, "run").split_whitespace().collect();

        assert!(module(&code, "schema").contains("\"signing_key_sets\""));
        assert!(!module(&code, "schema").contains("\"pending_authorizations\""));
        assert!(!module(&code, "schema").contains("\"client_assertions\""));
        assert!(run.contains("super::schema::schema()"));
    }

    #[test]
    fn injects_the_declared_database_into_an_application_singleton() {
        let code = generate(DATABASE_CRATE).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(construction.contains(
            "crate::ListArticles::create(::std::sync::Arc::clone(&framework_database_database_database),)"
        ));
        assert!(!has_module(&code, "schema"));
    }

    #[test]
    fn rejects_a_postgres_database_nothing_uses() {
        assert!(matches!(
            generate(
                "#[rustfmt::skip]\npub mod margaret;\n\n#[postgres_database(url_from = \"APPLICATION_DATABASE_URL\")]\nstruct ApplicationDatabase;\n",
            ),
            Err(CodegenError::Container {
                source: ContainerError::UnconsumedDeclaration { path },
            }) if path == "margaret::framework::database::database::Database"
        ));
    }

    #[test]
    fn propagates_a_database_failure() {
        assert!(matches!(
            generate(
                "#[rustfmt::skip]\npub mod margaret;\n\n#[postgres_database]\nstruct ApplicationDatabase;\n",
            ),
            Err(CodegenError::Database {
                source: DatabaseCodegenError::MissingUrlSource { anchor },
            }) if anchor == "crate::ApplicationDatabase"
        ));
    }

    #[test]
    fn propagates_a_token_issuance_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[issues_tokens(provider, issuer = \"https://issuer.fixture\")]\nstruct Issuer;\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to read the token issuance"));
    }

    #[test]
    fn propagates_a_trusted_issuer_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[verifies_tokens_from_issuer(partner, issuer = \"https://partner.fixture\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct Partner;\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to read the trusted issuers"));
    }

    #[test]
    fn propagates_an_oauth_client_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[acts_as_oauth_client(partner_client, client_id = \"partner\", issuer = partner)]\nstruct PartnerClient;\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to read the oauth clients"));
    }

    #[test]
    fn propagates_a_models_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[model(table = \"widgets\")]\nstruct Widget {\n    #[column]\n    id: u64,\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to generate the models"));
    }

    const CONSOLE_ARGUMENT_RESPONDER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]
struct Page;

impl Page {
    #[constructor]
    fn create(#[console_argument(from = \"greeting\")] greeting: String) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn resolves_a_responder_serve_input_during_container_construction() {
        let code = generate(CONSOLE_ARGUMENT_RESPONDER_CRATE).expect("the build succeeds");
        let serve: String = module(&code, "serve").split_whitespace().collect();

        assert!(serve.contains("letserve_input_0="));
        assert!(serve.contains(
            "super::container::build::serve(super::container::build::serve_arguments::ServeArguments{argument0:serve_input_0"
        ));
        assert!(serve.contains("server_public(container,routes"));
        assert!(!serve.contains("server_public(container,&serve_input_0"));

        let run = module(&code, "run");
        assert!(run.contains(r#"clap::Arg::new("greeting")"#));
    }

    const CONSOLE_ARGUMENT_VIEW_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[renders_view(name = \"banner\")]
struct Banner;

impl Banner {
    #[constructor]
    fn create(#[console_argument(from = \"title\")] title: String) -> anyhow::Result<Self> {}
}

#[singleton]
#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/x\", server = \"public\")]
struct Page;

impl Page {
    #[process]
    fn respond(&self, views: &crate::margaret::views::Views) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn resolves_a_view_serve_input_during_container_construction() {
        let code = generate(CONSOLE_ARGUMENT_VIEW_CRATE).expect("the build succeeds");
        let serve: String = module(&code, "serve").split_whitespace().collect();

        assert!(serve.contains("letserve_input_0="));
        assert!(serve.contains(
            "super::container::build::serve(super::container::build::serve_arguments::ServeArguments{argument0:serve_input_0"
        ));
        assert!(serve.contains("super::views::build::build(container)"));
    }

    const CONSOLE_ARGUMENT_WEBSOCKET_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[websocket_session(path = \"/room\", server = \"public\")]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}

#[websocket_message(request, method = \"chat\", response = margaret::framework::websocket::web_socket_response::WebSocketResponse::Single)]
struct Chat;

#[singleton]
struct Chatter;

impl Chatter {
    #[constructor]
    fn create(#[console_argument(from = \"greeting\")] greeting: String) -> anyhow::Result<Self> {}
}

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
";

    #[test]
    fn resolves_a_websocket_handler_serve_input_during_container_construction() {
        let code = generate(CONSOLE_ARGUMENT_WEBSOCKET_CRATE).expect("the build succeeds");
        let serve: String = module(&code, "serve").split_whitespace().collect();

        assert!(serve.contains("letserve_input_0="));
        assert!(serve.contains(
            "super::container::build::serve(super::container::build::serve_arguments::ServeArguments{argument0:serve_input_0"
        ));
        assert!(serve.contains("server_public(container,routes"));
        assert!(!serve.contains("server_public(container,&serve_input_0"));
    }

    const CONSOLE_ARGUMENT_SESSION_DEPENDENCY_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use std::sync::Arc;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[singleton]
struct SystemClock;

impl SystemClock {
    #[constructor]
    fn create(#[console_argument(from = \"timezone\")] timezone: String) -> anyhow::Result<Self> {}
}

#[websocket_session(path = \"/room\", server = \"public\")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(clock: Arc<SystemClock>) -> anyhow::Result<Self> {}
}

#[websocket_message(request, method = \"chat\", response = margaret::framework::websocket::web_socket_response::WebSocketResponse::Single)]
struct Chat;

#[singleton]
struct Chatter;

impl Chatter {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}
}

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
";

    #[test]
    fn resolves_a_serve_input_for_a_session_injected_dependency() {
        let code = generate(CONSOLE_ARGUMENT_SESSION_DEPENDENCY_CRATE).expect("the build succeeds");
        let serve: String = module(&code, "serve").split_whitespace().collect();

        assert!(serve.contains("letserve_input_0="));
        assert!(serve.contains(
            "super::container::build::serve(super::container::build::serve_arguments::ServeArguments{argument0:serve_input_0"
        ));
        assert!(serve.contains("server_public(container,routes"));
        assert!(!serve.contains("server_public(container,&serve_input_0"));
    }
}
