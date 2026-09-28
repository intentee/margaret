use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;

use crate::service_codegen_error::ServiceCodegenError;

pub(crate) struct TickTimerArguments {
    pub(crate) behavior: Option<Path>,
    pub(crate) interval: Path,
}

impl TickTimerArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        ticker: &str,
    ) -> Result<Self, ServiceCodegenError> {
        arguments.interpret(|reader| {
            let behavior = reader.take_path(ItemNamingArgument::TickBehavior.key())?;
            let interval = reader
                .take_path(ItemNamingArgument::TickInterval.key())?
                .ok_or_else(|| ServiceCodegenError::TickerMissingInterval {
                    ticker: ticker.to_string(),
                })?;

            Ok(Self { behavior, interval })
        })
    }
}
