use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;

#[derive(Debug, Eq, PartialEq)]
pub enum DeclaredSessionCookies {
    HostOnly,
    SharedWithDomain {
        domain_from: EnvironmentVariableName,
    },
}
