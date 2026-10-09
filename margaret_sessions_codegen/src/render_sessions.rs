use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::consumed_sessions_declaration::ConsumedSessionsDeclaration;
use crate::declared_sessions::DeclaredSessions;
use crate::issued_sessions_declaration::IssuedSessionsDeclaration;
use crate::sessions_item::SessionsItem;
use crate::sessions_module_name::SESSIONS_MODULE_NAME;

#[must_use]
pub fn render_sessions(
    sessions: &DeclaredSessions,
    served_endpoints: &[SessionsItem],
) -> Vec<GeneratedModuleTokens> {
    let endpoints = served_endpoints.iter().map(|endpoint| {
        let framework_path = endpoint.framework_path();

        quote! { pub use #framework_path; }
    });

    match sessions {
        DeclaredSessions::Absent => Vec::new(),
        DeclaredSessions::Consumed(ConsumedSessionsDeclaration { .. }) => {
            let consumed = SessionsItem::ConsumedSessions.framework_path();

            vec![GeneratedModuleTokens::new(
                SESSIONS_MODULE_NAME,
                quote! {
                    pub use #consumed;
                    #(#endpoints)*
                },
            )]
        }
        DeclaredSessions::Issued(IssuedSessionsDeclaration { audience, .. }) => {
            let issued = SessionsItem::IssuedSessions.framework_path();
            let audience = audience.as_str();

            vec![
                GeneratedModuleTokens::new(
                    SESSIONS_MODULE_NAME,
                    quote! {
                        pub mod session_audience;
                        pub use #issued;
                        #(#endpoints)*
                    },
                ),
                GeneratedModuleTokens::new(
                    format!("{SESSIONS_MODULE_NAME}/session_audience"),
                    quote! { pub const SESSION_AUDIENCE: &str = #audience; },
                ),
            ]
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

    fn rendered(source: &str, served_endpoints: &[SessionsItem]) -> Vec<String> {
        let indexed = IndexedSource::new(source);
        let issuance =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");
        let resources = DeclaredResourceIssuances::read(&indexed.index, &issuance)
            .expect("the resources are read");

        render_sessions(
            &DeclaredSessions::read(&indexed.index, &issuance, &resources)
                .expect("the sessions are read"),
            served_endpoints,
        )
        .into_iter()
        .map(|module| {
            format!(
                "{}:{}",
                module.name(),
                module.to_source().split_whitespace().collect::<String>()
            )
        })
        .collect()
    }

    #[test]
    fn renders_no_module_without_sessions() {
        assert!(rendered("", &[]).is_empty());
    }

    #[test]
    fn renders_the_issued_sessions_with_their_audience_and_served_endpoints() {
        assert_eq!(
            rendered(
                "#[issues_sessions(issuer = provider, audience = \"browser\", cookies = margaret::framework::sessions::session_cookies::SessionCookies::HostOnly)]\nstruct BrowserSessions;\n",
                &[
                    SessionsItem::SessionRefreshEndpoint,
                    SessionsItem::SessionSignOutEndpoint
                ],
            ),
            [
                "sessions:pubmodsession_audience;pubusemargaret::framework::sessions::issued_sessions::IssuedSessions;pubusemargaret::framework::sessions::session_refresh_endpoint::SessionRefreshEndpoint;pubusemargaret::framework::sessions::session_sign_out_endpoint::SessionSignOutEndpoint;",
                "sessions/session_audience:pubconstSESSION_AUDIENCE:&str=\"browser\";",
            ]
        );
        assert_eq!(
            session_audience_path().to_string(),
            "crate::margaret::sessions::session_audience::SESSION_AUDIENCE"
        );
    }

    #[test]
    fn renders_the_consumed_sessions() {
        assert_eq!(
            rendered(
                "#[consumes_sessions(issuer = partner, cookie_domain_from = \"SESSION_COOKIE_DOMAIN\", refresh_url_from = \"SESSION_REFRESH_URL\")]\nstruct PartnerSessions;\n",
                &[],
            ),
            ["sessions:pubusemargaret::framework::sessions::consumed_sessions::ConsumedSessions;"]
        );
    }
}
