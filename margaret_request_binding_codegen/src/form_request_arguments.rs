use quote::ToTokens;

use margaret_attributes::attribute_args::AttributeArgs;

use crate::request_binding_error::RequestBindingError;
use crate::request_input_source::RequestInputSource;

pub(crate) struct FormRequestArguments {
    pub(crate) source: RequestInputSource,
}

impl FormRequestArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        subject: &str,
        position: usize,
    ) -> Result<Self, RequestBindingError> {
        arguments.interpret(|reader| {
            let from = reader.take_path("from")?.ok_or_else(|| {
                RequestBindingError::FormRequestMissingSource {
                    subject: subject.to_string(),
                    parameter: position.to_string(),
                }
            })?;
            let source = RequestInputSource::from_path(&from).ok_or_else(|| {
                RequestBindingError::UnknownRequestInput {
                    subject: subject.to_string(),
                    parameter: position.to_string(),
                    written: from.to_token_stream().to_string(),
                }
            })?;

            Ok(Self { source })
        })
    }
}
