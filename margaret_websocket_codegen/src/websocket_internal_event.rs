use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Debug)]
pub struct WebsocketInternalEvent {
    pub canonical_path: CanonicalPath,
    pub variant: String,
}
