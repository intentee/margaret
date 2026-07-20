use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct HandlerBinding {
    pub(crate) handler_path: CanonicalPath,
    pub(crate) method: String,
}
