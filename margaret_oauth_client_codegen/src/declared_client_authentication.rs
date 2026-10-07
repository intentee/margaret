use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;

pub enum DeclaredClientAuthentication {
    ClientSecretBasic {
        client_secret_from: EnvironmentVariableName,
    },
    PrivateKeyJwt,
}
