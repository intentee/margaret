use syn::Path;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::actor_requirement::ActorRequirement;

pub(crate) struct SiteGate {
    pub(crate) action_path: Path,
    pub(crate) actor_requirement: ActorRequirement,
    pub(crate) gate_path: CanonicalPath,
}
