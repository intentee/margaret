pub(crate) struct JwksArtifacts {
    pub(crate) enabled: bool,
    pub(crate) modules:
        Vec<margaret_generated_module::generated_module_tokens::GeneratedModuleTokens>,
    pub(crate) services: Vec<margaret_service_codegen::framework_service::FrameworkService>,
}
