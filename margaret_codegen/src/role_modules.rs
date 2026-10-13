use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

pub(crate) struct RoleModules {
    pub(crate) console_roots: Vec<CanonicalPath>,
    pub(crate) modules: Vec<GeneratedModuleTokens>,
    pub(crate) service_roots: Vec<CanonicalPath>,
}
