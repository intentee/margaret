use margaret_attributes::canonical_path::CanonicalPath;

use crate::dependency_kind::DependencyKind;
use crate::parameter_plan::ParameterPlan;
use crate::provided_type::ProvidedType;

pub(crate) struct Provider {
    pub(crate) concrete_path: CanonicalPath,
    pub(crate) constructor_method: String,
    pub(crate) field_name: String,
    pub(crate) parameters: Vec<ParameterPlan>,
    pub(crate) provided: ProvidedType,
}

impl Provider {
    pub(crate) fn is_parameterized(&self) -> bool {
        self.parameters
            .iter()
            .any(|parameter| matches!(parameter, ParameterPlan::Input(_)))
    }

    pub(crate) fn dependencies(&self) -> impl Iterator<Item = &DependencyKind> {
        self.parameters
            .iter()
            .filter_map(|parameter| match parameter {
                ParameterPlan::Dependency(dependency) => Some(dependency),
                ParameterPlan::Input(_) => None,
            })
    }
}
