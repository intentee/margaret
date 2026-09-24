use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::web_socket_server_requirements::WebSocketServerRequirements;

#[derive(Debug)]
pub struct WebSocketArtifacts {
    pub modules: Vec<GeneratedModuleTokens>,
    pub retained_roots: Vec<CanonicalPath>,
    pub servers: BTreeMap<String, WebSocketServerRequirements>,
}
