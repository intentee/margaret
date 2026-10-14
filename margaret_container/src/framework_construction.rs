use margaret_attributes::canonical_path::CanonicalPath;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;

use crate::constructor_outcome::ConstructorOutcome;
use crate::framework_dependency::FrameworkDependency;
use crate::preparation_source::PreparationSource;

pub enum FrameworkConstruction {
    Accessor {
        accessor: String,
        source: CanonicalPath,
    },
    Constructor {
        dependencies: Vec<FrameworkDependency>,
        is_async: bool,
        method: String,
        outcome: ConstructorOutcome,
    },
    EnvironmentVariable {
        name: EnvironmentVariableName,
    },
    Prepared {
        method: String,
        sources: Vec<PreparationSource>,
    },
    Unit,
}
