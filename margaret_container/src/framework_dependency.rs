use margaret_attributes::canonical_path::CanonicalPath;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;

pub enum FrameworkDependency {
    Constant(CanonicalPath),
    EnvironmentVariable {
        name: EnvironmentVariableName,
        value_type: CanonicalPath,
    },
    Provider(CanonicalPath),
    Providers(Vec<CanonicalPath>),
    SingletonView(CanonicalPath),
    TokenIssuance,
}
