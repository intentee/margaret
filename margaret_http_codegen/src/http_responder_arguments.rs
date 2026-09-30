use std::num::NonZeroU64;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_route_method::route_method::RouteMethod;

use crate::http_codegen_error::HttpCodegenError;

pub(crate) struct HttpResponderArguments {
    pub(crate) max_body_bytes: Option<NonZeroU64>,
    pub(crate) method: RouteMethod,
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
            let method = RouteMethod::from_attribute_value(&declared_method).ok_or_else(|| {
                HttpCodegenError::UnsupportedHttpMethod {
                    responder: responder.to_string(),
                    method: declared_method.clone(),
                }
            })?;
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
            let max_body_bytes = reader.take_unsigned_integer("max_body_bytes")?;

            Ok(Self {
                max_body_bytes,
                method,
                name,
                path,
                server,
            })
        })
    }
}
