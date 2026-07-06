use crate::curve::Curve;

pub struct GenerateKeypairParams {
    pub crv: Curve,
    pub kid: String,
}
