use margaret_attributes::attribute_args::AttributeArgs;

use crate::http_codegen_error::HttpCodegenError;

fn normalized_method(method: &str, responder: &str) -> Result<String, HttpCodegenError> {
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
        arguments.interpret(|reader| {
            let declared_method = reader.take_string("method")?.ok_or_else(|| {
                HttpCodegenError::MissingHttpMethod {
                    responder: responder.to_string(),
                }
            })?;
            let method = normalized_method(&declared_method, responder)?;
            let path =
                reader
                    .take_string("path")?
                    .ok_or_else(|| HttpCodegenError::MissingHttpPath {
                        responder: responder.to_string(),
                    })?;
            let name = reader.take_string("name")?;
            let server = reader.take_string("server")?.ok_or_else(|| {
                HttpCodegenError::MissingHttpServer {
                    responder: responder.to_string(),
                }
            })?;

            Ok(Self {
                method,
                name,
                path,
                server,
            })
        })
    }
}
