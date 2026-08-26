use margaret_input_weaving::input_value::InputValue;

use crate::environment_variable_name::EnvironmentVariableName;

#[derive(Clone, Debug, PartialEq)]
pub struct EnvironmentVariable {
    pub name: EnvironmentVariableName,
    pub value: InputValue,
}
