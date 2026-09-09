use crate::bound_parameter::BoundParameter;
use crate::request_body_intake::RequestBodyIntake;

pub struct ClassifiedParameters {
    pub body_intake: RequestBodyIntake,
    pub parameters: Vec<BoundParameter>,
}
