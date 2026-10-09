use crate::continuation::Continuation;

pub trait QueryEdge<Modeled> {
    type Continued: Continuation<Modeled>;
}
