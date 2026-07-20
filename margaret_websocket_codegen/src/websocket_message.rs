use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Debug)]
pub struct WebsocketMessage {
    pub canonical_path: CanonicalPath,
    pub method: String,
    pub variant: String,
}
