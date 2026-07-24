use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;

use crate::message_cardinality::MessageCardinality;
use crate::websocket_codegen_error::WebSocketCodegenError;

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
        arguments.expect_only(
            &["method", "response"],
            &["notification", "request", "response"],
        )?;

        let is_request = arguments.has_positional_flag("request");
        let is_notification = arguments.has_positional_flag("notification");
        let is_response = arguments.has_positional_flag("response");

        match (is_request, is_notification, is_response) {
            (true, false, false) => Ok(Self::Request {
                cardinality: cardinality(arguments, message)?,
                method: method(arguments, message)?,
            }),
            (false, true, false) => {
                reject_cardinality(arguments, message)?;

                Ok(Self::Notification {
                    method: method(arguments, message)?,
                })
            }
            (false, false, true) => {
                reject_cardinality(arguments, message)?;

                Ok(Self::Response {
                    method: method(arguments, message)?,
                })
            }
            (false, false, false) => Err(WebSocketCodegenError::MissingMessageKind {
                message: message.to_string(),
            }),
            _ => Err(WebSocketCodegenError::InvalidMessageKind {
                message: message.to_string(),
            }),
        }
    }
}

fn cardinality(
    arguments: &AttributeArgs,
    message: &str,
) -> Result<MessageCardinality, WebSocketCodegenError> {
    match arguments.named("response") {
        None => Err(WebSocketCodegenError::MissingCardinality {
            message: message.to_string(),
        }),
        Some(_) => match arguments.path("response")? {
            Some(path) if path.is_ident("single") => Ok(MessageCardinality::Single),
            Some(path) if path.is_ident("stream") => Ok(MessageCardinality::Stream),
            _ => Err(WebSocketCodegenError::InvalidCardinality {
                message: message.to_string(),
            }),
        },
    }
}

fn method(arguments: &AttributeArgs, message: &str) -> Result<String, WebSocketCodegenError> {
    let method =
        arguments
            .string("method")?
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
    arguments: &AttributeArgs,
    message: &str,
) -> Result<(), WebSocketCodegenError> {
    if arguments.named("response").is_some() {
        return Err(WebSocketCodegenError::CardinalityOnNonRequest {
            message: message.to_string(),
        });
    }

    Ok(())
}
