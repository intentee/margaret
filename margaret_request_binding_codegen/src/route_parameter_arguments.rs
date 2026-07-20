use margaret_attributes::attribute_args::AttributeArgs;

use crate::request_binding_error::RequestBindingError;

pub(crate) struct RouteParameterArguments {
    pub(crate) from: String,
}

impl RouteParameterArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        subject: &str,
        position: usize,
    ) -> Result<Self, RequestBindingError> {
        let from = arguments.string("from")?.ok_or_else(|| {
            RequestBindingError::RouteParameterMissingFrom {
                subject: subject.to_string(),
                parameter: position.to_string(),
            }
        })?;

        Ok(Self { from })
    }
}
