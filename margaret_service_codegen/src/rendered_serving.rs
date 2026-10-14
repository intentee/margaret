use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::serve_invocation_read::ServeInvocationRead;

pub struct RenderedServing {
    pub invocation: ServeInvocationRead,
    pub modules: Vec<GeneratedModuleTokens>,
}
