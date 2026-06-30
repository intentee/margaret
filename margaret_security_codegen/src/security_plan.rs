use syn::Path;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::crud_gate::CrudGate;
use crate::site_gate::SiteGate;

pub(crate) struct SecurityPlan {
    pub(crate) action_type: Option<Path>,
    pub(crate) crud_gates: Vec<CrudGate>,
    pub(crate) site_gates: Vec<SiteGate>,
    pub(crate) store_path: CanonicalPath,
    pub(crate) actor_type: CanonicalPath,
}
