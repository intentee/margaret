use margaret_attributes::attribute_index::AttributeIndex;
use margaret_console_argument_codegen::console_argument_registry::ConsoleArgumentRegistry;

use crate::build_plan::build_plan;
use crate::container_bindings::ContainerBindings;
use crate::container_error::ContainerError;
use crate::framework_provider::FrameworkProvider;
use crate::planned_container::PlannedContainer;

pub fn plan_container(
    index: &AttributeIndex,
    registry: &ConsoleArgumentRegistry,
    framework_providers: &[FrameworkProvider],
) -> Result<PlannedContainer, ContainerError> {
    let plan = build_plan(index, registry, framework_providers)?;
    let bindings = ContainerBindings::from_plan(&plan);

    Ok(PlannedContainer::new(bindings, plan))
}
