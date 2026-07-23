use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::handler_binding::HandlerBinding;
use crate::session_plan::SessionPlan;

pub(crate) fn handler_console_arguments(
    handlers: &[HandlerBinding],
    bindings: &ContainerBindings,
) -> Vec<ConsoleArgument> {
    let mut collected: Vec<ConsoleArgument> = Vec::new();

    for handler in handlers {
        collected.extend_from_slice(bindings.console_arguments(&handler.handler_path));
    }

    collected
}

pub(crate) fn dispatch_table_console_arguments(
    plan: &SessionPlan,
    bindings: &ContainerBindings,
) -> Vec<ConsoleArgument> {
    let mut collected = handler_console_arguments(&plan.request_handlers, bindings);

    collected.extend(handler_console_arguments(
        &plan.notification_handlers,
        bindings,
    ));

    bindings.console_union(&collected)
}

pub(crate) fn session_console_arguments(
    plan: &SessionPlan,
    bindings: &ContainerBindings,
) -> Vec<ConsoleArgument> {
    let mut collected: Vec<ConsoleArgument> = Vec::new();

    for parameter in &plan.session.parameters {
        match &parameter.binding {
            RequestBinding::Injectable { dependency } => {
                for accessor_arguments in bindings.injected_console_arguments(dependency) {
                    collected.extend(accessor_arguments);
                }
            }
            RequestBinding::Bound {
                binder_provider, ..
            } => {
                collected.extend_from_slice(bindings.console_arguments(binder_provider));
            }
            _ => {}
        }
    }

    collected.extend(handler_console_arguments(&plan.request_handlers, bindings));
    collected.extend(handler_console_arguments(
        &plan.notification_handlers,
        bindings,
    ));

    bindings.console_union(&collected)
}
