use quote::ToTokens;
use syn::Path;

use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::tag::Tag;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;
use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::client_id_parsing::ClientIdParsing;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_parsing::ScopeParsing;
use margaret_oauth_vocabulary_codegen::client_authentication_methods::CLIENT_AUTHENTICATION_METHODS;

use crate::declared_client_authentication::DeclaredClientAuthentication;
use crate::declared_client_registration::DeclaredClientRegistration;
use crate::declared_sign_in::DeclaredSignIn;
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
    index: &AttributeIndex,
    item: &IndexedItem,
    variant: &Path,
    reader: &mut AttributeArgumentsReader,
) -> Result<DeclaredClientAuthentication, OAuthClientCodegenError> {
    let anchor = item.canonical_path().to_string();
    let anchor = anchor.as_str();
    let method = index
        .resolve_item_path(item, variant)
        .as_ref()
        .and_then(|resolved| CLIENT_AUTHENTICATION_METHODS.variant(resolved))
        .ok_or_else(|| OAuthClientCodegenError::UnknownAuthentication {
            anchor: anchor.to_string(),
            written: variant.to_token_stream().to_string(),
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

fn sign_in(
    index: &AttributeIndex,
    item: &IndexedItem,
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<DeclaredSignIn, OAuthClientCodegenError> {
    let declared = reader.take_group("sign_in", |group| {
        let written_route = group
            .take_path(ItemNamingArgument::RedirectRoute.key())?
            .ok_or_else(|| OAuthClientCodegenError::MissingSignInRedirectRoute {
                anchor: anchor.to_string(),
            })?;
        let redirect_route = index
            .resolve_item_path(item, &written_route)
            .ok_or_else(|| OAuthClientCodegenError::UnknownSignInRedirectRoute {
                anchor: anchor.to_string(),
                written: written_route.to_token_stream().to_string(),
            })?;
        let scopes = group
            .take_string_array("scopes")?
            .ok_or_else(|| OAuthClientCodegenError::MissingSignInScopes {
                anchor: anchor.to_string(),
            })?
            .iter()
            .map(|scope| match Scope::parse(scope) {
                ScopeParsing::Accepted(scope) => Ok(scope),
                ScopeParsing::Rejected(rejection) => {
                    Err(OAuthClientCodegenError::MalformedSignInScope {
                        anchor: anchor.to_string(),
                        rejection,
                    })
                }
            })
            .collect::<Result<Vec<Scope>, OAuthClientCodegenError>>()?;

        Ok::<DeclaredSignIn, OAuthClientCodegenError>(DeclaredSignIn::Declared {
            redirect_route,
            scopes,
        })
    })?;

    Ok(declared.unwrap_or(DeclaredSignIn::Undeclared))
}

fn external_registration(
    index: &AttributeIndex,
    anchor: &IndexedItem,
    reader: &mut AttributeArgumentsReader,
) -> Result<DeclaredClientRegistration, OAuthClientCodegenError> {
    let path = anchor.canonical_path().to_string();
    let authentication = reader
        .take_variant(
            ItemNamingArgument::ClientAuthentication.key(),
            |variant, arguments| declared_authentication(index, anchor, variant, arguments),
        )?
        .ok_or_else(|| OAuthClientCodegenError::MissingAuthentication {
            anchor: path.clone(),
        })?;
    let client_id = match ClientId::parse(&reader.take_string("client_id")?.ok_or_else(|| {
        OAuthClientCodegenError::MissingClientId {
            anchor: path.clone(),
        }
    })?) {
        ClientIdParsing::Accepted(client_id) => client_id,
        ClientIdParsing::Rejected(rejection) => {
            return Err(OAuthClientCodegenError::MalformedClientId {
                anchor: path.clone(),
                rejection,
            });
        }
    };
    let issuer = reader
        .take_path("issuer")?
        .as_ref()
        .and_then(Tag::from_path)
        .ok_or_else(|| OAuthClientCodegenError::MalformedIssuer {
            anchor: path.clone(),
        })?;

    Ok(DeclaredClientRegistration::External {
        authentication,
        client_id,
        issuer,
        sign_in: sign_in(index, anchor, reader, &path)?,
    })
}

pub struct OAuthClientDeclaration<'index> {
    pub anchor: &'index IndexedItem,
    pub registration: DeclaredClientRegistration,
    pub tag: Tag,
}

impl<'index> OAuthClientDeclaration<'index> {
    pub(crate) fn read(
        index: &'index AttributeIndex,
        matched: &MatchedAttribute<'index>,
    ) -> Result<Self, OAuthClientCodegenError> {
        let anchor =
            declaration_anchor(index, matched, FrameworkAttribute::ActsAsOAuthClient)?.item;
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
            let registration = match reader.take_path("admitted_as")? {
                Some(admitted) => DeclaredClientRegistration::Admitted {
                    admitted: Tag::from_path(&admitted).ok_or_else(|| {
                        OAuthClientCodegenError::MalformedAdmittedClient {
                            anchor: path.clone(),
                        }
                    })?,
                },
                None => external_registration(index, anchor, reader)?,
            };

            Ok(Self {
                anchor,
                registration,
                tag,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
    use margaret_oauth_vocabulary::scope::Scope;

    use crate::declared_client_authentication::DeclaredClientAuthentication;
    use crate::declared_client_registration::DeclaredClientRegistration;
    use crate::declared_oauth_clients::DeclaredOAuthClients;
    use crate::declared_sign_in::DeclaredSignIn;
    use crate::oauth_client_codegen_error::OAuthClientCodegenError;

    fn rejection(arguments: &str) -> OAuthClientCodegenError {
        let indexed = IndexedSource::new(&format!(
            "use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;\n\n#[acts_as_oauth_client({arguments})]\npub struct Client;\n"
        ));

        DeclaredOAuthClients::read(&indexed.index)
            .err()
            .expect("the declaration is rejected")
    }

    #[test]
    fn reads_a_client_that_authenticates_with_its_private_key() {
        let indexed = IndexedSource::new(
            "#[acts_as_oauth_client(blog, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"blog\", issuer = margaret)]\npub struct BlogClient;\n",
        );
        let clients = DeclaredOAuthClients::read(&indexed.index).expect("the client is read");

        assert!(matches!(
            clients.clients.as_slice(),
            [client] if client.tag.to_string() == "blog"
                && client.anchor.canonical_path().to_string() == "crate::BlogClient"
                && matches!(
                    &client.registration,
                    DeclaredClientRegistration::External {
                        authentication: DeclaredClientAuthentication::PrivateKeyJwt,
                        client_id,
                        issuer,
                        sign_in: DeclaredSignIn::Undeclared,
                    } if client_id.as_str() == "blog" && issuer.to_string() == "margaret"
                )
        ));
    }

    fn signing_in(sign_in: &str) -> String {
        format!(
            "blog, authentication = ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"blog\", issuer = margaret, {sign_in}"
        )
    }

    #[test]
    fn reads_the_route_and_scopes_a_client_signs_in_with() {
        let indexed = IndexedSource::new(&format!(
            "use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;\nuse crate::routes::Callback;\n\nmod routes {{\n    pub struct Callback;\n}}\n\n#[acts_as_oauth_client({})]\npub struct Client;\n",
            signing_in("sign_in(redirect_route = Callback, scopes = [\"profile\"])")
        ));
        let clients = DeclaredOAuthClients::read(&indexed.index).expect("the client is read");

        assert!(matches!(
            clients.clients.as_slice(),
            [client] if matches!(
                &client.registration,
                DeclaredClientRegistration::External {
                    sign_in: DeclaredSignIn::Declared { redirect_route, scopes },
                    ..
                } if redirect_route.to_string() == "crate::routes::Callback"
                    && scopes.iter().map(Scope::as_str).eq(["profile"])
            )
        ));
    }

    #[test]
    fn rejects_a_sign_in_without_its_redirect_route() {
        assert_eq!(
            rejection(&signing_in("sign_in(scopes = [\"profile\"])")).to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' signs in without naming the route it redirects back to"
        );
    }

    #[test]
    fn rejects_a_sign_in_through_a_route_that_names_no_item() {
        assert_eq!(
            rejection(&signing_in(
                "sign_in(redirect_route = Missing, scopes = [\"profile\"])"
            ))
            .to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' signs in through 'Missing', which names no item"
        );
    }

    #[test]
    fn rejects_a_sign_in_without_its_scopes() {
        assert_eq!(
            rejection(&signing_in("sign_in(redirect_route = Client)")).to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' signs in without declaring the scopes it requests"
        );
    }

    #[test]
    fn rejects_a_malformed_sign_in_scope() {
        assert!(matches!(
            rejection(&signing_in(
                "sign_in(redirect_route = Client, scopes = [\"a\\\"b\"])"
            )),
            OAuthClientCodegenError::MalformedSignInScope { anchor, .. } if anchor == "crate::Client"
        ));
    }

    #[test]
    fn reads_a_client_of_the_own_provider_by_its_admitted_client() {
        let indexed = IndexedSource::new(
            "#[acts_as_oauth_client(blog, admitted_as = blog_app)]
pub struct BlogClient;
",
        );
        let clients = DeclaredOAuthClients::read(&indexed.index).expect("the client is read");

        assert!(matches!(
            clients.clients.as_slice(),
            [client] if matches!(
                &client.registration,
                DeclaredClientRegistration::Admitted { admitted } if admitted.to_string() == "blog_app"
            )
        ));
    }

    #[test]
    fn rejects_an_admitted_client_that_is_not_a_plain_tag() {
        assert_eq!(
            rejection("blog, admitted_as = apps::blog").to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' must name the client the provider admits it as by a plain tag"
        );
    }

    #[test]
    fn rejects_a_client_id_beside_its_admitted_client() {
        assert!(matches!(
            rejection("blog, admitted_as = blog_app, client_id = \"blog\""),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnrecognizedArgument { argument, .. }
            ) if argument == "client_id"
        ));
    }

    #[test]
    fn reads_the_environment_variable_of_a_client_secret() {
        let indexed = IndexedSource::new(
            "#[acts_as_oauth_client(google, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::ClientSecretBasic(client_secret_from = \"GOOGLE_CLIENT_SECRET\"), client_id = \"app.apps.googleusercontent.com\", issuer = google_accounts)]\npub struct GoogleClient;\n",
        );
        let clients = DeclaredOAuthClients::read(&indexed.index).expect("the client is read");

        assert!(matches!(
            clients.clients.as_slice(),
            [client] if matches!(
                &client.registration,
                DeclaredClientRegistration::External {
                    authentication: DeclaredClientAuthentication::ClientSecretBasic { client_secret_from },
                    ..
                } if client_secret_from.as_str() == "GOOGLE_CLIENT_SECRET"
            )
        ));
    }

    #[test]
    fn rejects_a_client_without_a_tag() {
        assert_eq!(
            rejection("authentication = ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"blog\", issuer = margaret")
                .to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' does not name a tag"
        );
    }

    #[test]
    fn rejects_a_tag_that_is_not_a_plain_name() {
        assert_eq!(
            rejection(
                "clients::blog, authentication = ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"blog\", issuer = margaret"
            )
            .to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' names a tag that is not a single plain name"
        );
    }

    #[test]
    fn rejects_a_client_without_authentication() {
        assert_eq!(
            rejection("blog, client_id = \"blog\", issuer = margaret").to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' does not declare how the client authenticates"
        );
    }

    #[test]
    fn rejects_an_authentication_method_outside_the_vocabulary() {
        assert_eq!(
            rejection(
                "blog, authentication = ClientAuthenticationMethod::ClientSecretJwt, client_id = \"blog\", issuer = margaret"
            )
            .to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' authenticates with 'ClientAuthenticationMethod :: ClientSecretJwt', which is not a variant of margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod"
        );
    }

    #[test]
    fn rejects_a_client_that_does_not_authenticate() {
        assert_eq!(
            rejection("blog, authentication = ClientAuthenticationMethod::None, client_id = \"blog\", issuer = margaret")
                .to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' authenticates with 'none', but an oauth client authenticates with ClientAuthenticationMethod::PrivateKeyJwt or ClientAuthenticationMethod::ClientSecretBasic"
        );
    }

    #[test]
    fn rejects_a_client_secret_without_its_environment_variable() {
        assert_eq!(
            rejection(
                "blog, authentication = ClientAuthenticationMethod::ClientSecretBasic, client_id = \"blog\", issuer = margaret"
            )
            .to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' authenticates with ClientAuthenticationMethod::ClientSecretBasic, but does not name the environment variable it reads the secret from"
        );
    }

    #[test]
    fn rejects_a_client_secret_source_that_is_not_an_environment_variable_name() {
        assert_eq!(
            rejection(
                "blog, authentication = ClientAuthenticationMethod::ClientSecretBasic(client_secret_from = \"1SECRET\"), client_id = \"blog\", issuer = margaret"
            )
            .to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' reads its client secret from '1SECRET', which is not an environment variable name"
        );
    }

    #[test]
    fn rejects_arguments_of_a_private_key_jwt_client() {
        assert!(matches!(
            rejection(
                "blog, authentication = ClientAuthenticationMethod::PrivateKeyJwt(client_secret_from = \"SECRET\"), client_id = \"blog\", issuer = margaret"
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
                "blog, authentication = ClientAuthenticationMethod::ClientSecretBasic(client_secret_from = SECRET), client_id = \"blog\", issuer = margaret"
            ),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "client_secret_from"
        ));
    }

    #[test]
    fn rejects_a_sign_in_redirect_route_that_is_not_a_path() {
        assert!(matches!(
            rejection(&signing_in("sign_in(redirect_route = \"callback\", scopes = [\"profile\"])")),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "redirect_route"
        ));
    }

    #[test]
    fn rejects_sign_in_scopes_that_are_not_an_array() {
        assert!(matches!(
            rejection(&signing_in("sign_in(redirect_route = Client, scopes = \"profile\")")),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "scopes"
        ));
    }

    #[test]
    fn rejects_an_admitted_client_that_is_not_a_path() {
        assert!(matches!(
            rejection("blog, admitted_as = \"blog_app\""),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "admitted_as"
        ));
    }

    #[test]
    fn rejects_a_client_id_that_is_not_a_string() {
        assert!(matches!(
            rejection("blog, authentication = ClientAuthenticationMethod::PrivateKeyJwt, client_id = blog, issuer = margaret"),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "client_id"
        ));
    }

    #[test]
    fn rejects_a_client_without_a_client_id() {
        assert_eq!(
            rejection("blog, authentication = ClientAuthenticationMethod::PrivateKeyJwt, issuer = margaret").to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' does not declare its client_id"
        );
    }

    #[test]
    fn rejects_a_client_id_with_invisible_characters() {
        assert_eq!(
            rejection(
                "blog, authentication = ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"bl\\tog\", issuer = margaret"
            )
            .to_string(),
            "the client_id #[acts_as_oauth_client] declares on 'crate::Client' is malformed: the client identifier contains a character outside the visible ascii range"
        );
    }

    #[test]
    fn rejects_a_client_without_an_issuer() {
        assert_eq!(
            rejection("blog, authentication = ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"blog\"").to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' must name its issuer as `issuer = <tag>`"
        );
    }

    #[test]
    fn rejects_an_issuer_that_is_not_a_plain_name() {
        assert_eq!(
            rejection(
                "blog, authentication = ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"blog\", issuer = issuers::margaret"
            )
            .to_string(),
            "#[acts_as_oauth_client] on 'crate::Client' must name its issuer as `issuer = <tag>`"
        );
    }

    #[test]
    fn reports_an_issuer_that_is_not_a_path() {
        assert!(matches!(
            rejection(
                "blog, authentication = ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"blog\", issuer = \"margaret\""
            ),
            OAuthClientCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "issuer"
        ));
    }

    #[test]
    fn rejects_a_client_anchored_by_a_singleton() {
        let indexed = IndexedSource::new(
            "#[singleton]\n#[acts_as_oauth_client(blog, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"blog\", issuer = margaret)]\npub struct Client;\n",
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
            )) if attribute_path == "acts_as_oauth_client"
        ));
    }
}
