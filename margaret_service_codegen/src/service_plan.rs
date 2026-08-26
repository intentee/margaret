use proc_macro2::TokenStream;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::serve_input_binding::ServeInputBinding;
use margaret_serve_input_codegen::has_spiffe_http_client::has_spiffe_http_client;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::framework_service::FrameworkService;
use crate::serve_inputs::ServeInputs;
use crate::service_codegen_error::ServiceCodegenError;
use crate::service_unit::ServiceUnit;
use crate::service_units::service_units;

pub struct ServicePlan {
    pub(crate) construction_arguments: Vec<ServeInputBinding>,
    pub(crate) has_spiffe_http_client: bool,
    pub(crate) reads_clap_matches: bool,
    pub(crate) prelude: TokenStream,
    pub(crate) units: Vec<ServiceUnit>,
    roots: Vec<CanonicalPath>,
}

impl ServicePlan {
    /// # Errors
    ///
    /// Returns `ServiceCodegenError` propagated from the work it performs.
    pub fn build(
        index: &AttributeIndex,
        framework_services: &[FrameworkService],
        bindings: &ContainerBindings,
        serve_inputs: &[ServeInput],
    ) -> Result<Self, ServiceCodegenError> {
        let mut units = service_units(index)?;
        units.extend(framework_services.iter().map(ServiceUnit::from_framework));
        let roots = units
            .iter()
            .map(|unit| unit.concrete_path.clone())
            .collect();
        let ServeInputs {
            construction_arguments,
            prelude,
        } = ServeInputs::resolve(serve_inputs, bindings)?;

        Ok(Self {
            construction_arguments,
            has_spiffe_http_client: has_spiffe_http_client(serve_inputs),
            prelude,
            reads_clap_matches: serve_inputs.iter().any(ServeInput::reads_clap_matches),
            roots,
            units,
        })
    }

    #[must_use]
    pub fn roots(&self) -> &[CanonicalPath] {
        &self.roots
    }
}
