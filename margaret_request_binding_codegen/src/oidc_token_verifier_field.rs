use margaret_container::injected_dependency::InjectedDependency;

#[derive(Clone)]
pub struct OidcTokenVerifierField {
    pub client: InjectedDependency,
    pub field: String,
}
