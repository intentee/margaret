use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;

use crate::request_binding_error::RequestBindingError;

pub(crate) struct InfersAuthenticatedUserArguments {
    pub(crate) user_model: Path,
}

impl InfersAuthenticatedUserArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        provider: &str,
    ) -> Result<Self, RequestBindingError> {
        arguments.interpret(|reader| {
            let user_model = reader
                .take_path(ItemNamingArgument::UserModel.key())?
                .ok_or_else(
                    || RequestBindingError::AuthenticatedUserProviderMissingUserModel {
                        provider: provider.to_string(),
                    },
                )?;

            Ok(Self { user_model })
        })
    }
}
