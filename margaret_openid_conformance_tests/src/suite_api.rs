use reqwest::Client;
use reqwest::RequestBuilder;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use url::Url;

use crate::created_plan::CreatedPlan;
use crate::module_info::ModuleInfo;
use crate::module_outcome::ModuleOutcome;
use crate::module_status::ModuleStatus;
use crate::plan_module::PlanModule;

const CLIENT_TURN_STATUSES: &str = "WAITING,FINISHED,INTERRUPTED";
const LONG_POLL_MILLISECONDS: u32 = 30_000;
const RESULT_STATUSES: &str = "FINISHED,INTERRUPTED";

#[derive(Deserialize)]
struct CreatedModule {
    id: String,
}

#[derive(Serialize)]
struct ModuleQuery<'module> {
    plan: &'module str,
    test: &'module str,
    variant: String,
}

#[derive(Serialize)]
struct PlanQuery<'plan> {
    #[serde(rename = "planName")]
    plan_name: &'plan str,
    variant: String,
}

#[derive(Serialize)]
struct StatusQuery<'status> {
    states: &'status str,
    #[serde(rename = "timeoutMs")]
    timeout_ms: u32,
}

#[derive(Deserialize)]
struct ReachedStatus {
    state: ModuleStatus,
}

async fn answered<TAnswer: DeserializeOwned>(
    request: RequestBuilder,
    expected: StatusCode,
) -> TAnswer {
    let response = request.send().await.expect("the suite answers");

    assert_eq!(response.status(), expected, "the suite answers as expected");

    response.json().await.expect("the suite answers with json")
}

pub struct SuiteApi {
    pub base_url: Url,
    pub client: Client,
}

impl SuiteApi {
    /// # Panics
    ///
    /// Panics when the suite does not reach a turn of the client.
    pub async fn await_client_turn(&self, module: &str) -> ModuleStatus {
        self.await_status(module, CLIENT_TURN_STATUSES).await
    }

    /// # Panics
    ///
    /// Panics when the suite does not reach the result of the module.
    pub async fn await_result(&self, module: &str) -> ModuleStatus {
        self.await_status(module, RESULT_STATUSES).await
    }

    /// # Panics
    ///
    /// Panics when the suite refuses the module.
    pub async fn create_module(&self, plan: &str, module: &PlanModule) -> String {
        answered::<CreatedModule>(
            self.client.post(self.api("runner")).query(&ModuleQuery {
                plan,
                test: &module.test_module,
                variant: module.variant.to_string(),
            }),
            StatusCode::CREATED,
        )
        .await
        .id
    }

    /// # Panics
    ///
    /// Panics when the suite refuses the plan.
    pub async fn create_plan(
        &self,
        name: &str,
        variant: &Value,
        configuration: &Value,
    ) -> CreatedPlan {
        answered(
            self.client
                .post(self.api("plan"))
                .query(&PlanQuery {
                    plan_name: name,
                    variant: variant.to_string(),
                })
                .json(configuration),
            StatusCode::CREATED,
        )
        .await
    }

    /// # Panics
    ///
    /// Panics when the suite does not describe the module.
    pub async fn outcome(&self, module: &str) -> ModuleOutcome {
        ModuleOutcome {
            info: answered::<ModuleInfo>(
                self.client.get(self.api(&format!("info/{module}"))),
                StatusCode::OK,
            )
            .await,
            log: answered(
                self.client.get(self.api(&format!("log/{module}"))),
                StatusCode::OK,
            )
            .await,
        }
    }

    fn api(&self, path: &str) -> Url {
        self.base_url
            .join(&format!("api/{path}"))
            .expect("the suite api path joins its base url")
    }

    async fn await_status(&self, module: &str, statuses: &str) -> ModuleStatus {
        answered::<ReachedStatus>(
            self.client
                .get(self.api(&format!("runner/{module}/wait-state")))
                .query(&StatusQuery {
                    states: statuses,
                    timeout_ms: LONG_POLL_MILLISECONDS,
                }),
            StatusCode::OK,
        )
        .await
        .state
    }
}
