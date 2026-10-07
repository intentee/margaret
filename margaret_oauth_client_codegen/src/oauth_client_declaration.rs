use proc_macro2::Ident;

use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::tag::Tag;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret_oauth_vocabulary::client_id::ClientId;

use crate::declared_client_authentication::DeclaredClientAuthentication;
use crate::oauth_client_codegen_error::OAuthClientCodegenError;

fn client_secret_source(
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<EnvironmentVariableName, OAuthClientCodegenError> {
    let name = reader.take_string("client_secret_from")?.ok_or_else(|| {
        OAuthClientCodegenError::MissingClientSecretSource {
            anchor: anchor.to_string(),
        }
    })?;

    EnvironmentVariableName::new(&name).ok_or_else(|| {
        OAuthClientCodegenError::MalformedClientSecretSource {
            anchor: anchor.to_string(),
            name,
        }
    })
}

fn declared_authentication(
    keyword: &Ident,
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<DeclaredClientAuthentication, OAuthClientCodegenError> {
    let method = keyword
        .to_string()
        .parse::<ClientAuthenticationMethod>()
        .map_err(|source| OAuthClientCodegenError::MalformedAuthentication {
            anchor: anchor.to_string(),
            source,
        })?;

    match method {
        ClientAuthenticationMethod::ClientSecretBasic => {
            Ok(DeclaredClientAuthentication::ClientSecretBasic {
                client_secret_from: client_secret_source(reader, anchor)?,
            })
        }
        ClientAuthenticationMethod::None => {
            Err(OAuthClientCodegenError::UnsupportedAuthentication {
                anchor: anchor.to_string(),
                method: method.wire_name(),
            })
        }
        ClientAuthenticationMethod::PrivateKeyJwt => {
            Ok(DeclaredClientAuthentication::PrivateKeyJwt)
        }
    }
}

pub struct OAuthClientDeclaration<'index> {
    pub anchor: &'index IndexedItem,
    pub authentication: DeclaredClientAuthentication,
    pub client_id: ClientId,
    pub issuer: Tag,
    pub tag: Tag,
}

impl<'index> OAuthClientDeclaration<'index> {
    pub(crate) fn read(
        index: &'index AttributeIndex,
        matched: &MatchedAttribute<'index>,
    ) -> Result<Self, OAuthClientCodegenError> {
        let anchor = declaration_anchor(index, matched, FrameworkAttribute::OAuthClient)?.item;
        let path = anchor.canonical_path().to_string();

        matched.args()?.interpret(|reader| {
            let tag = reader
                .take_positional_path()
                .ok_or_else(|| OAuthClientCodegenError::MissingTag {
                    anchor: path.clone(),
                })
                .and_then(|tag| {
                    Tag::from_path(&tag).ok_or_else(|| OAuthClientCodegenError::MalformedTag {
                        anchor: path.clone(),
                    })
                })?;
            let authentication = reader
                .take_keyword("authentication", |keyword, arguments| {
                    declared_authentication(keyword, arguments, &path)
                })?
                .ok_or_else(|| OAuthClientCodegenError::MissingAuthentication {
                    anchor: path.clone(),
                })?;
            let client_id = reader
                .take_string("client_id")?
                .ok_or_else(|| OAuthClientCodegenError::MissingClientId {
                    anchor: path.clone(),
                })?
                .parse::<ClientId>()
                .map_err(|source| OAuthClientCodegenError::MalformedClientId {
                    anchor: path.clone(),
                    source,
                })?;
            let issuer = reader
                .take_path("issuer")?
                .as_ref()
                .and_then(Tag::from_path)
                .ok_or_else(|| OAuthClientCodegenError::MalformedIssuer {
                    anchor: path.clone(),
                })?;

            Ok(Self {
                anchor,
                authentication,
                client_id,
                issuer,
                tag,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

    use crate::declared_client_authentication::DeclaredClientAuthentication;
    use crate::declared_oauth_clients::DeclaredOAuthClients;
    use crate::oauth_client_codegen_error::OAuthClientCodegenError;

    fn rejection(arguments: &str) -> OAuthClientCodegenError {
        let indexed = IndexedSource::new(&format!(
            "#[oauth_client({arguments})]\npub struct Client;\n"
        ));

        DeclaredOAuthClients::read(&indexed.index)
            .err()
            .expect("the declaration is rejected")
    }

    #[test]
    fn reads_a_client_that_authenticates_with_its_private_key() {
        let indexed = IndexedSource::new(
            "#[oauth_client(blog, authentication = private_key_jwt, client_id = \"blog\", issuer = margaret)]\npub struct BlogClient;\n",
        );
        let clients = DeclaredOAuthClients::read(&indexed.index).expect("the client is read");
        let client = clients.clients.first().expect("the client is declared");

        assert_eq!(clients.clients.len(), 1);
        assert_eq!(
            discriminant(&client.authentication),
            discriminant(&DeclaredClientAuthentication::PrivateKeyJwt)
        );
        assert_eq!(client.client_id.as_str(), "blog");
        assert_eq!(client.issuer.to_string(), "margaret");
        assert_eq!(client.tag.to_string(), "blog");
        assert_eq!(
            client.anchor.canonical_path().to_string(),
            "crate::BlogClient"
        );
    }

    #[test]
    fn reads_the_environment_variable_of_a_client_secret() {
        let indexed = IndexedSource::new(
            "#[oauth_client(google, authentication = client_secret_basic(client_secret_from = \"GOOGLE_CLIENT_SECRET\"), client_id = \"app.apps.googleusercontent.com\", issuer = google_accounts)]\npub struct GoogleClient;\n",
        );
        let clients = DeclaredOAuthClients::read(&indexed.index).expect("the client is read");

        assert!(matches!(
            clients.clients.as_slice(),
            [client] if matches!(
                &client.authentication,
                DeclaredClientAuthentication::ClientSecretBasic { client_secret_from }
                    if client_secret_from.as_str() == "GOOGLE_CLIENT_SECRET"
            )
        ));
    }

    #[test]
    fn rejects_a_client_without_a_tag() {
        assert_eq!(
            rejection("authentication = private_key_jwt, client_id = \"blog\", issuer = margaret")
                .to_string(),
            "#[oauth_client] on 'crate::Client' does not name a tag"
        );
    }

    #[test]
    fn rejects_a_tag_that_is_not_a_plain_name() {
        assert_eq!(
            rejection(
                "clients::blog, authentication = private_key_jwt, client_id = \"blog\", issuer = margaret"
            )
            .to_string(),
            "#[oauth_client] on 'crate::Client' names a tag that is not a single plain name"
        );
    }

    #[test]
    fn rejects_a_client_without_authentication() {
        assert_eq!(
            rejection("blog, client_id = \"blog\", issuer = margaret").to_string(),
            "#[oauth_client] on 'crate::Client' does not declare how the client authenticates"
        );
    }

    #[test]
    fn rejects_an_authentication_method_outside_the_vocabulary() {
        assert_eq!(
            rejection(
                "blog, authentication = client_secret_jwt, client_id = \"blog\", issuer = margaret"
            )
            .to_string(),
            "the authentication #[oauth_client] declares on 'crate::Client' is malformed: the client authentication method 'client_secret_jwt' is not supported"
        );
    }

    #[test]
    fn rejects_a_client_that_does_not_authenticate() {
        assert_eq!(
            rejection("blog, authentication = none, client_id = \"blog\", issuer = margaret")
                .to_string(),
            "#[oauth_client] on 'crate::Client' authenticates with 'none', but an oauth client authenticates with private_key_jwt or client_secret_basic"
        );
    }

    #[test]
    fn rejects_a_client_secret_without_its_environment_variable() {
        assert_eq!(
            rejection(
                "blog, authentication = client_secret_basic, client_id = \"blog\", issuer = margaret"
            )
            .to_string(),
            "#[oauth_client] on 'crate::Client' authenticates with client_secret_basic, but does not name the environment variable it reads the secret from"
        );
    }

    #[test]
    fn rejects_a_client_secret_source_that_is_not_an_environment_variable_name() {
        assert_eq!(
            rejection(
                "blog, authentication = client_secret_basic(client_secret_from = \"1SECRET\"), client_id = \"blog\", issuer = margaret"
            )
            .to_string(),
            "#[oauth_client] on 'crate::Client' reads its client secret from '1SECRET', which is not an environment variable name"
        );
    }

    #[test]
    fn rejects_arguments_of_a_private_key_jwt_client() {
        assert!(matches!(
            rejection(
                "blog, authentication = private_key_jwt(client_secret_from = \"SECRET\"), client_id = \"blog\", issuer = margaret"
            ),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnrecognizedArgument { argument, .. }
            ) if argument == "client_secret_from"
        ));
    }

    #[test]
    fn rejects_a_client_secret_source_that_is_not_a_string() {
        assert!(matches!(
            rejection(
                "blog, authentication = client_secret_basic(client_secret_from = SECRET), client_id = \"blog\", issuer = margaret"
            ),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "client_secret_from"
        ));
    }

    #[test]
    fn rejects_a_client_id_that_is_not_a_string() {
        assert!(matches!(
            rejection("blog, authentication = private_key_jwt, client_id = blog, issuer = margaret"),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "client_id"
        ));
    }

    #[test]
    fn rejects_a_client_without_a_client_id() {
        assert_eq!(
            rejection("blog, authentication = private_key_jwt, issuer = margaret").to_string(),
            "#[oauth_client] on 'crate::Client' does not declare its client_id"
        );
    }

    #[test]
    fn rejects_a_client_id_with_invisible_characters() {
        assert_eq!(
            rejection(
                "blog, authentication = private_key_jwt, client_id = \"bl\\tog\", issuer = margaret"
            )
            .to_string(),
            "the client_id #[oauth_client] declares on 'crate::Client' is malformed: the client identifier contains a character outside the visible ascii range"
        );
    }

    #[test]
    fn rejects_a_client_without_an_issuer() {
        assert_eq!(
            rejection("blog, authentication = private_key_jwt, client_id = \"blog\"").to_string(),
            "#[oauth_client] on 'crate::Client' must name its issuer as `issuer = <tag>`"
        );
    }

    #[test]
    fn rejects_an_issuer_that_is_not_a_plain_name() {
        assert_eq!(
            rejection(
                "blog, authentication = private_key_jwt, client_id = \"blog\", issuer = issuers::margaret"
            )
            .to_string(),
            "#[oauth_client] on 'crate::Client' must name its issuer as `issuer = <tag>`"
        );
    }

    #[test]
    fn reports_an_issuer_that_is_not_a_path() {
        assert!(matches!(
            rejection(
                "blog, authentication = private_key_jwt, client_id = \"blog\", issuer = \"margaret\""
            ),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "issuer"
        ));
    }

    #[test]
    fn rejects_a_client_anchored_by_a_singleton() {
        let indexed = IndexedSource::new(
            "#[singleton]\n#[oauth_client(blog, authentication = private_key_jwt, client_id = \"blog\", issuer = margaret)]\npub struct Client;\n",
        );

        assert!(matches!(
            DeclaredOAuthClients::read(&indexed.index),
            Err(OAuthClientCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. }))
                if path == "crate::Client"
        ));
    }

    #[test]
    fn reports_unparseable_client_arguments() {
        assert!(matches!(
            rejection("= 5"),
            OAuthClientCodegenError::Index(AttributeError::Arguments(
                AttributeArgumentsError::Malformed { attribute_path, .. }
            )) if attribute_path == "oauth_client"
        ));
    }
}
