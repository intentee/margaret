use crate::web_socket_codegen_error::WebSocketCodegenError;
use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;

pub(crate) struct SessionArguments {
    pub(crate) origin: String,
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
            let Some(origin) = reader.take_string("origin")? else {
                return Err(WebSocketCodegenError::MissingSessionOrigin {
                    session: session.to_string(),
                });
            };
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

            let parsed = url::Url::parse(&origin).map_err(|source| {
                WebSocketCodegenError::InvalidSessionOrigin {
                    origin: origin.clone(),
                    session: session.to_string(),
                    source,
                }
            })?;
            let canonical_origin = parsed.origin().ascii_serialization();
            let canonical = parsed.scheme() == "https"
                && parsed.username().is_empty()
                && parsed.password().is_none()
                && parsed.path() == "/"
                && parsed.query().is_none()
                && parsed.fragment().is_none()
                && origin == canonical_origin;

            if !canonical {
                return Err(WebSocketCodegenError::NonCanonicalSessionOrigin {
                    origin,
                    session: session.to_string(),
                });
            }

            Ok(Self {
                origin: canonical_origin,
                path,
                server,
            })
        })
    }
}
