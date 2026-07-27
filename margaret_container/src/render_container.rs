use crate::container_error::ContainerError;
use crate::framework_provider::FrameworkProvider;
use crate::plan_container::plan_container;
use crate::rendered_container::RenderedContainer;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_console_argument_codegen::console_argument_registry::ConsoleArgumentRegistry;

pub fn render_container(
    index: &AttributeIndex,
    registry: &ConsoleArgumentRegistry,
    framework_providers: &[FrameworkProvider],
) -> Result<RenderedContainer, ContainerError> {
    let planned = plan_container(index, registry, framework_providers)?;
    let roots = planned.roots();

    planned.render(&roots, &roots, &roots)
}
