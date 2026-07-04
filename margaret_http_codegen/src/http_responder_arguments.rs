use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;

use crate::http_codegen_error::HttpCodegenError;

pub(crate) struct HttpResponderArguments {
    pub(crate) method: Path,
    pub(crate) name: Option<String>,
    pub(crate) path: String,
    pub(crate) server: String,
}

impl HttpResponderArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        responder: &str,
    ) -> Result<Self, HttpCodegenError> {
        let method =
            arguments
                .path("method")?
                .ok_or_else(|| HttpCodegenError::MissingHttpMethod {
                    responder: responder.to_string(),
                })?;
        let path = arguments
            .string("path")?
            .ok_or_else(|| HttpCodegenError::MissingHttpPath {
                responder: responder.to_string(),
            })?;
        let name = arguments.string("name")?;
        let server =
            arguments
                .string("server")?
                .ok_or_else(|| HttpCodegenError::MissingHttpServer {
                    responder: responder.to_string(),
                })?;

        Ok(Self {
            method,
            name,
            path,
            server,
        })
    }
}
