use margaret_attributes::attribute_args::AttributeArgs;

use crate::http_codegen_error::HttpCodegenError;

fn normalized_method(method: String, responder: &str) -> Result<String, HttpCodegenError> {
    let method = method.to_uppercase();

    http::Method::from_bytes(method.as_bytes()).map_err(|source| {
        HttpCodegenError::InvalidHttpMethod {
            responder: responder.to_string(),
            method: method.clone(),
            source,
        }
    })?;

    Ok(method)
}

pub(crate) struct HttpResponderArguments {
    pub(crate) method: String,
    pub(crate) name: Option<String>,
    pub(crate) path: String,
    pub(crate) server: String,
}

impl HttpResponderArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        responder: &str,
    ) -> Result<Self, HttpCodegenError> {
        let method = normalized_method(
            arguments
                .string("method")?
                .ok_or_else(|| HttpCodegenError::MissingHttpMethod {
                    responder: responder.to_string(),
                })?,
            responder,
        )?;
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
