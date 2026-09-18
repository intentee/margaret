use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_server_codegen::server_name::ServerName;

use crate::web_socket_codegen_error::WebSocketCodegenError;

pub(crate) struct SessionArguments {
    pub(crate) path: String,
    pub(crate) server: ServerName,
}

impl SessionArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        session: &str,
    ) -> Result<Self, WebSocketCodegenError> {
        arguments.interpret(|reader| {
            let path = reader.take_string("path")?.ok_or_else(|| {
                WebSocketCodegenError::MissingSessionPath {
                    session: session.to_string(),
                }
            })?;
            let server = reader.take_string("server")?.ok_or_else(|| {
                WebSocketCodegenError::MissingSessionServer {
                    session: session.to_string(),
                }
            })?;
            let server = ServerName::parse(server).map_err(|source| {
                WebSocketCodegenError::InvalidSessionServer {
                    session: session.to_string(),
                    source,
                }
            })?;

            Ok(Self { path, server })
        })
    }
}
