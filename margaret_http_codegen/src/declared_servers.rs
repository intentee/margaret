use std::collections::HashMap;

use heck::ToSnakeCase;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::declared_server::DeclaredServer;
use crate::http_codegen_error::HttpCodegenError;

pub(crate) fn declared_servers(
    index: &AttributeIndex,
) -> Result<Vec<DeclaredServer>, HttpCodegenError> {
    let selector = AttributeSelector::parse("http_server").expect("a valid selector");
    let mut servers: Vec<DeclaredServer> = Vec::new();
    let mut seen: HashMap<String, String> = HashMap::new();

    for matched in index.select(&selector) {
        let item = matched.item();
        let declaration = item.canonical_path().to_string();

        if !item.kind().is_struct() {
            return Err(HttpCodegenError::HttpServerNotOnStruct {
                target: declaration,
            });
        }

        let name = matched
            .args()?
            .string("name")?
            .ok_or_else(|| HttpCodegenError::MissingHttpServerName {
                marker: declaration.clone(),
            })?
            .to_snake_case();

        if let Some(first) = seen.get(&name) {
            return Err(HttpCodegenError::DuplicateHttpServer {
                name,
                first: first.clone(),
                second: declaration,
            });
        }

        seen.insert(name.clone(), declaration);
        servers.push(DeclaredServer::new(name, item.canonical_path().clone()));
    }

    Ok(servers)
}
