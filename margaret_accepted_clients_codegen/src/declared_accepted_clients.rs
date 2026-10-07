use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

use crate::accepted_client_declaration::AcceptedClientDeclaration;
use crate::accepted_clients_codegen_error::AcceptedClientsCodegenError;
use crate::declared_accepted_authentication::DeclaredAcceptedAuthentication;

fn unique_client_ids(
    clients: &[AcceptedClientDeclaration],
) -> Result<(), AcceptedClientsCodegenError> {
    let mut accepted: HashMap<&str, String> = HashMap::new();

    for client in clients {
        if let Some(first) = accepted.insert(client.client_id.as_str(), client.path()) {
            return Err(AcceptedClientsCodegenError::DuplicateClientId {
                client_id: client.client_id.to_string(),
                first,
                second: client.path(),
            });
        }
    }

    Ok(())
}

fn unique_jwks_uris(
    clients: &[AcceptedClientDeclaration],
) -> Result<(), AcceptedClientsCodegenError> {
    let mut published: HashMap<&str, String> = HashMap::new();

    for client in clients {
        if let DeclaredAcceptedAuthentication::PrivateKeyJwt(confidential) = &client.authentication
            && let Some(first) = published.insert(confidential.jwks_uri.as_str(), client.path())
        {
            return Err(AcceptedClientsCodegenError::SharedClientJwksUri {
                first,
                jwks_uri: confidential.jwks_uri.as_str().to_string(),
                second: client.path(),
            });
        }
    }

    Ok(())
}

pub struct DeclaredAcceptedClients<'index> {
    pub clients: Vec<AcceptedClientDeclaration<'index>>,
}

impl<'index> DeclaredAcceptedClients<'index> {
    /// # Errors
    ///
    /// Returns `AcceptedClientsCodegenError` when a declaration is malformed, when clients are
    /// accepted without a token issuance, or when the clients cannot be told apart.
    pub fn read(
        index: &'index AttributeIndex,
        token_issuance: &DeclaredTokenIssuance,
    ) -> Result<Self, AcceptedClientsCodegenError> {
        let mut clients = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::AcceptsOAuthClient) {
            let client = AcceptedClientDeclaration::read(index, &matched)?;
            let DeclaredTokenIssuance::Declared(issuance) = token_issuance else {
                return Err(AcceptedClientsCodegenError::AcceptedWithoutTokenIssuance {
                    anchor: client.path(),
                });
            };

            client.admitted_by(issuance)?;
            clients.push(client);
        }

        clients.sort_by(|left, right| {
            left.anchor
                .identifier
                .field()
                .cmp(right.anchor.identifier.field())
        });
        unique_client_ids(&clients)?;
        unique_jwks_uris(&clients)?;

        Ok(Self { clients })
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use super::DeclaredAcceptedClients;
    use crate::accepted_clients_codegen_error::AcceptedClientsCodegenError;

    const ISSUANCE: &str = "#[issues_tokens(audience = \"session\", issuer = \"https://issuer.example\")]\npub struct Issuer;\n";

    fn rejection(source: &str) -> AcceptedClientsCodegenError {
        let indexed = IndexedSource::new(source);
        let issuance = DeclaredTokenIssuance::read(&indexed.index).expect("the issuance is read");

        DeclaredAcceptedClients::read(&indexed.index, &issuance)
            .err()
            .expect("the clients are rejected")
    }

    #[test]
    fn orders_the_clients_by_their_anchors() {
        let indexed = IndexedSource::new(&format!(
            "{ISSUANCE}#[accepts_oauth_client(authentication = none, client_id = \"zeta\", resources = [\"artifacts\"])]\npub struct Alpha;\n#[accepts_oauth_client(authentication = none, client_id = \"alpha\", resources = [\"artifacts\"])]\npub struct Zeta;\n"
        ));
        let issuance = DeclaredTokenIssuance::read(&indexed.index).expect("the issuance is read");
        let accepted =
            DeclaredAcceptedClients::read(&indexed.index, &issuance).expect("the clients are read");

        assert_eq!(
            accepted
                .clients
                .iter()
                .map(|client| client.client_id.as_str())
                .collect::<Vec<&str>>(),
            vec!["zeta", "alpha"]
        );
    }

    #[test]
    fn rejects_a_client_accepted_without_a_token_issuance() {
        assert!(matches!(
            rejection(
                "#[accepts_oauth_client(authentication = none, client_id = \"spa\", resources = [\"artifacts\"])]\npub struct Spa;\n"
            ),
            AcceptedClientsCodegenError::AcceptedWithoutTokenIssuance { anchor } if anchor == "crate::Spa"
        ));
    }

    #[test]
    fn rejects_a_client_accepted_twice() {
        assert!(matches!(
            rejection(&format!(
                "{ISSUANCE}#[accepts_oauth_client(authentication = none, client_id = \"spa\", resources = [\"artifacts\"])]\npub struct First;\n#[accepts_oauth_client(authentication = none, client_id = \"spa\", resources = [\"reports\"])]\npub struct Second;\n"
            )),
            AcceptedClientsCodegenError::DuplicateClientId { client_id, first, second }
                if client_id == "spa" && first == "crate::First" && second == "crate::Second"
        ));
    }

    #[test]
    fn rejects_clients_sharing_a_jwks_uri() {
        assert!(matches!(
            rejection(&format!(
                "{ISSUANCE}#[accepts_oauth_client(authentication = private_key_jwt(jwks_uri = \"https://keys.example/jwks.json\"), client_id = \"first\", resources = [\"artifacts\"])]\npub struct First;\n#[accepts_oauth_client(authentication = private_key_jwt(jwks_uri = \"https://KEYS.example/jwks.json\"), client_id = \"second\", resources = [\"artifacts\"])]\npub struct Second;\n"
            )),
            AcceptedClientsCodegenError::SharedClientJwksUri { jwks_uri, .. }
                if jwks_uri == "https://keys.example/jwks.json"
        ));
    }

    #[test]
    fn rejects_a_client_anchored_by_a_singleton() {
        assert!(matches!(
            rejection(&format!(
                "{ISSUANCE}#[singleton]\n#[accepts_oauth_client(authentication = none, client_id = \"spa\", resources = [\"artifacts\"])]\npub struct Spa;\n"
            )),
            AcceptedClientsCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. })
                if path == "crate::Spa"
        ));
    }
}
