#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrudAction {
    Delete,
    Read,
    Update,
}
