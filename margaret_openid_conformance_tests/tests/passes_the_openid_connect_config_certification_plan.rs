use serde_json::json;

use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_openid_conformance_tests::conformance_suite::ConformanceSuite;
use margaret_openid_conformance_tests::docker_bridge_gateway::docker_bridge_gateway;
use margaret_openid_conformance_tests::module_result::ModuleResult;
use margaret_openid_conformance_tests::module_status::ModuleStatus;
use margaret_openid_conformance_tests::provider_host::PROVIDER_HOST;

#[tokio::test(flavor = "multi_thread")]
async fn passes_the_openid_connect_config_certification_plan() {
    let provider = ProviderFixture::published(
        format!("https://{PROVIDER_HOST}")
            .parse()
            .expect("the provider issuer is an https url"),
        docker_bridge_gateway().await,
    )
    .await;
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
                    "discoveryUrl": format!("https://{PROVIDER_HOST}/.well-known/openid-configuration"),
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
