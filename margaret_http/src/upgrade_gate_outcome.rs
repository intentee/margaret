use crate::request::Request;
use crate::response::Response;

pub(crate) enum UpgradeGateOutcome {
    Proceed(Box<Request>),
    ShortCircuit(Response),
}
