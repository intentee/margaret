use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::container_bindings::ContainerBindings;

pub struct RenderedContainer {
    pub bindings: ContainerBindings,
    pub modules: Vec<GeneratedModuleTokens>,
}
