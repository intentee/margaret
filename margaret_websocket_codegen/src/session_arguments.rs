use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;

use crate::web_socket_codegen_error::WebSocketCodegenError;

pub(crate) struct SessionArguments {
    pub(crate) path: String,
    pub(crate) server: String,
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

            if !is_snake_case_identifier(&server) {
                return Err(WebSocketCodegenError::InvalidSessionServer {
                    session: session.to_string(),
                    server,
                });
            }

            Ok(Self { path, server })
        })
    }
}
