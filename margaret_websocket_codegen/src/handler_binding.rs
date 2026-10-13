use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct HandlerBinding {
    pub(crate) dispatch_ident: String,
    pub(crate) handler_field: String,
    pub(crate) handler_path: CanonicalPath,
    pub(crate) message_path: CanonicalPath,
    pub(crate) method: String,
}
