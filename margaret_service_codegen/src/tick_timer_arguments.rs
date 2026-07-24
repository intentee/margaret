use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;

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
            let behavior = reader.take_path("behavior")?;
            let interval = reader.take_path("interval")?.ok_or_else(|| {
                ServiceCodegenError::TickerMissingInterval {
                    ticker: ticker.to_string(),
                }
            })?;

            Ok(Self { behavior, interval })
        })
    }
}
