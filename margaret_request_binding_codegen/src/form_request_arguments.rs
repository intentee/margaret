use quote::ToTokens;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;

use crate::request_binding_error::RequestBindingError;
use crate::request_input_source::RequestInputSource;

pub(crate) struct FormRequestArguments {
    pub(crate) source: RequestInputSource,
}

impl FormRequestArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        index: &AttributeIndex,
        item: &IndexedItem,
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
                .resolve_item_path(item, &from)
                .as_ref()
                .and_then(RequestInputSource::from_canonical)
                .ok_or_else(|| RequestBindingError::UnknownRequestInput {
                    subject: subject.to_string(),
                    parameter: position.to_string(),
                    written: from.to_token_stream().to_string(),
                })?;

            Ok(Self { source })
        })
    }
}
