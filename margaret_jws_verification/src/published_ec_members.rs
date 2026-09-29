use serde::Deserialize;

use margaret_jose_parameters::curve::Curve;

use crate::parameter_value::ParameterValue;

#[derive(Deserialize)]
pub(crate) struct PublishedEcMembers {
    pub(crate) crv: ParameterValue<Curve>,
    pub(crate) x: String,
    pub(crate) y: String,
}
