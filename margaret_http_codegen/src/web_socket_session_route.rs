use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Debug)]
pub struct WebSocketSessionRoute {
    pub path: String,
    pub session: CanonicalPath,
}
