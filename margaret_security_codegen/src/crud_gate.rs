use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct CrudGate {
    pub(crate) gate_path: CanonicalPath,
    pub(crate) subject_type: CanonicalPath,
}
