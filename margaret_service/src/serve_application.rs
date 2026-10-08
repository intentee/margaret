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
use crate::server_uploads::ServerUploads;

fn upload_config(
    matches: &ArgMatches,
    uploads: ServerUploads,
) -> Result<UploadConfig, CommandOutcome> {
    match uploads {
        ServerUploads::Accepted { directory_argument } => matches
            .get_one::<PathBuf>(directory_argument)
            .map(|directory| UploadConfig::enabled(directory.clone()))
            .ok_or(CommandOutcome::Failed),
        ServerUploads::Refused => Ok(UploadConfig::Disabled),
    }
}

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
        uploads,
    } in servers
    {
        let Some(address) = matches.get_one::<String>(address_argument).cloned() else {
            return Err(CommandOutcome::Failed);
        };
        let server_routes = match routes {
            Ok(server_routes) => server_routes,
            Err(error) => return Err(report_failure(error)),
        };
        let upload_config = upload_config(matches, uploads)?;

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
    use std::path::PathBuf;

    use clap::Arg;
    use clap::ArgMatches;
    use clap::Command;

    use margaret_console::command_outcome::CommandOutcome;
    use margaret_http::route_entry::RouteEntry;
    use margaret_http::router::Router;
    use margaret_http::router_error::RouterError;
    use margaret_http::server_routes::ServerRoutes;
    use margaret_http::transport_config::TransportConfig;

    use margaret_http_uploaded_file::upload_config::UploadConfig;

    use super::serve_application;
    use super::upload_config;
    use crate::server_assembly::ServerAssembly;
    use crate::server_uploads::ServerUploads;

    fn matches(arguments: &[&str]) -> ArgMatches {
        Command::new("test")
            .arg(Arg::new("public-addr").long("public-addr"))
            .arg(
                Arg::new("public-upload-dir")
                    .long("public-upload-dir")
                    .value_parser(clap::value_parser!(PathBuf)),
            )
            .try_get_matches_from(iter::once("test").chain(arguments.iter().copied()))
            .expect("the test arguments parse")
    }

    fn public_assembly(routes: Result<ServerRoutes, RouterError>) -> ServerAssembly {
        ServerAssembly {
            address_argument: "public-addr",
            routes,
            transport: TransportConfig::Plain,
            uploads: ServerUploads::Refused,
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

    const ACCEPTED_UPLOADS: ServerUploads = ServerUploads::Accepted {
        directory_argument: "public-upload-dir",
    };

    #[test]
    fn accepts_uploads_into_the_declared_directory() {
        assert_eq!(
            upload_config(
                &matches(&["--public-upload-dir", "/srv/margaret-uploads"]),
                ACCEPTED_UPLOADS
            ),
            Ok(UploadConfig::enabled(PathBuf::from(
                "/srv/margaret-uploads"
            )))
        );
    }

    #[test]
    fn reports_failure_when_the_upload_directory_of_an_accepting_server_is_missing() {
        assert_eq!(
            upload_config(&matches(&[]), ACCEPTED_UPLOADS),
            Err(CommandOutcome::Failed)
        );
    }

    #[test]
    fn refuses_uploads_of_a_server_without_upload_routes() {
        assert_eq!(
            upload_config(&matches(&[]), ServerUploads::Refused),
            Ok(UploadConfig::Disabled)
        );
    }

    #[test]
    fn reports_failure_when_an_accepting_server_lacks_its_upload_directory() {
        let outcome = serve_application(
            &matches(&["--public-addr", "127.0.0.1:0"]),
            vec![ServerAssembly {
                uploads: ACCEPTED_UPLOADS,
                ..public_assembly(Ok(empty_routes()))
            }],
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
