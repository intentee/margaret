use margaret_container::synthetic_provider::SyntheticProvider;

pub struct SecurityArtifacts {
    pub module_source: String,
    pub providers: Vec<SyntheticProvider>,
}
