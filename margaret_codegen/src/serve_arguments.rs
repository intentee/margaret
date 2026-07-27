use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;

pub(crate) fn serve_roots(index: &AttributeIndex) -> Vec<CanonicalPath> {
    let roles = [
        FrameworkAttribute::Service,
        FrameworkAttribute::ScheduledWithTickTimer,
    ];
    let mut roots = Vec::new();

    for role in roles {
        for matched in index.select_framework_attribute(role) {
            roots.push(matched.item().canonical_path().clone());
        }
    }

    roots
}

pub(crate) fn serve_arguments(bindings: &ContainerBindings) -> Vec<ConsoleArgument> {
    bindings.all_console_arguments().to_vec()
}
