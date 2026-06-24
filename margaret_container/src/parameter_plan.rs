use crate::dependency_kind::DependencyKind;
use crate::input_parameter::InputParameter;

pub(crate) enum ParameterPlan {
    Dependency(DependencyKind),
    Input(Box<InputParameter>),
}
