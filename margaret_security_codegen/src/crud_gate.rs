use margaret_attributes::canonical_path::CanonicalPath;

use crate::actor_requirement::ActorRequirement;

pub(crate) struct CrudGate {
    pub(crate) actor_requirement: ActorRequirement,
    pub(crate) gate_path: CanonicalPath,
    pub(crate) subject_type: CanonicalPath,
}
