use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;

use crate::constants::SESSIONS_MODULE_NAME;
use crate::constants::SIGN_OUT_ENDPOINTS_MODULE_NAME;
use crate::consumed_sessions_declaration::ConsumedSessionsDeclaration;
use crate::declared_sessions::DeclaredSessions;
use crate::issued_sessions_declaration::IssuedSessionsDeclaration;
use crate::render_sign_out_endpoints::render_sign_out_endpoints;
use crate::sessions_item::SessionsItem;
use crate::sign_out_endpoint_module::SignOutEndpointModule;

#[must_use]
pub fn render_sessions(
    sessions: &DeclaredSessions,
    served_endpoints: &[SessionsItem],
    sign_out_endpoints: &[&SignOutEndpointModule],
) -> Vec<GeneratedModuleTokens> {
    let endpoints = served_endpoints.iter().map(|endpoint| {
        let framework_path = endpoint.framework_path();

        quote! { pub use #framework_path; }
    });
    let sign_out_module = if sign_out_endpoints.is_empty() {
        TokenStream::new()
    } else {
        let name = format_ident!("{SIGN_OUT_ENDPOINTS_MODULE_NAME}");

        quote! { pub mod #name; }
    };

    match sessions {
        DeclaredSessions::Absent => Vec::new(),
        DeclaredSessions::Consumed(ConsumedSessionsDeclaration { .. }) => {
            let consumed = SessionsItem::ConsumedSessions.framework_path();

            [GeneratedModuleTokens::new(
                ModulePath::root(SESSIONS_MODULE_NAME),
                quote! {
                    #sign_out_module
                    pub use #consumed;
                    #(#endpoints)*
                },
            )]
            .into_iter()
            .chain(render_sign_out_endpoints(sign_out_endpoints))
            .collect()
        }
        DeclaredSessions::Issued(IssuedSessionsDeclaration { audience, .. }) => {
            let issued = SessionsItem::IssuedSessions.framework_path();
            let audience = audience.as_str();

            [
                GeneratedModuleTokens::new(
                    ModulePath::root(SESSIONS_MODULE_NAME),
                    quote! {
                        pub mod session_audience;
                        #sign_out_module
                        pub use #issued;
                        #(#endpoints)*
                    },
                ),
                GeneratedModuleTokens::new(
                    ModulePath::root(SESSIONS_MODULE_NAME).child("session_audience"),
                    quote! { pub const SESSION_AUDIENCE: &str = #audience; },
                ),
            ]
            .into_iter()
            .chain(render_sign_out_endpoints(sign_out_endpoints))
            .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use super::render_sessions;
    use crate::declared_sessions::DeclaredSessions;
    use crate::session_audience_path::session_audience_path;
    use crate::sessions_item::SessionsItem;
    use crate::sign_out_endpoint_module::SignOutEndpointModule;

    const ISSUED_SESSIONS: &str = "use margaret::framework::macros::issues_sessions;\n#[issues_sessions(issuer = provider, audience = \"browser\", cookies = margaret::framework::sessions::session_cookies::SessionCookies::HostOnly)]\nstruct BrowserSessions;\n";

    fn rendered(
        source: &str,
        served_endpoints: &[SessionsItem],
        sign_out_endpoints: &[&SignOutEndpointModule],
    ) -> Vec<String> {
        let indexed = IndexedSource::new(source);
        let issuance =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");
        let resources = DeclaredResourceIssuances::read(&indexed.index, &issuance)
            .expect("the resources are read");

        render_sessions(
            &DeclaredSessions::read(&indexed.index, &issuance, &resources)
                .expect("the sessions are read"),
            served_endpoints,
            sign_out_endpoints,
        )
        .into_iter()
        .map(|module| {
            format!(
                "{}:{}",
                module.path(),
                module.to_source().split_whitespace().collect::<String>()
            )
        })
        .collect()
    }

    #[test]
    fn renders_no_module_without_sessions_so_apps_without_them_carry_no_session_code() {
        assert!(rendered("", &[], &[]).is_empty());
    }

    #[test]
    fn renders_the_issued_sessions_with_their_audience_and_served_endpoints() {
        assert_eq!(
            rendered(
                ISSUED_SESSIONS,
                &[SessionsItem::SessionRefreshEndpoint],
                &[]
            ),
            [
                "sessions:pubmodsession_audience;pubusemargaret::framework::sessions::issued_sessions::IssuedSessions;pubusemargaret::framework::sessions::session_refresh_endpoint::SessionRefreshEndpoint;",
                "sessions/session_audience:pubconstSESSION_AUDIENCE:&str=\"browser\";",
            ]
        );
        assert_eq!(
            session_audience_path().to_string(),
            "crate::margaret::sessions::session_audience::SESSION_AUDIENCE"
        );
    }

    #[test]
    fn renders_consumed_sessions_as_a_lone_reexport_without_an_audience_of_their_own() {
        assert_eq!(
            rendered(
                "use margaret::framework::macros::consumes_sessions;\n#[consumes_sessions(issuer = partner, cookie_domain_from = \"SESSION_COOKIE_DOMAIN\", refresh_url_from = \"SESSION_REFRESH_URL\")]\nstruct PartnerSessions;\n",
                &[],
                &[],
            ),
            ["sessions:pubusemargaret::framework::sessions::consumed_sessions::ConsumedSessions;"]
        );
    }

    #[test]
    fn renders_a_handler_module_per_sign_out_route_so_each_route_lands_on_its_own_page() {
        let public = SignOutEndpointModule::new("post_sign_out".to_string());
        let staff = SignOutEndpointModule::new("post_staff_sign_out".to_string());

        assert_eq!(
            rendered(ISSUED_SESSIONS, &[], &[&public, &staff]),
            [
                "sessions:pubmodsession_audience;pubmodsign_out_endpoints;pubusemargaret::framework::sessions::issued_sessions::IssuedSessions;",
                "sessions/session_audience:pubconstSESSION_AUDIENCE:&str=\"browser\";",
                "sessions/sign_out_endpoints:pubmodpost_sign_out;pubmodpost_staff_sign_out;",
                "sessions/sign_out_endpoints/post_sign_out:pubmodsession_sign_out_endpoint;",
                "sessions/sign_out_endpoints/post_sign_out/session_sign_out_endpoint:pubusemargaret::framework::sessions::session_sign_out_endpoint::SessionSignOutEndpoint;",
                "sessions/sign_out_endpoints/post_staff_sign_out:pubmodsession_sign_out_endpoint;",
                "sessions/sign_out_endpoints/post_staff_sign_out/session_sign_out_endpoint:pubusemargaret::framework::sessions::session_sign_out_endpoint::SessionSignOutEndpoint;",
            ]
        );
    }
}
