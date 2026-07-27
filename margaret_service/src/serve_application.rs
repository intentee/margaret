use std::path::PathBuf;
use std::sync::Arc;

use clap::ArgMatches;

use margaret_console::command_outcome::CommandOutcome;
use margaret_console::report_failure::report_failure;
use margaret_http::body_limit::BodyLimit;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::upload_config::UploadConfig;

use crate::server_assembly::ServerAssembly;
use crate::server_service::ServerService;

pub fn serve_application(
    matches: &ArgMatches,
    servers: Vec<ServerAssembly>,
) -> Result<Vec<ServerService>, CommandOutcome> {
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
            let Some(directory) = matches.get_one::<String>(upload_dir_argument) else {
                return Err(CommandOutcome::Failed);
            };

            UploadConfig::enabled(PathBuf::from(directory))
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

    let server_registry = Arc::new(ServerRegistry::new(server_models));
    let mut server_services = Vec::new();

    for (forward_targets, name) in server_forward_targets {
        server_services.push(ServerService::new(
            server_registry.clone(),
            forward_targets,
            name,
        ));
    }

    Ok(server_services)
}

#[cfg(test)]
mod tests {
    use clap::Arg;
    use clap::ArgAction;
    use clap::ArgMatches;
    use clap::Command;

    use margaret_console::command_outcome::CommandOutcome;
    use margaret_http::route_entry::RouteEntry;
    use margaret_http::router::Router;
    use margaret_http::server_routes::ServerRoutes;
    use margaret_http::transport_config::TransportConfig;

    use super::serve_application;
    use crate::server_assembly::ServerAssembly;

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
        routes: std::result::Result<ServerRoutes, margaret_http::router_error::RouterError>,
    ) -> ServerAssembly {
        ServerAssembly {
            address_argument: "public-addr",
            name: "public",
            routes,
            transport: TransportConfig::FixturePlain,
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

    #[test]
    fn assembles_one_server_service_per_assembly() {
        let server_services = serve_application(
            &matches(&["--public-addr", "127.0.0.1:0"]),
            vec![public_assembly(Ok(empty_routes()))],
        )
        .expect("the assembly succeeds");

        assert_eq!(server_services.len(), 1);
    }

    #[test]
    fn enables_uploads_into_a_configured_directory() {
        let outcome = serve_application(
            &matches(&[
                "--public-addr",
                "127.0.0.1:0",
                "--public-uploads",
                "--public-upload-dir",
                "/tmp/margaret-uploads",
            ]),
            vec![public_assembly(Ok(empty_routes()))],
        );

        assert!(outcome.is_ok());
    }

    #[test]
    fn rejects_uploads_without_an_explicit_directory() {
        let outcome = serve_application(
            &matches(&["--public-addr", "127.0.0.1:0", "--public-uploads"]),
            vec![public_assembly(Ok(empty_routes()))],
        );

        assert_eq!(outcome.err(), Some(CommandOutcome::Failed));
    }

    #[test]
    fn reports_failure_when_a_server_address_is_missing() {
        let outcome = serve_application(&matches(&[]), vec![public_assembly(Ok(empty_routes()))]);

        assert_eq!(outcome.err(), Some(CommandOutcome::Failed));
    }

    #[test]
    fn reports_failure_when_a_server_router_conflicts() {
        let conflict = Router::build(vec![
            RouteEntry::new("/conflict", Vec::new()),
            RouteEntry::new("/conflict", Vec::new()),
        ])
        .err()
        .expect("duplicate routes conflict");

        let outcome = serve_application(
            &matches(&["--public-addr", "127.0.0.1:0"]),
            vec![public_assembly(Err(conflict))],
        );

        assert_eq!(outcome.err(), Some(CommandOutcome::Failed));
    }
}
