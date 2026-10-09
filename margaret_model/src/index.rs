#[derive(Debug, Eq, PartialEq)]
pub struct Index {
    pub columns: &'static [&'static str],
    pub name: &'static str,
}
