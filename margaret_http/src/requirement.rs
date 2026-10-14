use crate::response::Response;

pub enum Requirement<Value> {
    Met(Value),
    Unmet(Response),
}
