use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

pub struct HttpArtifacts {
    pub retained_roots: Vec<CanonicalPath>,
    pub modules: Vec<GeneratedModuleTokens>,
}

impl HttpArtifacts {
    pub(crate) fn new(
        modules: Vec<GeneratedModuleTokens>,
        retained_roots: Vec<CanonicalPath>,
    ) -> Self {
        Self {
            retained_roots,
            modules,
        }
    }

    #[must_use]
    pub fn into_modules(self) -> Vec<GeneratedModuleTokens> {
        self.modules
    }
}
