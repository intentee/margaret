use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::serve_input_binding::ServeInputBinding;
use margaret_container::slotted_serve_input::SlottedServeInput;
use margaret_serve_input_codegen::chained_read::ChainedRead;
use margaret_serve_input_codegen::spiffe_client_requirements::SpiffeClientRequirements;

use crate::framework_service::FrameworkService;
use crate::serve_inputs::ServeInputs;
use crate::service_codegen_error::ServiceCodegenError;
use crate::service_unit::ServiceUnit;
use crate::service_units::service_units;

pub struct ServicePlan {
    pub(crate) construction_arguments: Vec<ServeInputBinding>,
    pub(crate) reads: Vec<ChainedRead>,
    pub(crate) reads_clap_matches: bool,
    pub(crate) units: Vec<ServiceUnit>,
    roots: Vec<CanonicalPath>,
    pub(crate) served_roots: Vec<CanonicalPath>,
    serve_inputs: Vec<SlottedServeInput>,
    pub(crate) spiffe_clients: SpiffeClientRequirements,
}

impl ServicePlan {
    /// # Errors
    ///
    /// Returns `ServiceCodegenError` propagated from the work it performs.
    pub fn build(
        index: &AttributeIndex,
        framework_services: &[FrameworkService],
        bindings: &ContainerBindings,
        serving_roots: &[CanonicalPath],
    ) -> Result<Self, ServiceCodegenError> {
        let mut units = service_units(index)?;
        units.extend(framework_services.iter().map(ServiceUnit::from_framework));
        let roots: Vec<CanonicalPath> = units
            .iter()
            .map(|unit| unit.concrete_path.clone())
            .collect();
        let served_roots: Vec<CanonicalPath> = roots.iter().chain(serving_roots).cloned().collect();
        let serve_inputs = bindings.serve_inputs(&served_roots)?;
        let ServeInputs {
            construction_arguments,
            reads,
        } = ServeInputs::resolve(&serve_inputs, bindings);

        Ok(Self {
            construction_arguments,
            reads,
            reads_clap_matches: serve_inputs
                .iter()
                .any(|slotted| slotted.input.reads_clap_matches()),
            roots,
            served_roots,
            spiffe_clients: SpiffeClientRequirements::of(
                serve_inputs.iter().map(|slotted| &slotted.input),
            ),
            serve_inputs,
            units,
        })
    }

    #[must_use]
    pub fn roots(&self) -> &[CanonicalPath] {
        &self.roots
    }

    #[must_use]
    pub fn serve_inputs(&self) -> &[SlottedServeInput] {
        &self.serve_inputs
    }
}
