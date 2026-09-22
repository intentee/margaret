use serde_json::Value;
use serde_json::json;

use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;
use margaret_websocket_envelope::credit_grant::CreditGrant;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_tests::client_request_frame::client_request_frame;
use margaret_websocket_tests::driver_harness::DriverHarness;
use margaret_websocket_tests::test_dispatch_table::test_dispatch_table;

const ONE_FRAME: CreditGrant = CreditGrant::from_frames(1);
const SILENT_EXCHANGE: i64 = 4;
const WHOLE_WINDOW: CreditGrant = CreditGrant::from_frames(u16::MAX);

fn credit_frame(credit: CreditGrant) -> String {
    serde_json::to_string(&ClientSentFrame::<()>::Credit {
        credit,
        id: RequestId::Number(SILENT_EXCHANGE),
    })
    .expect("a credit frame serializes")
}

fn silent_request(credit: CreditGrant) -> String {
    client_request_frame(
        credit,
        RequestId::Number(SILENT_EXCHANGE),
        "silent",
        json!({"label": "held open"}),
    )
}

#[tokio::test]
async fn reports_a_grant_that_would_pass_the_credit_window() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness.send(&silent_request(WHOLE_WINDOW)).await;
    harness.send(&credit_frame(ONE_FRAME)).await;

    let refused: Value =
        serde_json::from_str(&harness.recv().await).expect("the error response is valid JSON");

    assert_eq!(
        refused,
        json!({
            "kind": "error",
            "error": {
                "code": "credit_window_exceeded",
                "details": null,
                "message":
                    "the grant would take this exchange past the credit window the protocol allows"
            },
            "id": SILENT_EXCHANGE
        })
    );

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn closes_on_a_request_that_grants_more_than_the_protocol_allows() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(
            &json!({
                "kind": "request",
                "credit": CreditGrant::MAXIMUM + 1,
                "id": SILENT_EXCHANGE,
                "method": "silent",
                "params": {"label": "held open"},
            })
            .to_string(),
        )
        .await;
    harness.driver.await.expect("the driver finishes");
}
