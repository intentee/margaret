use std::num::NonZeroU64;

use quote::ToTokens;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;
use margaret_route_method::route_method::RouteMethod;

use crate::http_codegen_error::HttpCodegenError;
use crate::route_methods::ROUTE_METHODS;

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
        index: &AttributeIndex,
        item: &IndexedItem,
        responder: &str,
    ) -> Result<Self, HttpCodegenError> {
        arguments.interpret(|reader| {
            let declared_method = reader
                .take_path(ItemNamingArgument::RouteMethod.key())?
                .ok_or_else(|| HttpCodegenError::MissingHttpMethod {
                    responder: responder.to_string(),
                })?;
            let method = index
                .resolve_item_path(item, &declared_method)
                .as_ref()
                .and_then(|resolved| ROUTE_METHODS.variant(resolved))
                .ok_or_else(|| HttpCodegenError::UnknownHttpMethod {
                    responder: responder.to_string(),
                    written: declared_method.to_token_stream().to_string(),
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
