use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;

use crate::request_binding_error::RequestBindingError;

pub(crate) struct InfersAuthenticatedUserArguments {
    pub(crate) login_route: Path,
    pub(crate) user_model: Path,
}

impl InfersAuthenticatedUserArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        provider: &str,
    ) -> Result<Self, RequestBindingError> {
        arguments.interpret(|reader| {
            let login_route = reader.take_path("login_route")?.ok_or_else(|| {
                RequestBindingError::AuthenticatedUserProviderMissingLoginRoute {
                    provider: provider.to_string(),
                }
            })?;
            let user_model = reader.take_path("user_model")?.ok_or_else(|| {
                RequestBindingError::AuthenticatedUserProviderMissingUserModel {
                    provider: provider.to_string(),
                }
            })?;

            Ok(Self {
                login_route,
                user_model,
            })
        })
    }
}
