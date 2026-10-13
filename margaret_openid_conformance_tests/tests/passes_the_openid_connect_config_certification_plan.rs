use serde_json::json;

use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_openid_conformance_tests::conformance_suite::ConformanceSuite;
use margaret_openid_conformance_tests::docker_bridge_gateway::docker_bridge_gateway;
use margaret_openid_conformance_tests::module_result::ModuleResult;
use margaret_openid_conformance_tests::module_status::ModuleStatus;
use margaret_openid_conformance_tests::published_provider::PUBLISHED_PROVIDER;

#[tokio::test(flavor = "multi_thread")]
async fn passes_the_openid_connect_config_certification_plan() {
    let provider =
        ProviderFixture::published(&PUBLISHED_PROVIDER, docker_bridge_gateway().await).await;
    let suite = ConformanceSuite::relaying_to(provider.server.port()).await;
    let plan = suite
        .api
        .create_plan(
            "oidcc-config-certification-test-plan",
            &json!({}),
            &json!({
                "alias": "margaret-provider",
                "description": "margaret provider",
                "server": {
                    "discoveryUrl": PUBLISHED_PROVIDER.location.discovery_url,
                },
            }),
        )
        .await;

    for module in &plan.modules {
        let id = suite.api.create_module(&plan.id, module).await;

        assert_eq!(suite.api.await_result(&id).await, ModuleStatus::Finished);

        let outcome = suite.api.outcome(&id).await;

        assert_eq!(
            outcome.info.result,
            ModuleResult::Passed,
            "{} {:#}",
            module.test_module,
            outcome.log
        );
    }

    suite.stop();
    provider.stop().await;
}
