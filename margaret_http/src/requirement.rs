use crate::response_continuation::ResponseContinuation;

pub enum Requirement<Value> {
    Met(Value),
    Unmet(ResponseContinuation),
}
