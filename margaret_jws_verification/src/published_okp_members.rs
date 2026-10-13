use serde::Deserialize;

use margaret_jose_parameters::octet_key_pair_curve::OctetKeyPairCurve;

use crate::parameter_value::ParameterValue;

#[derive(Deserialize)]
pub(crate) struct PublishedOkpMembers {
    pub(crate) crv: ParameterValue<OctetKeyPairCurve>,
    pub(crate) x: String,
}
