use crate::narrowed::Narrowed;

pub trait Continuation<Modeled>: Sized {
    fn continued(narrowed: Narrowed<Modeled>) -> Self;
}
