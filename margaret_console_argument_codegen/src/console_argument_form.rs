#[derive(Debug, Eq, PartialEq)]
pub enum ConsoleArgumentForm {
    Named { key: String },
    Positional,
}
