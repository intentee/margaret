use margaret_attributes::canonical_path::CanonicalPath;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
use margaret_serve_input_codegen::route_url_input::RouteUrlInput;

use crate::framework_preparation::FrameworkPreparation;
use crate::url_source::UrlSource;

pub enum FrameworkDependency {
    Constant(CanonicalPath),
    Database {
        framework_tables: Vec<CanonicalPath>,
    },
    EnvironmentVariable {
        name: EnvironmentVariableName,
        value_type: CanonicalPath,
    },
    Provider(CanonicalPath),
    Prepared(FrameworkPreparation),
    Providers(Vec<CanonicalPath>),
    RouteUrl(RouteUrlInput),
    Routes,
    SingletonView(CanonicalPath),
    SpiffeHttpClient,
    SpiffeIssuerRequestClient,
    TokenIssuance,
    Urls(Vec<UrlSource>),
}
