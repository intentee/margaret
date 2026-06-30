use crate::responder_output::ResponderOutput;
use crate::route_parameter::RouteParameter;

pub(crate) struct ResponderSignature {
    pub(crate) output: ResponderOutput,
    pub(crate) parameters: Vec<RouteParameter>,
}
