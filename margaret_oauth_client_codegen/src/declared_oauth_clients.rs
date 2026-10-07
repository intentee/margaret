use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::oauth_client_codegen_error::OAuthClientCodegenError;
use crate::oauth_client_declaration::OAuthClientDeclaration;

pub struct DeclaredOAuthClients<'index> {
    pub clients: Vec<OAuthClientDeclaration<'index>>,
}

impl<'index> DeclaredOAuthClients<'index> {
    /// # Errors
    ///
    /// Returns `OAuthClientCodegenError` when a declaration is malformed.
    pub fn read(index: &'index AttributeIndex) -> Result<Self, OAuthClientCodegenError> {
        let mut clients = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::OAuthClient) {
            clients.push(OAuthClientDeclaration::read(index, &matched)?);
        }

        clients.sort_by_key(|client| client.tag.to_string());

        Ok(Self { clients })
    }
}
