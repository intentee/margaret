use margaret_container::injected_dependency::InjectedDependency;

use crate::framework_input::FrameworkInput;

pub struct FrameworkResponder {
    pub handler: InjectedDependency,
    pub input: FrameworkInput,
}
