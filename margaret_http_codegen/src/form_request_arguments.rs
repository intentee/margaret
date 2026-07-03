use quote::ToTokens;

use margaret_attributes::attribute_args::AttributeArgs;

use crate::http_codegen_error::HttpCodegenError;
use crate::request_input_source::RequestInputSource;

pub(crate) struct FormRequestArguments {
    pub(crate) source: RequestInputSource,
}

impl FormRequestArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        responder: &str,
        position: usize,
    ) -> Result<Self, HttpCodegenError> {
        let from =
            arguments
                .path("from")?
                .ok_or_else(|| HttpCodegenError::FormRequestMissingSource {
                    responder: responder.to_string(),
                    parameter: position.to_string(),
                })?;
        let source = RequestInputSource::from_path(&from).ok_or_else(|| {
            HttpCodegenError::UnknownRequestInput {
                responder: responder.to_string(),
                parameter: position.to_string(),
                written: from.to_token_stream().to_string(),
            }
        })?;

        Ok(Self { source })
    }
}
