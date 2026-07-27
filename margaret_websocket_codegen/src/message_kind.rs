use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;

use crate::message_cardinality::MessageCardinality;
use crate::web_socket_codegen_error::WebSocketCodegenError;

fn cardinality(
    reader: &mut AttributeArgumentsReader,
    message: &str,
) -> Result<MessageCardinality, WebSocketCodegenError> {
    match reader.take_path("response")? {
        None => Err(WebSocketCodegenError::MissingCardinality {
            message: message.to_string(),
        }),
        Some(path) if path.is_ident("single") => Ok(MessageCardinality::Single),
        Some(path) if path.is_ident("stream") => Ok(MessageCardinality::Stream),
        Some(_) => Err(WebSocketCodegenError::InvalidCardinality {
            message: message.to_string(),
        }),
    }
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
    if reader.take_path("response")?.is_some() {
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
        message: &str,
    ) -> Result<Self, WebSocketCodegenError> {
        arguments.interpret(|reader| {
            let is_request = reader.take_flag("request");
            let is_notification = reader.take_flag("notification");
            let is_response = reader.take_flag("response");

            match (is_request, is_notification, is_response) {
                (true, false, false) => {
                    let cardinality = cardinality(reader, message)?;

                    Ok(Self::Request {
                        cardinality,
                        method: method(reader, message)?,
                    })
                }
                (false, true, false) => {
                    reject_cardinality(reader, message)?;

                    Ok(Self::Notification {
                        method: method(reader, message)?,
                    })
                }
                (false, false, true) => {
                    reject_cardinality(reader, message)?;

                    Ok(Self::Response {
                        method: method(reader, message)?,
                    })
                }
                (false, false, false) => Err(WebSocketCodegenError::MissingMessageKind {
                    message: message.to_string(),
                }),
                _ => Err(WebSocketCodegenError::InvalidMessageKind {
                    message: message.to_string(),
                }),
            }
        })
    }
}
