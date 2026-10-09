use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_oauth_vocabulary_codegen::declared_scopes::DeclaredScopes;
use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

use crate::accepted_client_declaration::AcceptedClientDeclaration;
use crate::accepted_clients_codegen_error::AcceptedClientsCodegenError;
use crate::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use crate::declared_client_keys::DeclaredClientKeys;
use crate::declared_confidential_client::DeclaredConfidentialClient;

fn publish_keys(
    published: &mut BTreeMap<String, String>,
    client: &AcceptedClientDeclaration,
) -> Result<(), AcceptedClientsCodegenError> {
    let DeclaredAcceptedAuthentication::PrivateKeyJwt(DeclaredConfidentialClient {
        keys: DeclaredClientKeys::Published { jwks_uri, .. },
        ..
    }) = &client.authentication
    else {
        return Ok(());
    };

    match published.entry(jwks_uri.as_str().to_string()) {
        Entry::Occupied(first) => Err(AcceptedClientsCodegenError::SharedClientJwksUri {
            first: first.get().clone(),
            jwks_uri: jwks_uri.as_str().to_string(),
            second: client.path(),
        }),
        Entry::Vacant(vacant) => {
            vacant.insert(client.path());

            Ok(())
        }
    }
}

pub struct DeclaredAcceptedClients<'index> {
    pub clients: Vec<AcceptedClientDeclaration<'index>>,
}

impl<'index> DeclaredAcceptedClients<'index> {
    /// # Errors
    ///
    /// Returns `AcceptedClientsCodegenError` when a declaration is malformed, when clients are
    /// admitted without a token issuance, or when the clients cannot be told apart.
    pub fn read(
        index: &'index AttributeIndex,
        token_issuance: &DeclaredTokenIssuance,
        resources: &DeclaredResourceIssuances,
        scopes: &DeclaredScopes,
    ) -> Result<Self, AcceptedClientsCodegenError> {
        let mut admitted: BTreeMap<String, AcceptedClientDeclaration<'index>> = BTreeMap::new();
        let mut published: BTreeMap<String, String> = BTreeMap::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::AdmitsOAuthClient) {
            let DeclaredTokenIssuance::Declared(issuance) = token_issuance else {
                return Err(AcceptedClientsCodegenError::AcceptedWithoutTokenIssuance {
                    anchor: matched.item().canonical_path().to_string(),
                });
            };
            let client = AcceptedClientDeclaration::read(index, &matched, resources, scopes)?;

            client.admitted_by(issuance)?;
            publish_keys(&mut published, &client)?;

            match admitted.entry(client.client_id.as_str().to_string()) {
                Entry::Occupied(first) => {
                    return Err(AcceptedClientsCodegenError::DuplicateClientId {
                        client_id: client.client_id.to_string(),
                        first: first.get().path(),
                        second: client.path(),
                    });
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(client);
                }
            }
        }

        let mut clients: Vec<AcceptedClientDeclaration<'index>> = admitted.into_values().collect();

        clients.sort_by(|left, right| {
            left.anchor
                .identifier
                .field()
                .cmp(right.anchor.identifier.field())
        });

        Ok(Self { clients })
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
    use margaret_oauth_vocabulary_codegen::declared_scopes::DeclaredScopes;
    use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use super::DeclaredAcceptedClients;
    use crate::accepted_clients_codegen_error::AcceptedClientsCodegenError;

    const ISSUANCE: &str = "#[issues_tokens(provider, issuer = \"https://issuer.example\")]\npub struct Issuer;\n#[issues_resource_tokens(artifacts, audience = \"artifacts\")]\npub struct Artifacts;\n#[issues_resource_tokens(reports, audience = \"reports\")]\npub struct Reports;\n";

    fn rejection(source: &str) -> AcceptedClientsCodegenError {
        let indexed = IndexedSource::new(source);
        let issuance = DeclaredTokenIssuance::read(&indexed.index).expect("the issuance is read");

        DeclaredAcceptedClients::read(
            &indexed.index,
            &issuance,
            &DeclaredResourceIssuances::read(&indexed.index, &issuance)
                .expect("the resources are read"),
            &DeclaredScopes::read(&indexed.index).expect("the scopes are read"),
        )
        .err()
        .expect("the clients are rejected")
    }

    #[test]
    fn orders_the_clients_by_their_anchors() {
        let indexed = IndexedSource::new(&format!(
            "{ISSUANCE}#[admits_oauth_client(zeta_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"zeta\", resources = [artifacts])]\npub struct Alpha;\n#[admits_oauth_client(alpha_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"alpha\", resources = [artifacts])]\npub struct Zeta;\n"
        ));
        let issuance = DeclaredTokenIssuance::read(&indexed.index).expect("the issuance is read");
        let accepted = DeclaredAcceptedClients::read(
            &indexed.index,
            &issuance,
            &DeclaredResourceIssuances::read(&indexed.index, &issuance)
                .expect("the resources are read"),
            &DeclaredScopes::read(&indexed.index).expect("the scopes are read"),
        )
        .expect("the clients are read");

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
                "#[admits_oauth_client(spa_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"spa\", resources = [artifacts])]\npub struct Spa;\n"
            ),
            AcceptedClientsCodegenError::AcceptedWithoutTokenIssuance { anchor } if anchor == "crate::Spa"
        ));
    }

    #[test]
    fn rejects_a_client_accepted_twice() {
        assert!(matches!(
            rejection(&format!(
                "{ISSUANCE}#[admits_oauth_client(spa_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"spa\", resources = [artifacts])]\npub struct First;\n#[admits_oauth_client(spa_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"spa\", resources = [reports])]\npub struct Second;\n"
            )),
            AcceptedClientsCodegenError::DuplicateClientId { client_id, first, second }
                if client_id == "spa" && first == "crate::First" && second == "crate::Second"
        ));
    }

    #[test]
    fn rejects_clients_sharing_a_jwks_uri() {
        assert!(matches!(
            rejection(&format!(
                "{ISSUANCE}#[admits_oauth_client(first_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt(keys = margaret::framework::accepted_clients::client_keys::ClientKeys::Published(jwks_uri = \"https://keys.example/jwks.json\", signing = margaret::framework::jose_parameters::jws_algorithm::JwsAlgorithm::Es256)), client_id = \"first\", resources = [artifacts])]\npub struct First;\n#[admits_oauth_client(second_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt(keys = margaret::framework::accepted_clients::client_keys::ClientKeys::Published(jwks_uri = \"https://KEYS.example/jwks.json\", signing = margaret::framework::jose_parameters::jws_algorithm::JwsAlgorithm::Es256)), client_id = \"second\", resources = [artifacts])]\npub struct Second;\n"
            )),
            AcceptedClientsCodegenError::SharedClientJwksUri { jwks_uri, .. }
                if jwks_uri == "https://keys.example/jwks.json"
        ));
    }

    #[test]
    fn rejects_a_client_anchored_by_a_singleton() {
        assert!(matches!(
            rejection(&format!(
                "{ISSUANCE}#[singleton]\n#[admits_oauth_client(spa_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"spa\", resources = [artifacts])]\npub struct Spa;\n"
            )),
            AcceptedClientsCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. })
                if path == "crate::Spa"
        ));
    }
}
