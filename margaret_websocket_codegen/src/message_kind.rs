use quote::ToTokens;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;

use crate::message_cardinality::MessageCardinality;
use crate::web_socket_codegen_error::WebSocketCodegenError;
use crate::web_socket_responses::WEB_SOCKET_RESPONSES;

fn cardinality(
    reader: &mut AttributeArgumentsReader,
    index: &AttributeIndex,
    item: &IndexedItem,
    message: &str,
) -> Result<MessageCardinality, WebSocketCodegenError> {
    let declared = reader
        .take_path(ItemNamingArgument::WebSocketResponse.key())?
        .ok_or_else(|| WebSocketCodegenError::MissingCardinality {
            message: message.to_string(),
        })?;

    index
        .resolve_item_path(item, &declared)
        .as_ref()
        .and_then(|resolved| WEB_SOCKET_RESPONSES.variant(resolved))
        .ok_or_else(|| WebSocketCodegenError::UnknownCardinality {
            message: message.to_string(),
            written: declared.to_token_stream().to_string(),
        })
}

fn method(
    reader: &mut AttributeArgumentsReader,
    message: &str,
) -> Result<String, WebSocketCodegenError> {
    let method =
        reader
            .take_string("method")?
            .ok_or_else(|| WebSocketCodegenError::MissingMethod {
                message: message.to_string(),
            })?;

    if !is_snake_case_identifier(&method) {
        return Err(WebSocketCodegenError::InvalidMethod {
            message: message.to_string(),
            method,
        });
    }

    Ok(method)
}

fn reject_cardinality(
    reader: &mut AttributeArgumentsReader,
    message: &str,
) -> Result<(), WebSocketCodegenError> {
    if reader
        .take_path(ItemNamingArgument::WebSocketResponse.key())?
        .is_some()
    {
        return Err(WebSocketCodegenError::CardinalityOnNonRequest {
            message: message.to_string(),
        });
    }

    Ok(())
}

pub(crate) enum MessageKind {
    Notification {
        method: String,
    },
    Request {
        cardinality: MessageCardinality,
        method: String,
    },
    Response {
        method: String,
    },
}

impl MessageKind {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        index: &AttributeIndex,
        item: &IndexedItem,
        message: &str,
    ) -> Result<Self, WebSocketCodegenError> {
        arguments.interpret(|reader| {
            let is_request = reader.take_flag("request");
            let is_notification = reader.take_flag("notification");
            let is_response = reader.take_flag("response");

            let declared_kinds = [is_request, is_notification, is_response]
                .into_iter()
                .filter(|declared| *declared)
                .count();

            match declared_kinds {
                0 => Err(WebSocketCodegenError::MissingMessageKind {
                    message: message.to_string(),
                }),
                1 if is_request => {
                    let cardinality = cardinality(reader, index, item, message)?;

                    Ok(Self::Request {
                        cardinality,
                        method: method(reader, message)?,
                    })
                }
                1 if is_notification => {
                    reject_cardinality(reader, message)?;

                    Ok(Self::Notification {
                        method: method(reader, message)?,
                    })
                }
                1 => {
                    reject_cardinality(reader, message)?;

                    Ok(Self::Response {
                        method: method(reader, message)?,
                    })
                }
                _ => Err(WebSocketCodegenError::InvalidMessageKind {
                    message: message.to_string(),
                }),
            }
        })
    }
}
