use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use margaret_accepted_clients_codegen::accepted_client_item::AcceptedClientItem;
use margaret_accepted_clients_codegen::accepted_client_item_path::accepted_client_item_path;
use margaret_accepted_clients_codegen::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
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
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_artifacts::HttpArtifacts;
use margaret_http_codegen::http_plan::HttpPlan;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::render_http::render_http;
use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
use margaret_middleware_codegen::render_middleware_wrappers::render_middleware_wrappers;
use margaret_model_codegen::models::models;
use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
use margaret_oidc_provider_codegen::oidc_provider_item::OidcProviderItem;
use margaret_oidc_provider_codegen::oidc_provider_item_path::oidc_provider_item_path;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::render_authenticated_user_wrappers::render_authenticated_user_wrappers;
use margaret_request_binding_codegen::views_availability::ViewsAvailability;
use margaret_schema_codegen::framework_tables::FrameworkTables;
use margaret_schema_codegen::render_schema::render_schema;
use margaret_serve_input_codegen::scan::scan;
use margaret_service_codegen::framework_service::FrameworkService;
use margaret_service_codegen::render_services::render_services;
use margaret_service_codegen::served_origin_check::ServedOriginCheck;
use margaret_service_codegen::service_plan::ServicePlan;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;
use margaret_tag_codegen::subject_token_exchanger_binding::SubjectTokenExchangerBinding;
use margaret_tag_codegen::tag_pool::TagPool;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
use margaret_token_issuance_codegen::render_token_issuance::render_token_issuance;
use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;
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
use crate::format_pass::format_pass;
use crate::generated_code::GeneratedCode;
use crate::generated_feature::GeneratedFeature;
use crate::generated_features::GeneratedFeatures;
use crate::issuer_directory_framework_providers::issuer_directory_framework_providers;
use crate::issuer_directory_services::issuer_directory_services;
use crate::jwks_framework_providers::jwks_framework_providers;
use crate::jwks_secret_storage_provider::jwks_secret_storage_provider;
use crate::oauth_client_framework_providers::oauth_client_framework_providers;
use crate::oidc_provider_framework_providers::oidc_provider_framework_providers;
use crate::provider_endpoint_artifacts::ProviderEndpointArtifacts;
use crate::provider_endpoint_modules::provider_endpoint_modules;
use crate::provider_state_canonical_path::provider_state_canonical_path;
use crate::serve_inputs::serve_inputs;
use crate::trusted_issuer_framework_providers::trusted_issuer_framework_providers;
use crate::umbrella::umbrella;

/// # Errors
///
/// Returns `CodegenError` propagated from the work it performs.
fn framework_providers(
    trusts: &DeclaredTrusts,
    oauth_client_bindings: &[OAuthClientBinding],
    accepted_clients: &DeclaredAcceptedClients,
    exchanger_bindings: &[SubjectTokenExchangerBinding],
) -> Vec<FrameworkProvider> {
    let mut framework_providers = vec![
        FrameworkProvider {
            construction: FrameworkConstruction::Unit,
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::Unmarked,
            provided: asset_responder_canonical_path(),
        },
        jwks_secret_storage_provider(),
    ];
    framework_providers.extend(jwks_framework_providers());
    framework_providers.extend(trusted_issuer_framework_providers(trusts));
    framework_providers.extend(
        accepted_clients
            .clients
            .iter()
            .flat_map(accepted_client_framework_providers),
    );
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
                            DeclaredAcceptedAuthentication::PrivateKeyJwt(_)
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
    framework_providers.extend(oauth_client_framework_providers(oauth_client_bindings));
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

    framework_providers
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
    let trusted_issuers = build_trusted_issuer_artifacts(declarations.trusts);
    let oauth_clients = build_oauth_client_artifacts(declarations.oauth_clients);
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
    features.enable_if(
        GeneratedFeature::Schema,
        bindings.provides(&provider_state_canonical_path()),
    );

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
    bindings: &ContainerBindings,
    features: &GeneratedFeatures,
    middleware_plans: &MiddlewarePlans,
    registries: &BindingRegistries,
    token_issuance: &DeclaredTokenIssuance,
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
        render_views(ViewsPlan::build(index, bindings)?, bindings)
    } else {
        ViewsArtifacts {
            serve_inputs: Vec::new(),
            modules: Vec::new(),
            retained_roots: Vec::new(),
        }
    };
    let http = if features.contains(GeneratedFeature::Http)
        || features.contains(GeneratedFeature::Websockets)
    {
        let plan = HttpPlan::build(
            index,
            features.contains(GeneratedFeature::Views),
            &websockets.servers,
            middleware_plans,
            bindings,
            registries,
        )?;
        let provider_endpoints =
            provider_endpoint_modules(&plan.route_locations(), bindings, token_issuance)?;

        HttpModules {
            artifacts: render_http(plan, bindings),
            provider_endpoints,
        }
    } else {
        HttpModules {
            artifacts: HttpArtifacts {
                retained_roots: Vec::new(),
                modules: Vec::new(),
                server_serve_inputs: BTreeMap::new(),
                servers: Vec::new(),
            },
            provider_endpoints: provider_endpoint_modules(&[], bindings, token_issuance)?,
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
        served_origin_checks: http.provider_endpoints.checks,
        servers: http.artifacts.servers,
    })
}

fn render_role_modules(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
    features: &GeneratedFeatures,
    framework_services: &[FrameworkService],
    served_origin_checks: &[ServedOriginCheck],
    servers: &[HttpServer],
) -> Result<RoleModules, CodegenError> {
    let mut modules: Vec<GeneratedModuleTokens> = Vec::new();

    let serve_inputs = serve_inputs(bindings);
    let service_plan = ServicePlan::build(index, framework_services, bindings, &serve_inputs)?;
    let service_roots = service_plan.roots().to_vec();

    if features.contains(GeneratedFeature::Serves) {
        modules.push(render_services(
            &service_plan,
            servers,
            features.contains(GeneratedFeature::Views),
            bindings,
            served_origin_checks,
        ));
    }

    if features.contains(GeneratedFeature::Schema) {
        let models = models(index)?;
        modules.push(render_schema(
            &models,
            if bindings.provides(&provider_state_canonical_path()) {
                FrameworkTables::OidcProviderState
            } else {
                FrameworkTables::Unused
            },
        ));
    }

    let console_roots = if features.contains(GeneratedFeature::Console) {
        let plan = ConsolePlan::build(index, bindings)?;
        let console = render_console(
            &plan,
            features.contains(GeneratedFeature::Serves),
            features.contains(GeneratedFeature::Schema),
            servers,
            &serve_inputs,
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

struct FrameworkDeclarations<'declarations, 'index> {
    accepted_clients: &'declarations DeclaredAcceptedClients<'index>,
    oauth_clients: &'declarations DeclaredOAuthClients<'index>,
    trusts: &'declarations DeclaredTrusts<'index>,
}

struct FrameworkModules {
    modules: Vec<GeneratedModuleTokens>,
    services: Vec<FrameworkService>,
}

struct WrapperModules<'tags> {
    middleware_plans: MiddlewarePlans<'tags>,
    modules: Vec<GeneratedModuleTokens>,
    registries: BindingRegistries,
}

struct ServingModules {
    modules: Vec<GeneratedModuleTokens>,
    retained_roots: Vec<CanonicalPath>,
    served_origin_checks: Vec<ServedOriginCheck>,
    servers: Vec<HttpServer>,
}

struct HttpModules {
    artifacts: HttpArtifacts,
    provider_endpoints: ProviderEndpointArtifacts,
}

struct RoleModules {
    console_roots: Vec<CanonicalPath>,
    modules: Vec<GeneratedModuleTokens>,
    service_roots: Vec<CanonicalPath>,
}

/// # Errors
///
/// Returns `CodegenError` propagated from the work it performs.
pub fn build(
    crate_root: &CrateRoot,
    metafile_contents: Option<&str>,
    assets_directory: &Path,
    embed_relative: &str,
) -> Result<GeneratedCode, CodegenError> {
    let index = AttributeIndexBuilder::new()
        .exclude_root_module(UMBRELLA_MODULE_NAME)
        .index_crate(crate_root)?
        .build();
    let mut features = GeneratedFeatures::from_index(&index);
    let registry = scan(&index)?;
    let token_issuance = DeclaredTokenIssuance::read(&index)?;
    let trusts = DeclaredTrusts::read(&index)?;
    let oauth_clients = DeclaredOAuthClients::read(&index)?;
    let accepted_clients = DeclaredAcceptedClients::read(&index, &token_issuance)?;
    let tags = TagPool::collect(&index, &trusts, &oauth_clients)?;
    let oauth_client_bindings = tags.oauth_client_bindings(&trusts, &oauth_clients)?;
    let exchanger_bindings = tags.subject_token_exchanger_bindings(&index, &trusts)?;
    let framework_providers = framework_providers(
        &trusts,
        &oauth_client_bindings,
        &accepted_clients,
        &exchanger_bindings,
    );
    let planned_container =
        plan_container(&index, &registry, &framework_providers, &token_issuance)?;
    let bindings = planned_container.bindings();
    let application_roots = planned_container.roots();
    let mut module_tokens = asset_bag_modules(
        &index,
        metafile_contents,
        bindings,
        assets_directory,
        embed_relative,
    )?;
    features.enable_if(GeneratedFeature::AssetBag, !module_tokens.is_empty());

    if let DeclaredTokenIssuance::Declared(declaration) = &token_issuance {
        module_tokens.push(render_token_issuance(declaration));
        features.enable(GeneratedFeature::TokenIssuance);
    }
    let FrameworkModules {
        modules: framework_modules,
        services: framework_services,
    } = framework_modules(
        bindings,
        &FrameworkDeclarations {
            accepted_clients: &accepted_clients,
            oauth_clients: &oauth_clients,
            trusts: &trusts,
        },
        &exchanger_bindings,
        &mut features,
    );

    module_tokens.extend(framework_modules);

    let WrapperModules {
        middleware_plans,
        modules: wrapper_modules,
        registries,
    } = render_wrapper_modules(&index, bindings, &features, &tags)?;

    module_tokens.extend(wrapper_modules);

    let ServingModules {
        modules: serving_modules,
        retained_roots: serving_roots,
        served_origin_checks,
        servers,
    } = render_serving_modules(
        &index,
        bindings,
        &features,
        &middleware_plans,
        &registries,
        &token_issuance,
    )?;

    module_tokens.extend(serving_modules);

    let RoleModules {
        console_roots,
        modules: role_modules,
        service_roots,
    } = render_role_modules(
        &index,
        bindings,
        &features,
        &framework_services,
        &served_origin_checks,
        &servers,
    )?;

    module_tokens.extend(role_modules);

    let retained_roots = service_roots
        .into_iter()
        .chain(serving_roots)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    planned_container
        .render(&application_roots, &retained_roots, &console_roots)
        .map_err(CodegenError::from)
        .and_then(|rendered_container| {
            module_tokens.extend(rendered_container.modules);

            format_pass(module_tokens)
                .map_err(CodegenError::from)
                .map(|mut modules| {
                    modules.push(umbrella(&features));

                    GeneratedCode::new(modules)
                })
        })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use margaret_accepted_clients_codegen::accepted_clients_codegen_error::AcceptedClientsCodegenError;
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_attributes_tests::source_crate::SourceCrate;
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
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]
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
struct Config;

impl Config {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}
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

#[websocket_message(request, method = \"chat\", response = single)]
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

    const EMBED_RELATIVE: &str = ".";

    fn generate(lib_source: &str) -> Result<GeneratedCode, CodegenError> {
        let source_crate = SourceCrate::new(lib_source);

        build(
            &CrateRoot::new("crate", source_crate.source_directory()),
            None,
            &source_crate.root().join("assets"),
            EMBED_RELATIVE,
        )
    }

    fn generate_from_source(source: &Path) -> GeneratedCode {
        build(
            &CrateRoot::new("crate", source),
            None,
            &source.join("assets"),
            EMBED_RELATIVE,
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
            EMBED_RELATIVE,
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
struct Config;
";

    #[test]
    fn generates_the_asset_macro_a_module_imports() {
        let source_crate = SourceCrate::new(ASSET_MACRO_CRATE);

        let code = build(
            &CrateRoot::new("crate", source_crate.source_directory()),
            Some(r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#),
            &source_crate.root().join("assets"),
            EMBED_RELATIVE,
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
struct AssetRoute {
    responder: std::sync::Arc<crate::margaret::asset_bag::asset_responder::AssetResponder>,
}

impl AssetRoute {
    #[constructor]
    fn create(
        responder: std::sync::Arc<crate::margaret::asset_bag::asset_responder::AssetResponder>,
    ) -> anyhow::Result<Self> {}
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
            EMBED_RELATIVE,
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
            EMBED_RELATIVE,
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
            EMBED_RELATIVE,
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

        assert!(message.contains("no esbuild metafile was found at the workspace root"));
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
            "a module imports the asset macro, but no esbuild metafile was found at the workspace root"
        );
    }

    #[test]
    fn propagates_an_asset_bag_failure() {
        let source_crate = SourceCrate::new(ASSET_MACRO_CRATE);

        let message = build(
            &CrateRoot::new("crate", source_crate.source_directory()),
            Some(r#"{ "outputs": {} }"#),
            &source_crate.root().join("assets"),
            EMBED_RELATIVE,
        )
        .expect_err("an invalid metafile is rejected")
        .to_string();

        assert!(message.contains("failed to generate the asset bag"));
    }

    const JWKS_ROLLER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = \"get\", path = \"/.well-known/jwks.json\", server = \"internal\")]
struct GetJwks {
    handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>,
}

impl GetJwks {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    const JWKS_MULTI_CLIENT_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;


#[provides_jwks_endpoint(auth, audience = \"api\", issuer = \"https://auth.example\", jwks_uri = \"https://auth.example/jwks\")]
struct AuthJwksEndpoint;



#[provides_jwks_endpoint(partner, audience = \"api\", issuer = \"https://partner.example\", jwks_uri = \"https://partner.example/jwks\")]
struct PartnerJwksEndpoint;



#[singleton]
#[responds_to_http(method = \"get\", path = \"/verify\", server = \"public\")]
struct GetVerify;

impl GetVerify {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    const JWKS_SERVER_STORE_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;


#[issues_tokens(audience = \"session\", issuer = \"https://issuer.fixture\")]
struct Issuer;

#[singleton]
#[responds_to_http(method = \"post\", path = \"/mint\", server = \"internal\")]
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
            "iflet::std::result::Result::Err(error)=margaret::framework::spiffe_svid::install_default_crypto_provider::install_default_crypto_provider(){returnmargaret::framework::console::report_failure::report_failure(error);}"
        ));
        assert!(serve.contains(
            "letspiffe_bundle=margaret::framework::spiffe_svid_client::svid_client_bundle::SvidClientBundle::new(margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams{"
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
#[responds_to_http(method = \"get\", path = \"/call\", server = \"public\")]
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
            "letspiffe_bundle=margaret::framework::spiffe_svid_client::svid_client_bundle::SvidClientBundle::new(margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams{"
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
#[responds_to_http(method = \"get\", path = \"/identity\", server = \"internal\")]
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
            "letspiffe_bundle=margaret::framework::spiffe_svid_bundle::svid_bundle::SvidBundle::new(margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams{"
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
            "margaret::framework::jwks_secret_storage_selection::resolve_jwks_secret_storage::resolve_jwks_secret_storage"
        ));

        let serve: String = module(&code, "serve").split_whitespace().collect();
        assert!(serve.contains("impltrzcina::TickerforMargaretJwksJwksRoller"));
        assert!(serve.contains(
            "margaret::framework::jwks_roller_server::jwks_roll_interval::JWKS_ROLL_INTERVAL"
        ));
    }

    #[test]
    fn generates_a_trusted_issuer_per_provides_jwks_endpoint() {
        let code = generate(JWKS_MULTI_CLIENT_CRATE).expect("the build succeeds");
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
            "crate::margaret::trusted_issuers::auth::TrustedIssuer::create(::std::sync::Arc::<crate::margaret::trusted_issuers::auth::IssuerKeySet,>::clone(&margaret_trusted_issuers_auth_issuer_key_set),crate::margaret::trusted_issuers::auth::token_trust::TOKEN_TRUST,)"
        ));
        assert!(!concatenated(&code).contains("PublicJwksHandler"));
    }

    #[test]
    fn polls_every_trusted_issuer_through_one_issuer_directory() {
        let code = generate(JWKS_MULTI_CLIENT_CRATE).expect("the build succeeds");
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

    #[trusts_oidc_issuer(first, audience = \"api\", issuer = \"https://first.example\")]
    pub struct Issuer;

}

mod second {

    #[trusts_oidc_issuer(second, audience = \"api\", issuer = \"https://second.example\")]
    pub struct Issuer;

}

#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]
struct Page;

impl Page {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn generates_a_trusted_issuer_per_trusted_oidc_issuer() {
        let code = generate(OIDC_ISSUERS_CRATE).expect("the build succeeds");
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
            "crate::margaret::trusted_issuers::second::TrustedIssuer::create(::std::sync::Arc::<crate::margaret::trusted_issuers::second::IssuerKeySet,>::clone(&margaret_trusted_issuers_second_issuer_key_set),crate::margaret::trusted_issuers::second::token_trust::TOKEN_TRUST,)"
        ));
    }

    const OAUTH_CLIENT_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

#[trusts_oidc_issuer(partner, audience = \"api\", issuer = \"https://partner.example\")]
struct PartnerIssuer;


#[oauth_client(partner_client, authentication = client_secret_basic(client_secret_from = \"PARTNER_CLIENT_SECRET\"), client_id = \"partner\", issuer = partner)]
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
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]
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


#[issues_tokens(audience = \"session\", issuer = \"https://issuer.fixture\")]
struct Issuer;

#[trusts_oidc_issuer(partner, audience = \"api\", issuer = \"https://partner.example\")]
struct PartnerIssuer;


#[oauth_client(partner_client, authentication = private_key_jwt, client_id = \"partner\", issuer = partner)]
struct PartnerClient;

#[singleton]
#[responds_to_http(method = \"get\", path = \"/sign-in\", server = \"public\")]
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
";

    const OIDC_PROVIDER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;

#[issues_tokens(audience = \"session\", issuer = \"https://issuer.fixture\")]
struct Issuer;

#[trusts_oidc_issuer(ci, audience = \"api\", issuer = \"https://ci.example\")]
struct CiIssuer;


#[singleton]
#[exchanges_subject_tokens(issuer = ci)]
struct CiExchanger;

impl ExchangesSubjectTokens for CiExchanger {}

#[accepts_oauth_client(
    authentication = private_key_jwt(introspection, jwks_uri = \"https://portal.fixture/jwks.json\"),
    authorization_code(consent = prompted, id_token_signing = rsa, redirect_uris = [\"https://portal.fixture/callback\"], scopes = [\"openid\"]),
    client_id = \"portal\",
    resources = [\"artifacts\"],
)]
struct PortalClient;

#[accepts_oauth_client(authentication = none, client_id = \"spa\", resources = [\"reports\"])]
struct SpaClient;

#[singleton]
#[responds_to_http(method = \"get\", path = \"/authorize\", server = \"public\")]
struct GetAuthorize;

impl GetAuthorize {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::AuthorizationEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = \"post\", path = \"/authorize\", server = \"public\")]
struct PostAuthorize;

impl PostAuthorize {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::AuthorizationEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = \"get\", path = \"/.well-known/openid-configuration\", server = \"public\")]
struct GetDiscovery;

impl GetDiscovery {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::ProviderMetadataHandler>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = \"post\", path = \"/introspect\", server = \"public\")]
struct PostIntrospect;

impl PostIntrospect {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::IntrospectionEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = \"post\", path = \"/revoke\", server = \"public\")]
struct PostRevoke;

impl PostRevoke {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::RevocationEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = \"post\", path = \"/token\", server = \"public\")]
struct PostToken;

impl PostToken {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::TokenEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = \"get\", path = \"/userinfo\", server = \"public\")]
struct GetUserinfo;

impl GetUserinfo {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::UserinfoEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = \"get\", path = \"/jwks.json\", server = \"public\")]
struct GetJwks;

impl GetJwks {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = \"post\", path = \"/consent\", server = \"public\")]
struct PostConsent;

impl PostConsent {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::oidc_provider::ConsentEndpoint>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

";

    #[test]
    fn generates_the_endpoints_of_an_oidc_provider() {
        let code = generate(OIDC_PROVIDER_CRATE).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(module(&code, "mod").contains("pub mod oidc_provider;"));
        assert!(module(&code, "mod").contains("pub mod token_issuance;"));
        assert!(module(&code, "token_issuance").contains("issuer: \"https://issuer.fixture\","));
        assert!(
            module(&code, "oidc_provider/provider_endpoints")
                .contains("authorization: \"https://issuer.fixture/authorize\",")
        );
        assert!(module(&code, "oidc_provider/subject_token_exchangers").contains("pub mod ci;"));
        assert!(construction.contains(
            "crate::margaret::oidc_provider::AcceptedClients::create(::std::vec::Vec::from([::std::sync::Arc::<crate::margaret::accepted_clients::clients::portal_client::RegisteredClient,>::clone(&margaret_accepted_clients_clients_portal_client_registered_client,),::std::sync::Arc::<crate::margaret::accepted_clients::clients::spa_client::RegisteredClient,>::clone(&margaret_accepted_clients_clients_spa_client_registered_client),]),crate::margaret::token_issuance::TOKEN_ISSUANCE,)"
        ));
        assert!(construction.contains(
            "crate::margaret::oidc_provider::IntrospectionEndpoint::create(::std::sync::Arc::<crate::margaret::oidc_provider::AcceptedClients,>::clone(&margaret_oidc_provider_accepted_clients),::std::sync::Arc::<crate::margaret::jwks::JwksSecretStore,>::clone(&margaret_jwks_jwks_secret_store),::std::sync::Arc::<dynmargaret::framework::provider_state_storage::stores_provider_state::StoresProviderState,>::clone(&framework_provider_state_storage_stores_provider_state_stores_provider_state,),)"
        ));
        assert!(construction.contains(
            "crate::margaret::oidc_provider::subject_token_exchangers::ci::SubjectTokenExchanger::create(::std::sync::Arc::<crate::margaret::trusted_issuers::ci::TrustedIssuer,>::clone(&margaret_trusted_issuers_ci_trusted_issuer),::std::sync::Arc::<crate::CiExchanger>::clone(&ci_exchanger),)"
        ));
        assert!(construction.contains(
            "crate::margaret::oidc_provider::ConsentEndpoint::create(::std::sync::Arc::<dynmargaret::framework::provider_state_storage::stores_provider_state::StoresProviderState,>::clone(&framework_provider_state_storage_stores_provider_state_stores_provider_state,),crate::margaret::token_issuance::TOKEN_ISSUANCE,)"
        ));
        assert!(
            module(&code, "serve")
                .split_whitespace()
                .collect::<String>()
                .contains("read_required::<margaret::framework::provider_state_storage_selection::provider_state_storage_uri::ProviderStateStorageUri,>(\"MARGARET_OIDC_PROVIDER_STATE_STORAGE\")")
        );
        assert!(
            module(&code, "serve")
                .split_whitespace()
                .collect::<String>()
                .contains("read_required::<margaret::framework::jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri,>(\"MARGARET_JWKS_SECRET_STORAGE\")")
        );
        assert!(!module(&code, "run").contains("storage"));
        assert!(module(&code, "schema").contains(
            "margaret::framework::provider_state_postgres::provider_state_tables::provider_state_tables()"
        ));
        assert!(
            module(&code, "serve")
                .split_whitespace()
                .collect::<String>()
                .contains(
                    "crate::margaret::oidc_provider::provider_endpoints::PROVIDER_ENDPOINTS.served_by(origin_public)"
                )
        );
    }

    #[test]
    fn registers_each_accepted_client_by_how_it_authenticates() {
        let code = generate(OIDC_PROVIDER_CRATE).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(module(&code, "mod").contains("pub mod accepted_clients;"));
        assert!(construction.contains(
            "crate::margaret::accepted_clients::clients::portal_client::RegisteredClient::private_key_jwt(crate::margaret::accepted_clients::clients::portal_client::accepted_client::ACCEPTED_CLIENT,crate::margaret::accepted_clients::clients::portal_client::confidential_privileges::CONFIDENTIAL_PRIVILEGES,::std::sync::Arc::<crate::margaret::accepted_clients::clients::portal_client::IssuerKeySet,>::clone(&margaret_accepted_clients_clients_portal_client_issuer_key_set),)"
        ));
        assert!(construction.contains(
            "crate::margaret::accepted_clients::clients::spa_client::RegisteredClient::public(crate::margaret::accepted_clients::clients::spa_client::accepted_client::ACCEPTED_CLIENT,)"
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
        let error = generate(&OIDC_PROVIDER_CRATE.replace(
            "#[issues_tokens(audience = \"session\", issuer = \"https://issuer.fixture\")]\nstruct Issuer;\n",
            "",
        ))
        .expect_err("an accepted client needs a token issuance");

        assert!(matches!(
            error,
            CodegenError::AcceptedClients {
                source: AcceptedClientsCodegenError::AcceptedWithoutTokenIssuance { anchor }
            } if anchor == "crate::PortalClient"
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


#[issues_tokens(audience = \"session\", issuer = \"https://issuer.fixture\")]
struct Issuer;

#[singleton]
struct Watcher;

impl Watcher {
    #[constructor]
    fn create(
        handler: std::sync::Arc<crate::margaret::oidc_provider::ProviderMetadataHandler>,
    ) -> anyhow::Result<Self> {}
}
",
        )
        .expect_err("the provider metadata needs a route");

        assert_eq!(
            error.to_string(),
            "failed to derive the openid connect provider endpoints: no route serves the discovery document"
        );
    }

    #[test]
    fn generates_the_sign_in_flow_of_an_oauth_client() {
        let code = generate(SIGN_IN_CRATE).expect("the build succeeds");
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(construction.contains(
            "crate::margaret::oauth_clients::partner_client::SignInFlow::create(::std::sync::Arc::<crate::margaret::oauth_clients::partner_client::AuthorizationServerClient,>::clone(&margaret_oauth_clients_partner_client_authorization_server_client),::std::sync::Arc::<crate::margaret::jwks::JwksRoller,>::clone(&margaret_jwks_jwks_roller),)"
        ));
    }

    #[test]
    fn signs_in_an_application_that_issues_no_tokens() {
        let code = generate(&SIGN_IN_CRATE.replace(
            "#[issues_tokens(audience = \"session\", issuer = \"https://issuer.fixture\")]\nstruct Issuer;\n",
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
                "#[rustfmt::skip]\npub mod margaret;\n\n#[provides_jwks_endpoint(partner, audience = \"api\", issuer = \"https://partner.example\", jwks_uri = \"https://partner.example/jwks\")]\nstruct Endpoint;\n\n#[oauth_client(partner_client, authentication = private_key_jwt, client_id = \"partner\", issuer = partner)]\nstruct PartnerClient;\n"
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
                "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[exchanges_subject_tokens(issuer = ci)]\nstruct CiExchanger;\n"
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

#[trusts_oidc_issuer(partner, audience = \"api\", issuer = \"https://partner.example\")]
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
#[responds_to_http(method = \"get\", path = \"/runner\", server = \"public\")]
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


#[trusts_oidc_issuer(partner, audience = \"api\", issuer = \"https://partner.example\")]
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

    const JWKS_BEARER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;

#[provides_jwks_endpoint(auth, audience = \"api\", issuer = \"https://auth.example\", jwks_uri = \"https://auth.example/jwks\")]
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
#[responds_to_http(method = \"get\", path = \"/holder\", server = \"public\")]
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
#[responds_to_http(method = \"get\", path = \"/.well-known/jwks.json\", server = \"internal\")]
struct GetJwks {
    handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>,
}

impl GetJwks {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn framework_jwks_names_do_not_collide_with_a_user_component_of_the_same_name() {
        let code = generate(JWKS_NAME_COLLISION_CRATE)
            .expect("a user component named JwksRoller coexists");

        let container: String = module(&code, "container").split_whitespace().collect();

        assert!(container.contains("std::sync::Arc<crate::JwksRoller>"));
        assert!(container.contains("std::sync::Arc<crate::margaret::jwks::JwksRoller>"));
    }

    const JWKS_FIELD_COLLISION_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

pub mod margaret_jwks {
    #[singleton]
    pub struct JwksRoller;
}

#[singleton]
#[responds_to_http(method = \"get\", path = \"/.well-known/jwks.json\", server = \"internal\")]
struct GetJwks {
    handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>,
}

impl GetJwks {
    #[constructor]
    fn create(handler: std::sync::Arc<crate::margaret::jwks::PublicJwksHandler>) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
";

    #[test]
    fn framework_jwks_field_is_disambiguated_from_a_colliding_user_component() {
        let code = generate(JWKS_FIELD_COLLISION_CRATE)
            .expect("a user component flattening to a framework field coexists");

        let container: String = module(&code, "container").split_whitespace().collect();
        let construction: String = module(&code, "container/build/serve")
            .split_whitespace()
            .collect();

        assert!(container.contains("std::sync::Arc<crate::margaret_jwks::JwksRoller>"));
        assert!(construction.contains("margaret_jwks_jwks_roller_2"));
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

        assert!(message.contains("references the tag 'missing', which no trusted issuer declares"));
    }

    const JWKS_DUPLICATE_ENDPOINT_TAG_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;


#[provides_jwks_endpoint(auth, audience = \"api\", issuer = \"https://first.example\", jwks_uri = \"https://first.example/jwks\")]
struct FirstEndpoint;



#[provides_jwks_endpoint(auth, audience = \"api\", issuer = \"https://second.example\", jwks_uri = \"https://second.example/jwks\")]
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
    fn generates_only_container_without_responders() {
        let code = generate(PLAIN_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod container;"));
        assert!(!module(&code, "mod").contains("pub mod http;"));
        assert!(!module(&code, "mod").contains("pub mod routes;"));
        assert!(!module(&code, "mod").contains("pub mod run;"));
        assert!(!has_module(&code, "http"));
        assert!(!has_module(&code, "routes"));
        assert!(!has_module(&code, "run"));
        assert!(module(&code, "container").contains("struct Container"));
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
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]
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
#[responds_to_http(method = \"get\", path = \"/profile\", server = \"public\")]
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
    fn omits_the_authenticated_users_module_without_a_served_request() {
        let code = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\nuse margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;\n\nstruct User;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
        )
        .expect("the build succeeds");

        assert!(!module(&code, "mod").contains("pub mod authenticated_users;"));
        assert!(!has_module(&code, "authenticated_users"));
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

#[websocket_message(request, method = \"chat\", response = single)]
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
        assert!(!generated.join("run.rs").exists());
        assert!(!umbrella_source.contains("pub mod http;"));
        assert!(!umbrella_source.contains("pub mod routes;"));
        assert!(!umbrella_source.contains("pub mod run;"));
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
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]
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
#[responds_to_http(method = \"get\", path = \"/card\", server = \"public\")]
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

#[websocket_message(request, method = \"chat\", response = single)]
struct Chat;

#[singleton]
struct Chatter;

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
";

    #[test]
    fn omits_the_views_module_for_a_crate_that_serves_no_responders() {
        let code = generate(WEBSOCKET_ONLY_VIEW_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod websocket;"));
        assert!(!module(&code, "mod").contains("pub mod views;"));
        assert!(!has_module(&code, "views"));
        assert!(!concatenated(&code).contains("views::build::build"));
    }

    #[test]
    fn propagates_a_views_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[renders_view(name = \"CardLayout\")]\nstruct Card;\n\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
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
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nstruct Pool;\n\nimpl Pool {\n    #[constructor]\n    async fn create() -> anyhow::Result<Self> {}\n}\n\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
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
#[responds_to_http(method = \"get\", path = \"/\", server = \"public\")]
struct Index;

impl Index {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = \"get\", path = \"/metrics\", server = \"internal\")]
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
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/\", server = crate::Ghost)]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
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
            "#[rustfmt::skip]\npub mod margaret;\n\nmod routes {\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/a\", server = \"public\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n}\n\nmod endpoints {\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/b\", server = \"internal\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n}\n",
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
#[responds_to_http(method = \"get\", name = \"origin\", path = \"/o\", server = \"routes\")]
struct Origin;

impl Origin {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = \"get\", name = \"new\", path = \"/n/{id}\", server = \"routes\")]
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

        let container: String = module(&code, "container").split_whitespace().collect();

        assert!(build.contains("pubfnserve("));
        assert!(container.contains("std::sync::Arc<crate::Build>"));
        assert!(container.contains("std::sync::Arc<crate::Container>"));
        assert!(container.contains("std::sync::Arc<crate::Routes>"));
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
    fn propagates_a_token_issuance_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[issues_tokens(issuer = \"https://issuer.fixture\")]\nstruct Issuer;\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to read the token issuance"));
    }

    #[test]
    fn propagates_a_trusted_issuer_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[trusts_oidc_issuer(partner, issuer = \"https://partner.fixture\")]\nstruct Partner;\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to read the trusted issuers"));
    }

    #[test]
    fn propagates_an_oauth_client_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[oauth_client(partner_client, client_id = \"partner\", issuer = partner)]\nstruct PartnerClient;\n",
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
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]
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
        assert!(serve.contains("server_public(container,&routes"));
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
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]
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

#[websocket_message(request, method = \"chat\", response = single)]
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
        assert!(serve.contains("server_public(container,&routes"));
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

#[websocket_message(request, method = \"chat\", response = single)]
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
        assert!(serve.contains("server_public(container,&routes"));
        assert!(!serve.contains("server_public(container,&serve_input_0"));
    }
}
