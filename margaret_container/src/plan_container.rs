use margaret_attributes::attribute_index::AttributeIndex;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_serve_input_codegen::declared_serve_inputs::DeclaredServeInputs;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

use crate::build_plan::build_plan;
use crate::container_bindings::ContainerBindings;
use crate::container_error::ContainerError;
use crate::framework_provider::FrameworkProvider;
use crate::planned_container::PlannedContainer;

/// # Errors
///
/// Returns `ContainerError` propagated from the work it performs.
pub fn plan_container(
    index: &AttributeIndex,
    serve_inputs: &DeclaredServeInputs,
    framework_providers: &[FrameworkProvider],
    database: &DeclaredPostgresDatabase,
    token_issuance: &DeclaredTokenIssuance,
) -> Result<PlannedContainer, ContainerError> {
    let plan = build_plan(
        index,
        serve_inputs,
        framework_providers,
        database,
        token_issuance,
    )?;
    let bindings = ContainerBindings::from_plan(&plan);

    Ok(PlannedContainer::new(bindings, plan))
}
