use margaret_container::injected_dependency::InjectedDependency;

use crate::route_content::RouteContent;

pub(crate) struct FrameworkRoute {
    pub(crate) content: RouteContent<()>,
    pub(crate) handler: InjectedDependency,
}
