use margaret_attributes::canonical_path::CanonicalPath;

use crate::framework_dependency::FrameworkDependency;

pub enum FrameworkConstruction {
    Accessor {
        accessor: String,
        source: CanonicalPath,
    },
    Constructor {
        dependencies: Vec<FrameworkDependency>,
        is_async: bool,
        method: String,
    },
    UriSelected {
        argument_name: String,
        resolver: CanonicalPath,
        value_type: CanonicalPath,
    },
    Unit,
}
