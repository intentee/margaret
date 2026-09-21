use std::num::ParseIntError;

use margaret::framework::macros::route_parameter_value;

#[route_parameter_value]
pub struct ProjectRevision(pub u32);

impl TryFrom<String> for ProjectRevision {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, ParseIntError> {
        value.parse().map(Self)
    }
}
