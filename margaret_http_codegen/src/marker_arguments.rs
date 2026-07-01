use syn::Expr;

use margaret_attributes::attribute_args::AttributeArgs;

use crate::http_codegen_error::HttpCodegenError;

pub(crate) struct MarkerArguments {
    pub(crate) value: Option<Expr>,
}

impl MarkerArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        marker: String,
        responder: &str,
    ) -> Result<Self, HttpCodegenError> {
        if arguments.is_empty() {
            return Ok(Self { value: None });
        }

        match arguments.positional(0) {
            Some(value) if arguments.positional(1).is_none() => Ok(Self {
                value: Some(value.clone()),
            }),
            _ => Err(HttpCodegenError::MalformedMarker {
                marker,
                responder: responder.to_string(),
            }),
        }
    }
}
