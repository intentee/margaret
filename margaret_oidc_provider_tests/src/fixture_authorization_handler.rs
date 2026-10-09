use std::sync::Arc;

use margaret_oidc_provider::authorization_handler::AuthorizationHandler;
use margaret_oidc_provider::consent_page_props::ConsentPageProps;
use margaret_views::renders_view::RendersView;

use crate::fixture_routes::FixtureRoutes;
use crate::provider_fixture::ProviderFixture;

#[must_use]
pub fn fixture_authorization_handler<TConsentView>(
    fixture: &ProviderFixture,
    consent_view: TConsentView,
) -> AuthorizationHandler<TConsentView, FixtureRoutes>
where
    TConsentView:
        for<'page> RendersView<Props<'page> = ConsentPageProps<'page, FixtureRoutes>> + Send + Sync,
{
    AuthorizationHandler::create(
        Arc::clone(&fixture.authorization),
        Arc::clone(&fixture.sessions),
        Arc::new(consent_view),
        "https://issuer.fixture/authorize/consent".to_string(),
        Arc::new(FixtureRoutes {
            home_url: "https://issuer.fixture/",
        }),
    )
}
