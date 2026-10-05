use std::env;
use std::path::PathBuf;
use std::sync::Arc;

use clap::ArgMatches;

use margaret_console::command_outcome::CommandOutcome;
use margaret_console::report_failure::report_failure;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::server::Server;
use margaret_http_uploaded_file::upload_config::UploadConfig;

use crate::server_assembly::ServerAssembly;
use crate::server_service::ServerService;

/// # Errors
///
/// Returns `CommandOutcome::Failed`.
pub fn serve_application(
    matches: &ArgMatches,
    servers: Vec<ServerAssembly>,
) -> Result<Vec<ServerService>, CommandOutcome> {
    let mut server_services = Vec::new();

    for ServerAssembly {
        address_argument,
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
                    .map_or_else(env::temp_dir, PathBuf::from),
            )
        } else {
            UploadConfig::Disabled
        };

        server_services.push(ServerService::new(
            Arc::new(Server::new(
                address,
                transport,
                upload_config,
                server_routes.router,
            )),
            Arc::new(ForwardTargets::new(server_routes.named_handlers)),
        ));
    }

    Ok(server_services)
}

#[cfg(test)]
mod tests {
    use std::iter;

    use clap::Arg;
    use clap::ArgAction;
    use clap::ArgMatches;
    use clap::Command;

    use margaret_console::command_outcome::CommandOutcome;
    use margaret_http::matchit::InsertError;
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
            .try_get_matches_from(iter::once("test").chain(arguments.iter().copied()))
            .expect("the test arguments parse")
    }

    fn public_assembly(routes: Result<ServerRoutes, InsertError>) -> ServerAssembly {
        ServerAssembly {
            address_argument: "public-addr",
            routes,
            transport: TransportConfig::Plain,
            upload_dir_argument: "public-upload-dir",
            uploads_argument: "public-uploads",
        }
    }

    fn empty_routes() -> ServerRoutes {
        ServerRoutes::build(Vec::new()).expect("an empty entry list builds server routes")
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
    fn enables_uploads_into_the_temporary_directory_by_default() {
        let outcome = serve_application(
            &matches(&["--public-addr", "127.0.0.1:0", "--public-uploads"]),
            vec![public_assembly(Ok(empty_routes()))],
        );

        assert!(outcome.is_ok());
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
