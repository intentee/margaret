#[derive(Debug, Eq, PartialEq)]
pub enum Lookup<Found> {
    Found(Found),
    Missing,
}
