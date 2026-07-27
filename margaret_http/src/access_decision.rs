use crate::response::Response;

pub enum AccessDecision {
    Allowed,
    Denied(Response),
}
