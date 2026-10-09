#[derive(Debug, Eq, PartialEq)]
pub enum Removal<Modeled> {
    Missing,
    Removed(Modeled),
}
