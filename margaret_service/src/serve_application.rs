use std::path::PathBuf;
use std::sync::Arc;

use clap::ArgMatches;
use trzcina::ServiceBundle;
use trzcina::ServiceManager;

use margaret_console::command_outcome::CommandOutcome;
use margaret_console::report_failure::report_failure;
use margaret_http::body_limit::BodyLimit;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::upload_config::UploadConfig;

use crate::server_assembly::ServerAssembly;
use crate::server_service::ServerService;

pub async fn serve_application<TServiceBundle: ServiceBundle>(
    matches: &ArgMatches,
    servers: Vec<ServerAssembly>,
    bundle: TServiceBundle,
) -> Result<ServiceManager, CommandOutcome> {
    let mut manager = ServiceManager::default();
    let mut server_models = Vec::new();
    let mut server_forward_targets = Vec::new();

    for ServerAssembly {
        address_argument,
        name,
        routes,
        transport,
        upload_dir_argument,
        uploads_argument,
    } in servers
    {
        let Some(address) = matches.get_one::<String>(address_argument).cloned() else {
            return Err(CommandOutcome::Failed);
        };
        let server_routes = match routes {
            Ok(server_routes) => server_routes,
            Err(error) => return Err(report_failure(error)),
        };
        let upload_config = if matches.get_flag(uploads_argument) {
            UploadConfig::enabled(
                matches
                    .get_one::<String>(upload_dir_argument)
                    .map(PathBuf::from)
                    .unwrap_or_else(std::env::temp_dir),
            )
        } else {
            UploadConfig::Disabled
        };

        server_forward_targets.push((
            Arc::new(ForwardTargets::new(server_routes.named_handlers)),
            name,
        ));
        server_models.push(Server::new(
            name,
            address,
            transport,
            upload_config,
            BodyLimit::default(),
            server_routes.router,
        ));
    }

    if let Err(error) = manager.register_bundle(bundle).await {
        return Err(report_failure(error));
    }

    let server_registry = Arc::new(ServerRegistry::new(server_models));

    for (forward_targets, name) in server_forward_targets {
        manager.register_service(ServerService::new(
            server_registry.clone(),
            forward_targets,
            name,
        ));
    }

    Ok(manager)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use anyhow::bail;
    use async_trait::async_trait;
    use clap::Arg;
    use clap::ArgAction;
    use clap::ArgMatches;
    use clap::Command;
    use trzcina::Service;
    use trzcina::ServiceBundle;

    use margaret_console::command_outcome::CommandOutcome;
    use margaret_http::route_entry::RouteEntry;
    use margaret_http::router::Router;
    use margaret_http::server_routes::ServerRoutes;
    use margaret_http::transport_config::TransportConfig;

    use super::serve_application;
    use crate::resolved_services::ResolvedServices;
    use crate::server_assembly::ServerAssembly;

    struct FailingBundle;

    #[async_trait]
    impl ServiceBundle for FailingBundle {
        async fn services(self) -> Result<Vec<Box<dyn Service>>> {
            bail!("the bundle could not provide its services")
        }
    }

    fn matches(arguments: &[&str]) -> ArgMatches {
        Command::new("test")
            .arg(Arg::new("public-addr").long("public-addr"))
            .arg(
                Arg::new("public-uploads")
                    .long("public-uploads")
                    .action(ArgAction::SetTrue),
            )
            .arg(Arg::new("public-upload-dir").long("public-upload-dir"))
            .try_get_matches_from(std::iter::once("test").chain(arguments.iter().copied()))
            .expect("the test arguments parse")
    }

    fn public_assembly(
        routes: std::result::Result<ServerRoutes, margaret_http::matchit::InsertError>,
    ) -> ServerAssembly {
        ServerAssembly {
            address_argument: "public-addr",
            name: "public",
            routes,
            transport: TransportConfig::Plain,
            upload_dir_argument: "public-upload-dir",
            uploads_argument: "public-uploads",
        }
    }

    fn empty_routes() -> ServerRoutes {
        ServerRoutes::new(
            Router::build(Vec::new()).expect("an empty router builds"),
            Vec::new(),
        )
    }

    #[tokio::test]
    async fn assembles_a_manager_with_a_bundle() {
        let outcome = serve_application(
            &matches(&["--public-addr", "127.0.0.1:0"]),
            vec![public_assembly(Ok(empty_routes()))],
            ResolvedServices {
                services: Vec::new(),
            },
        )
        .await;

        assert!(outcome.is_ok());
    }

    #[tokio::test]
    async fn enables_uploads_into_a_configured_directory() {
        let outcome = serve_application(
            &matches(&[
                "--public-addr",
                "127.0.0.1:0",
                "--public-uploads",
                "--public-upload-dir",
                "/tmp/margaret-uploads",
            ]),
            vec![public_assembly(Ok(empty_routes()))],
            ResolvedServices {
                services: Vec::new(),
            },
        )
        .await;

        assert!(outcome.is_ok());
    }

    #[tokio::test]
    async fn enables_uploads_into_the_temporary_directory_by_default() {
        let outcome = serve_application(
            &matches(&["--public-addr", "127.0.0.1:0", "--public-uploads"]),
            vec![public_assembly(Ok(empty_routes()))],
            ResolvedServices {
                services: Vec::new(),
            },
        )
        .await;

        assert!(outcome.is_ok());
    }

    #[tokio::test]
    async fn reports_failure_when_a_server_address_is_missing() {
        let outcome = serve_application(
            &matches(&[]),
            vec![public_assembly(Ok(empty_routes()))],
            ResolvedServices {
                services: Vec::new(),
            },
        )
        .await;

        assert_eq!(outcome.err(), Some(CommandOutcome::Failed));
    }

    #[tokio::test]
    async fn reports_failure_when_a_server_router_conflicts() {
        let conflict = Router::build(vec![
            RouteEntry::new("/conflict", Vec::new()),
            RouteEntry::new("/conflict", Vec::new()),
        ])
        .err()
        .expect("duplicate routes conflict");

        let outcome = serve_application(
            &matches(&["--public-addr", "127.0.0.1:0"]),
            vec![public_assembly(Err(conflict))],
            ResolvedServices {
                services: Vec::new(),
            },
        )
        .await;

        assert_eq!(outcome.err(), Some(CommandOutcome::Failed));
    }

    #[tokio::test]
    async fn reports_failure_when_the_bundle_cannot_register() {
        let outcome = serve_application(
            &matches(&["--public-addr", "127.0.0.1:0"]),
            vec![public_assembly(Ok(empty_routes()))],
            FailingBundle,
        )
        .await;

        assert_eq!(outcome.err(), Some(CommandOutcome::Failed));
    }
}
