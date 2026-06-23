#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ItemKind {
    Enum,
    Function,
    Module,
    Struct,
    Trait,
}
