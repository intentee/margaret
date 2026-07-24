use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;

use crate::jwks_client_verifier_path::jwks_client_verifier_path;
use crate::service_codegen_error::ServiceCodegenError;

pub struct JwksClientEndpoint {
    pub console_arguments: Vec<ConsoleArgument>,
    pub field_name: String,
    pub path: CanonicalPath,
}

impl JwksClientEndpoint {
    pub fn resolve(
        index: &AttributeIndex,
        bindings: &ContainerBindings,
    ) -> Result<Option<Self>, ServiceCodegenError> {
        if !bindings.provides(&jwks_client_verifier_path()) {
            return Ok(None);
        }

        let mut endpoints: Vec<Self> = index
            .select(&AttributeSelector::from_marker("provides_endpoint"))
            .into_iter()
            .filter_map(|matched| {
                let path = matched.item().canonical_path().clone();

                bindings.provider_field(&path).map(|field_name| Self {
                    console_arguments: bindings.console_arguments(&path).to_vec(),
                    field_name: field_name.to_string(),
                    path,
                })
            })
            .collect();

        match endpoints.len() {
            0 => Err(ServiceCodegenError::JwksClientRequiresEndpoint),
            1 => Ok(endpoints.pop()),
            _ => Err(ServiceCodegenError::JwksClientAmbiguousEndpoint {
                endpoints: endpoints
                    .iter()
                    .map(|endpoint| endpoint.path.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            }),
        }
    }
}
