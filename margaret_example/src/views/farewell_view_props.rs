use crate::margaret::routes::Routes;

pub struct FarewellViewProps<'routes> {
    pub name: String,
    pub routes: &'routes Routes,
}
