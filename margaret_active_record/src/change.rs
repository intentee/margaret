#[derive(Debug, Eq, PartialEq)]
pub enum Change<Modeled> {
    Changed(Modeled),
    Unmatched,
}
