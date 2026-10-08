#[derive(Debug, Eq, PartialEq)]
pub enum Creation<Modeled> {
    Created(Modeled),
    Refused,
}
