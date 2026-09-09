use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::serves_spiffe::serves_spiffe;
use margaret_serve_input_codegen::has_spiffe_http_client::has_spiffe_http_client;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::console_codegen_error::ConsoleCodegenError;

pub(crate) struct ServeCommand {
    pub(crate) http_servers: Vec<HttpServer>,
    pub(crate) registers_spiffe_arguments: bool,
    pub(crate) registers_transport_arguments: bool,
    pub(crate) serve_inputs: Vec<ServeInput>,
}

fn register_argument(
    registered: &mut BTreeMap<String, String>,
    name: &str,
    owner: String,
) -> Result<(), ConsoleCodegenError> {
    match registered.entry(name.to_string()) {
        Entry::Occupied(entry) => Err(ConsoleCodegenError::ServeArgumentNameCollision {
            first_owner: entry.get().clone(),
            name: name.to_string(),
            owner,
        }),
        Entry::Vacant(entry) => {
            entry.insert(owner);
            Ok(())
        }
    }
}

fn register_server_arguments(
    registered: &mut BTreeMap<String, String>,
    server: &HttpServer,
    registers_transport_arguments: bool,
) {
    let server_name = server.name();

    for (name, purpose) in [
        (server.address_argument(), "address"),
        (server.body_limit_argument(), "body limit"),
        (server.url_argument(), "URL"),
        (server.uploads_argument(), "uploads switch"),
        (server.upload_dir_argument(), "upload directory"),
    ] {
        registered.insert(
            name.to_string(),
            format!("HTTP server '{server_name}' {purpose}"),
        );
    }

    if registers_transport_arguments {
        registered.insert(
            server.transport_argument().to_string(),
            format!("HTTP server '{server_name}' transport"),
        );
    }
}

impl ServeCommand {
    /// # Errors
    ///
    /// Returns `ConsoleCodegenError::ServeArgumentNameCollision` when two active inputs claim the
    /// same Clap argument name.
    pub(crate) fn build(
        http_servers: &[HttpServer],
        serve_inputs: &[ServeInput],
    ) -> Result<Self, ConsoleCodegenError> {
        let registers_transport_arguments = serves_spiffe(http_servers);
        let registers_spiffe_arguments =
            registers_transport_arguments || has_spiffe_http_client(serve_inputs);
        let mut registered = BTreeMap::new();

        for server in http_servers {
            register_server_arguments(&mut registered, server, registers_transport_arguments);
        }

        if registers_spiffe_arguments {
            registered.insert(
                "spiffe-trust-domain".to_string(),
                "SPIFFE trust domain".to_string(),
            );
            registered.insert(
                "spire-agent-addr".to_string(),
                "SPIRE agent address".to_string(),
            );
        }

        for input in serve_inputs {
            if matches!(input, ServeInput::ConsoleArgument(_)) {
                register_argument(
                    &mut registered,
                    input.name(),
                    "consumer serve input".to_string(),
                )?;
            }
        }

        Ok(Self {
            http_servers: http_servers.to_vec(),
            registers_spiffe_arguments,
            registers_transport_arguments,
            serve_inputs: serve_inputs.to_vec(),
        })
    }
}
