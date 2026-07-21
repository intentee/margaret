use crate::margaret::routes::Routes;

pub struct GreetingViewProps<'routes> {
    pub greeting: String,
    pub routes: &'routes Routes,
}
