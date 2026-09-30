use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;

pub(crate) struct OAuthClientArguments {
    pub(crate) issuer: Option<Path>,
    pub(crate) tag: Option<Path>,
}

impl OAuthClientArguments {
    pub(crate) fn read(args: &AttributeArgs) -> Result<Self, AttributeArgumentsError> {
        args.interpret(|reader| {
            Ok(Self {
                tag: reader.take_positional_path(),
                issuer: reader.take_path("issuer")?,
            })
        })
    }
}
