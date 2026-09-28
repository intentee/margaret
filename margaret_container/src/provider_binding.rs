use crate::framework_injection_role::FrameworkInjectionRole;

pub struct ProviderBinding {
    pub field_name: String,
    pub injection: FrameworkInjectionRole,
    pub type_name: String,
}
