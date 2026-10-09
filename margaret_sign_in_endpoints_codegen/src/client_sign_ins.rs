use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_http_codegen::declared_routes::DeclaredRoutes;
use margaret_sessions_codegen::declared_sessions::DeclaredSessions;
use margaret_tag_codegen::bound_sign_in::BoundSignIn;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;
use margaret_tag_codegen::sign_in_callback_routes::SignInCallbackRoutes;

use crate::client_sign_in::ClientSignIn;
use crate::marked_sign_in_endpoints::MarkedSignInEndpoints;
use crate::served_sign_in::ServedSignIn;
use crate::sign_in_admission_declaration::SignInAdmissionDeclaration;
use crate::sign_in_admissions::sign_in_admissions;
use crate::sign_in_callback::SignInCallback;
use crate::sign_in_endpoints_codegen_error::SignInEndpointsCodegenError;
use crate::sign_in_service::SignInService;

fn signing_in(
    bindings: &[OAuthClientBinding],
    anchor: &CanonicalPath,
    client: &Tag,
) -> Result<(), SignInEndpointsCodegenError> {
    match bindings
        .iter()
        .find(|binding| binding.client.tag == *client)
        .map(|binding| &binding.sign_in)
    {
        Some(BoundSignIn::Available { .. }) => Ok(()),
        Some(BoundSignIn::Unavailable) => Err(SignInEndpointsCodegenError::ClientWithoutSignIn {
            anchor: anchor.to_string(),
            client: client.to_string(),
        }),
        None => Err(SignInEndpointsCodegenError::UnknownSignInClient {
            anchor: anchor.to_string(),
            client: client.to_string(),
        }),
    }
}

fn unique<'declared, TDeclared>(
    mut candidates: impl Iterator<Item = &'declared TDeclared>,
    path: impl Fn(&TDeclared) -> &CanonicalPath,
    ambiguous: impl FnOnce(String, String) -> SignInEndpointsCodegenError,
    missing: impl FnOnce() -> SignInEndpointsCodegenError,
) -> Result<&'declared TDeclared, SignInEndpointsCodegenError> {
    let Some(first) = candidates.next() else {
        return Err(missing());
    };

    match candidates.next() {
        Some(second) => Err(ambiguous(path(first).to_string(), path(second).to_string())),
        None => Ok(first),
    }
}

fn service_of<'declarations>(
    binding: &OAuthClientBinding<'declarations, '_>,
    marked: &MarkedSignInEndpoints,
    admissions: &[SignInAdmissionDeclaration],
    declared_routes: &DeclaredRoutes,
    sessions: &DeclaredSessions,
) -> Result<SignInService<'declarations>, SignInEndpointsCodegenError> {
    let BoundSignIn::Available {
        callback_routes,
        scopes,
    } = &binding.sign_in
    else {
        return Ok(SignInService::Unavailable);
    };
    let client = &binding.client.tag;
    let named = || client.to_string();

    if !matches!(sessions, DeclaredSessions::Issued(_)) {
        return Err(SignInEndpointsCodegenError::SignInWithoutIssuedSessions { client: named() });
    }

    let start = unique(
        marked.starts.iter().filter(|start| start.client == *client),
        |start| &start.route,
        |first, second| SignInEndpointsCodegenError::AmbiguousSignInStart {
            client: named(),
            first,
            second,
        },
        || SignInEndpointsCodegenError::MissingSignInStart { client: named() },
    )?;
    let callback = unique(
        marked
            .callbacks
            .iter()
            .filter(|callback| callback.client == *client),
        |callback| &callback.route,
        |first, second| SignInEndpointsCodegenError::AmbiguousSignInCallback {
            client: named(),
            first,
            second,
        },
        || SignInEndpointsCodegenError::MissingSignInCallback { client: named() },
    )?;
    let admission = unique(
        admissions
            .iter()
            .filter(|admission| admission.client == *client),
        |admission| &admission.admission,
        |first, second| SignInEndpointsCodegenError::AmbiguousSignInAdmission {
            client: named(),
            first,
            second,
        },
        || SignInEndpointsCodegenError::MissingSignInAdmission { client: named() },
    )?;

    if let SignInCallbackRoutes::Admitted(routes) = callback_routes
        && !routes.contains(&callback.route)
    {
        return Err(SignInEndpointsCodegenError::UnregisteredSignInCallback {
            client: named(),
            route: callback.route.to_string(),
        });
    }

    Ok(SignInService::Served(ServedSignIn {
        admission: admission.admission.clone(),
        callback: SignInCallback {
            landing: callback.landing.clone(),
            route: callback.route.clone(),
            url: declared_routes
                .redirect_target(&callback.route, binding.client.anchor.canonical_path())?,
        },
        scopes,
        start: start.route.clone(),
    }))
}

/// # Errors
///
/// Returns `SignInEndpointsCodegenError` when a `#[serves_sign_in]` or `#[admits_sign_in]`
/// declaration is malformed or names a client that does not sign in, when a signing-in client
/// lacks or repeats its start route, its callback route or its admission hook, when its callback
/// route is not registered with its admitted client, or when the crate issues no sessions to
/// start.
pub fn client_sign_ins<'bindings, 'declarations, 'index>(
    index: &AttributeIndex,
    declared_routes: &DeclaredRoutes,
    bindings: &'bindings [OAuthClientBinding<'declarations, 'index>],
    sessions: &DeclaredSessions,
) -> Result<Vec<ClientSignIn<'bindings, 'declarations, 'index>>, SignInEndpointsCodegenError> {
    let marked = MarkedSignInEndpoints::read(index, declared_routes)?;
    let admissions = sign_in_admissions(index)?;

    for start in &marked.starts {
        signing_in(bindings, &start.route, &start.client)?;
    }

    for callback in &marked.callbacks {
        signing_in(bindings, &callback.route, &callback.client)?;
    }

    for admission in &admissions {
        signing_in(bindings, &admission.admission, &admission.client)?;
    }

    bindings
        .iter()
        .map(|binding| {
            Ok(ClientSignIn {
                binding,
                service: service_of(binding, &marked, &admissions, declared_routes, sessions)?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
    use margaret_http_codegen::declared_routes::DeclaredRoutes;
    use margaret_http_codegen::http_codegen_error::HttpCodegenError;
    use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
    use margaret_oauth_vocabulary::scope::Scope;
    use margaret_oauth_vocabulary_codegen::declared_scopes::DeclaredScopes;
    use margaret_route_method::route_method::RouteMethod;
    use margaret_serve_input_codegen::route_url_input::RouteUrlInput;
    use margaret_sessions_codegen::declared_sessions::DeclaredSessions;
    use margaret_tag_codegen::tag_pool::TagPool;
    use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
    use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;

    use super::client_sign_ins;
    use crate::client_sign_in::ClientSignIn;
    use crate::served_sign_in::ServedSignIn;
    use crate::sign_in_callback::SignInCallback;
    use crate::sign_in_endpoints_codegen_error::SignInEndpointsCodegenError;
    use crate::sign_in_service::SignInService;

    const PREAMBLE: &str = "use margaret::framework::oidc_sign_in::sign_in_endpoint::SignInEndpoint;\nuse margaret::framework::route_method::route_method::RouteMethod;\n\n#[issues_tokens(provider, issuer = \"https://issuer.example\")]\nstruct ProviderIssuance;\n#[issues_resource_tokens(reports, audience = \"reports\")]\nstruct ReportsResource;\n#[oauth_scope(name = \"profile\")]\nstruct ProfileScope;\n#[singleton]\n#[responds_to_http(method = RouteMethod::Get, path = \"/welcome\", server = \"public\")]\nstruct GetWelcome;\n";

    const SESSIONS: &str = "#[issues_sessions(issuer = provider, audience = \"browser\", cookies = margaret::framework::sessions::session_cookies::SessionCookies::HostOnly)]\nstruct BrowserSessions;\n";

    const PARTNER: &str = "#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct PartnerIssuer;\n#[oauth_client(partner_client, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"partner\", issuer = partner, sign_in(scopes = [ProfileScope]))]\nstruct PartnerClient;\n#[admits_oauth_client(spa_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"spa\", resources = [reports])]\nstruct SpaClient;\n";

    const OWN: &str = "#[admits_oauth_client(blog_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt(keys = margaret::framework::accepted_clients::client_keys::ClientKeys::Own), authorization_code(consent = margaret::framework::accepted_clients::consent_policy::ConsentPolicy::Implicit, id_token_signing = margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning::Rsa, redirect_routes = [GetBlogCallback], scopes = [margaret::framework::oauth_vocabulary::openid_scope::OpenidScope, ProfileScope]), client_id = \"blog\", resources = [reports])]\nstruct BlogApp;\n#[oauth_client(blog, admitted_as = blog_app)]\nstruct BlogClient;\n";

    const REGISTERED_ROUTE: &str = "#[singleton]\n#[responds_to_http(method = RouteMethod::Get, path = \"/blog/registered\", server = \"public\")]\nstruct GetBlogCallback;\n";

    fn routed(name: &str, method: &str, path: &str, marker: &str) -> String {
        format!(
            "#[responds_to_http(method = RouteMethod::{method}, path = \"{path}\", server = \"public\")]\n{marker}\nstruct {name};\n"
        )
    }

    fn start(client: &str) -> String {
        routed(
            "GetSignIn",
            "Get",
            "/sign-in",
            &format!("#[serves_sign_in(SignInEndpoint::Start, client = {client})]"),
        )
    }

    fn callback(client: &str) -> String {
        routed(
            "GetSignInCallback",
            "Get",
            "/sign-in/callback",
            &format!(
                "#[serves_sign_in(SignInEndpoint::Callback(landing_route = GetWelcome), client = {client})]"
            ),
        )
    }

    fn admission(client: &str) -> String {
        format!("#[singleton]\n#[admits_sign_in(client = {client})]\nstruct Readers;\n")
    }

    fn partner_sign_in() -> String {
        format!(
            "{PREAMBLE}{SESSIONS}{PARTNER}{}{}{}",
            start("partner_client"),
            callback("partner_client"),
            admission("partner_client")
        )
    }

    fn read<TOutcome>(
        source: &str,
        inspect: impl FnOnce(Result<Vec<ClientSignIn>, SignInEndpointsCodegenError>) -> TOutcome,
    ) -> TOutcome {
        let indexed = IndexedSource::new(source);
        let index = &indexed.index;
        let issuance = DeclaredTokenIssuance::read(index).expect("the token issuance is read");
        let resources =
            DeclaredResourceIssuances::read(index, &issuance).expect("the resources are read");
        let scopes = DeclaredScopes::read(index).expect("the scopes are read");
        let trusts = DeclaredTrusts::read(index).expect("the trusts are read");
        let clients = DeclaredOAuthClients::read(index, &scopes).expect("the clients are read");
        let admitted = DeclaredAcceptedClients::read(index, &issuance, &resources, &scopes)
            .expect("the admitted clients are read");
        let sessions =
            DeclaredSessions::read(index, &issuance, &resources).expect("the sessions are read");
        let bindings = TagPool::collect(
            index, &trusts, &clients, &issuance, &resources, &admitted, &sessions,
        )
        .expect("the tags are collected")
        .oauth_client_bindings(&trusts, &clients, &issuance, &admitted)
        .expect("the clients bind");
        let routes = DeclaredRoutes::read(index).expect("the routes are declared");

        inspect(client_sign_ins(index, &routes, &bindings, &sessions))
    }

    fn rejection(source: &str) -> SignInEndpointsCodegenError {
        read(source, |read| read.err().expect("the sign-in is rejected"))
    }

    fn public(path: &str) -> RouteUrlInput {
        RouteUrlInput {
            path: path.to_string(),
            server: "public".to_string(),
        }
    }

    #[test]
    fn serves_the_sign_in_of_an_external_client() {
        read(&partner_sign_in(), |read| {
            assert!(matches!(
                read.expect("the sign-in is read").as_slice(),
                [ClientSignIn {
                    binding,
                    service: SignInService::Served(ServedSignIn {
                        admission,
                        callback: SignInCallback { landing, route, url },
                        scopes,
                        start,
                    }),
                }] if binding.client.tag.to_string() == "partner_client"
                    && scopes.iter().map(Scope::as_str).eq(["profile"])
                    && admission.to_string() == "crate::Readers"
                    && *landing == public("/welcome")
                    && route.to_string() == "crate::GetSignInCallback"
                    && *url == public("/sign-in/callback")
                    && start.to_string() == "crate::GetSignIn"
            ));
        });
    }

    #[test]
    fn serves_the_sign_in_of_an_own_client_through_its_registered_redirect_route() {
        read(
            &format!(
                "{PREAMBLE}{SESSIONS}{OWN}{}{}{}",
                start("blog"),
                routed(
                    "GetBlogCallback",
                    "Get",
                    "/blog/registered",
                    "#[serves_sign_in(SignInEndpoint::Callback(landing_route = GetWelcome), client = blog)]"
                ),
                admission("blog")
            ),
            |read| {
                assert!(matches!(
                    read.expect("the sign-in is read").as_slice(),
                    [ClientSignIn {
                        service: SignInService::Served(ServedSignIn { callback, .. }),
                        ..
                    }] if callback.route.to_string() == "crate::GetBlogCallback"
                ));
            },
        );
    }

    #[test]
    fn leaves_a_client_without_sign_in_unavailable() {
        read(
            &format!("{PREAMBLE}{PARTNER}").replace(", sign_in(scopes = [ProfileScope])", ""),
            |read| {
                assert!(matches!(
                    read.expect("the clients are read").as_slice(),
                    [ClientSignIn {
                        service: SignInService::Unavailable,
                        ..
                    }]
                ));
            },
        );
    }

    #[test]
    fn rejects_a_start_of_an_unknown_client() {
        assert!(matches!(
            rejection(&format!("{}{}", partner_sign_in(), start("missing").replace("GetSignIn;", "GetOtherSignIn;").replace("/sign-in\"", "/other\""))),
            SignInEndpointsCodegenError::UnknownSignInClient { anchor, client }
                if anchor == "crate::GetOtherSignIn" && client == "missing"
        ));
    }

    #[test]
    fn rejects_a_callback_of_a_client_without_sign_in() {
        assert!(matches!(
            rejection(&format!("{PREAMBLE}{SESSIONS}{PARTNER}{}", callback("partner_client")).replace(", sign_in(scopes = [ProfileScope])", "")),
            SignInEndpointsCodegenError::ClientWithoutSignIn { anchor, client }
                if anchor == "crate::GetSignInCallback" && client == "partner_client"
        ));
    }

    #[test]
    fn rejects_an_admission_of_an_unknown_client() {
        assert!(matches!(
            rejection(&format!("{PREAMBLE}{SESSIONS}{PARTNER}{}", admission("missing"))),
            SignInEndpointsCodegenError::UnknownSignInClient { anchor, client }
                if anchor == "crate::Readers" && client == "missing"
        ));
    }

    #[test]
    fn rejects_a_sign_in_without_issued_sessions() {
        assert!(matches!(
            rejection(&partner_sign_in().replace(SESSIONS, "")),
            SignInEndpointsCodegenError::SignInWithoutIssuedSessions { client } if client == "partner_client"
        ));
    }

    #[test]
    fn rejects_a_sign_in_starting_at_two_routes() {
        assert!(matches!(
            rejection(&format!("{}{}", partner_sign_in(), routed("GetOtherSignIn", "Get", "/other", "#[serves_sign_in(SignInEndpoint::Start, client = partner_client)]"))),
            SignInEndpointsCodegenError::AmbiguousSignInStart { client, first, second }
                if client == "partner_client" && first == "crate::GetOtherSignIn" && second == "crate::GetSignIn"
        ));
    }

    #[test]
    fn rejects_a_sign_in_returning_to_two_routes() {
        assert!(matches!(
            rejection(&format!("{}{}", partner_sign_in(), routed("GetOtherCallback", "Get", "/other", "#[serves_sign_in(SignInEndpoint::Callback(landing_route = GetWelcome), client = partner_client)]"))),
            SignInEndpointsCodegenError::AmbiguousSignInCallback { client, first, second }
                if client == "partner_client" && first == "crate::GetOtherCallback" && second == "crate::GetSignInCallback"
        ));
    }

    #[test]
    fn rejects_a_sign_in_admitted_twice() {
        assert!(matches!(
            rejection(&format!("{}{}", partner_sign_in(), admission("partner_client").replace("Readers", "Editors"))),
            SignInEndpointsCodegenError::AmbiguousSignInAdmission { client, first, second }
                if client == "partner_client" && first == "crate::Editors" && second == "crate::Readers"
        ));
    }

    #[test]
    fn rejects_a_sign_in_without_its_start_route() {
        assert!(matches!(
            rejection(&partner_sign_in().replace(&start("partner_client"), "")),
            SignInEndpointsCodegenError::MissingSignInStart { client } if client == "partner_client"
        ));
    }

    #[test]
    fn rejects_a_sign_in_without_its_callback_route() {
        assert!(matches!(
            rejection(&partner_sign_in().replace(&callback("partner_client"), "")),
            SignInEndpointsCodegenError::MissingSignInCallback { client } if client == "partner_client"
        ));
    }

    #[test]
    fn rejects_a_sign_in_without_its_admission() {
        assert!(matches!(
            rejection(&partner_sign_in().replace(&admission("partner_client"), "")),
            SignInEndpointsCodegenError::MissingSignInAdmission { client } if client == "partner_client"
        ));
    }

    #[test]
    fn rejects_an_own_callback_its_admitted_client_does_not_register() {
        assert!(matches!(
            rejection(&format!("{PREAMBLE}{SESSIONS}{OWN}{REGISTERED_ROUTE}{}{}{}", start("blog"), callback("blog"), admission("blog"))),
            SignInEndpointsCodegenError::UnregisteredSignInCallback { client, route }
                if client == "blog" && route == "crate::GetSignInCallback"
        ));
    }

    #[test]
    fn rejects_a_parameterized_callback_route() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("/sign-in/callback", "/sign-in/{provider}")),
            SignInEndpointsCodegenError::Route(HttpCodegenError::ParameterizedRedirectRoute {
                path,
                route,
                ..
            }) if path == "/sign-in/{provider}" && route == "crate::GetSignInCallback"
        ));
    }

    #[test]
    fn rejects_a_landing_route_a_redirect_cannot_reach() {
        assert!(matches!(
            rejection(&partner_sign_in().replace(
                "method = RouteMethod::Get, path = \"/welcome\"",
                "method = RouteMethod::Post, path = \"/welcome\""
            )),
            SignInEndpointsCodegenError::Route(HttpCodegenError::RedirectRouteNotGet {
                referrer,
                route,
            }) if referrer == "crate::GetSignInCallback" && route == "crate::GetWelcome"
        ));
    }

    #[test]
    fn rejects_a_callback_without_its_landing_route() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("SignInEndpoint::Callback(landing_route = GetWelcome)", "SignInEndpoint::Callback")),
            SignInEndpointsCodegenError::MissingLandingRoute { anchor } if anchor == "crate::GetSignInCallback"
        ));
    }

    #[test]
    fn rejects_a_landing_route_that_is_not_a_path() {
        assert_eq!(
            rejection(&partner_sign_in().replace(
                "landing_route = GetWelcome",
                "landing_route = \"GetWelcome\""
            ))
            .to_string(),
            "argument 'landing_route' of attribute 'serves_sign_in::SignInEndpoint::Callback' is not a path"
        );
    }

    #[test]
    fn rejects_a_landing_route_that_names_no_item() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("landing_route = GetWelcome", "landing_route = Missing")),
            SignInEndpointsCodegenError::UnknownLandingRoute { anchor, written }
                if anchor == "crate::GetSignInCallback" && written == "Missing"
        ));
    }

    #[test]
    fn rejects_a_marker_without_an_endpoint() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("#[serves_sign_in(SignInEndpoint::Start, client = partner_client)]", "#[serves_sign_in(client = partner_client)]")),
            SignInEndpointsCodegenError::MissingSignInEndpoint { anchor } if anchor == "crate::GetSignIn"
        ));
    }

    #[test]
    fn rejects_a_variant_of_a_foreign_enum() {
        assert!(matches!(
            rejection(&format!("{}enum Endpoint {{ Start }}\n", partner_sign_in()).replace("SignInEndpoint::Start,", "Endpoint::Start,")),
            SignInEndpointsCodegenError::UnknownSignInEndpoint { anchor, written }
                if anchor == "crate::GetSignIn" && written == "Endpoint::Start"
        ));
    }

    #[test]
    fn rejects_a_marker_without_its_client() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("SignInEndpoint::Start, client = partner_client", "SignInEndpoint::Start")),
            SignInEndpointsCodegenError::MissingSignInClient { anchor } if anchor == "crate::GetSignIn"
        ));
    }

    #[test]
    fn rejects_an_admission_naming_a_client_that_is_not_a_tag() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("#[admits_sign_in(client = partner_client)]", "#[admits_sign_in(client = clients::partner_client)]")),
            SignInEndpointsCodegenError::MalformedSignInClient { anchor } if anchor == "crate::Readers"
        ));
    }

    #[test]
    fn rejects_an_unrouted_marker() {
        assert!(matches!(
            rejection(&format!("{}#[serves_sign_in(SignInEndpoint::Start, client = partner_client)]\nstruct Unrouted;\n", partner_sign_in())),
            SignInEndpointsCodegenError::UnroutedSignInEndpoint { anchor } if anchor == "crate::Unrouted"
        ));
    }

    #[test]
    fn rejects_a_marker_routed_by_another_method() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("method = RouteMethod::Get, path = \"/sign-in\"", "method = RouteMethod::Post, path = \"/sign-in\"")),
            SignInEndpointsCodegenError::SignInEndpointMethod { anchor, method }
                if anchor == "crate::GetSignIn" && method == RouteMethod::Post
        ));
    }

    #[test]
    fn rejects_a_marker_on_a_singleton() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("#[responds_to_http(method = RouteMethod::Get, path = \"/sign-in\"", "#[singleton]\n#[responds_to_http(method = RouteMethod::Get, path = \"/sign-in\"")),
            SignInEndpointsCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. })
                if path == "crate::GetSignIn"
        ));
    }

    #[test]
    fn rejects_marker_arguments_that_do_not_parse() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("#[serves_sign_in(SignInEndpoint::Start, client = partner_client)]", "#[serves_sign_in(= 5)]")),
            SignInEndpointsCodegenError::Index(AttributeError::Arguments(
                AttributeArgumentsError::Malformed { attribute_path, .. }
            )) if attribute_path == "serves_sign_in"
        ));
    }

    #[test]
    fn rejects_admission_arguments_that_do_not_parse() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("#[admits_sign_in(client = partner_client)]", "#[admits_sign_in(= 5)]")),
            SignInEndpointsCodegenError::Index(AttributeError::Arguments(
                AttributeArgumentsError::Malformed { attribute_path, .. }
            )) if attribute_path == "admits_sign_in"
        ));
    }

    #[test]
    fn rejects_a_client_that_is_not_a_path() {
        assert!(matches!(
            rejection(&partner_sign_in().replace("#[admits_sign_in(client = partner_client)]", "#[admits_sign_in(client = \"partner_client\")]")),
            SignInEndpointsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "client"
        ));
    }
}
