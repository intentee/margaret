use quote::ToTokens;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;

use crate::constants::REQUEST_INPUTS;
use crate::request_binding_error::RequestBindingError;
use crate::request_input_source::RequestInputSource;

pub(crate) struct FormRequestArguments {
    pub(crate) source: RequestInputSource,
}

impl FormRequestArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        index: &AttributeIndex,
        module_path: &[String],
        subject: &str,
        position: usize,
    ) -> Result<Self, RequestBindingError> {
        arguments.interpret(|reader| {
            let from = reader
                .take_path(ItemNamingArgument::FormRequestSource.key())?
                .ok_or_else(|| RequestBindingError::FormRequestMissingSource {
                    subject: subject.to_string(),
                    parameter: position.to_string(),
                })?;
            let source = index
                .resolve_module_path(module_path, &from)
                .as_ref()
                .and_then(|resolved| REQUEST_INPUTS.variant(resolved))
                .ok_or_else(|| RequestBindingError::UnknownRequestInput {
                    subject: subject.to_string(),
                    parameter: position.to_string(),
                    written: from.to_token_stream().to_string(),
                })?;

            Ok(Self { source })
        })
    }
}
