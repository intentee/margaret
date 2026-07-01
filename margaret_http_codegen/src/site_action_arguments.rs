use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;

use crate::http_codegen_error::HttpCodegenError;

pub(crate) struct SiteActionArguments {
    pub(crate) action: Path,
}

impl SiteActionArguments {
    pub(crate) fn parse(arguments: &AttributeArgs, gate: &str) -> Result<Self, HttpCodegenError> {
        let action = arguments
            .positional_path(0)
            .ok_or_else(|| HttpCodegenError::MissingSiteActionArgument {
                gate: gate.to_string(),
            })?
            .clone();

        Ok(Self { action })
    }
}
