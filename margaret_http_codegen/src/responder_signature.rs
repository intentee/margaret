use crate::responder_argument::ResponderArgument;
use crate::responder_output::ResponderOutput;

pub(crate) struct ResponderSignature {
    pub(crate) arguments: Vec<ResponderArgument>,
    pub(crate) output: ResponderOutput,
}
