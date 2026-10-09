use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::role_roots::RoleRoots;

pub(crate) struct ServedModules {
    pub(crate) modules: Vec<GeneratedModuleTokens>,
    pub(crate) roots: RoleRoots,
}
