use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::credit_grant::CreditGrant;
use crate::request_id::RequestId;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case", tag = "kind")]
pub enum ClientSentFrame<Params = Value> {
    Cancel {
        id: RequestId,
    },
    Credit {
        credit: CreditGrant,
        id: RequestId,
    },
    Notification {
        method: String,
        params: Params,
    },
    Request {
        credit: CreditGrant,
        id: RequestId,
        method: String,
        params: Params,
    },
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde_json::json;

    use super::ClientSentFrame;
    use crate::credit_grant::CreditGrant;
    use crate::request_id::RequestId;

    const FOUR_FRAMES: CreditGrant = CreditGrant::from_frames(4);
    const THREE_FRAMES: CreditGrant = CreditGrant::from_frames(3);
    const TWO_FRAMES: CreditGrant = CreditGrant::from_frames(2);

    fn read(frame: &Value) -> Option<ClientSentFrame> {
        serde_json::from_value(frame.clone()).ok()
    }

    #[test]
    fn reads_a_request_that_grants_its_window() {
        assert!(matches!(
            read(&json!({"kind": "request", "credit": 4, "id": 1, "method": "ping", "params": {}})),
            Some(ClientSentFrame::Request { credit: FOUR_FRAMES, ref id, .. })
                if *id == RequestId::Number(1)
        ));
    }

    #[test]
    fn reads_a_credit_grant() {
        assert!(matches!(
            read(&json!({"kind": "credit", "credit": 2, "id": 1})),
            Some(ClientSentFrame::Credit { credit: TWO_FRAMES, ref id })
                if *id == RequestId::Number(1)
        ));
    }

    #[test]
    fn reads_a_notification() {
        assert!(matches!(
            read(&json!({"kind": "notification", "method": "typing", "params": {}})),
            Some(ClientSentFrame::Notification { ref method, .. }) if method == "typing"
        ));
    }

    #[test]
    fn refuses_a_request_that_grants_no_window() {
        assert!(
            read(&json!({"kind": "request", "id": 1, "method": "ping", "params": {}})).is_none()
        );
    }

    #[test]
    fn refuses_a_frame_that_names_no_kind() {
        assert!(read(&json!({"credit": 2, "id": 1})).is_none());
    }

    #[test]
    fn refuses_a_request_that_grants_more_than_the_protocol_allows() {
        assert!(
            read(&json!({
                "kind": "request",
                "credit": CreditGrant::MAXIMUM + 1,
                "id": 1,
                "method": "ping",
                "params": {},
            }))
            .is_none()
        );
    }

    #[test]
    fn refuses_a_credit_grant_that_exceeds_the_protocol_maximum() {
        assert!(
            read(&json!({"kind": "credit", "credit": CreditGrant::MAXIMUM + 1, "id": 1})).is_none()
        );
    }

    #[test]
    fn reads_a_credit_grant_of_the_protocol_maximum() {
        assert!(matches!(
            read(&json!({"kind": "credit", "credit": CreditGrant::MAXIMUM, "id": 1})),
            Some(ClientSentFrame::Credit { credit, ref id })
                if credit.frames() == CreditGrant::MAXIMUM && *id == RequestId::Number(1)
        ));
    }

    #[test]
    fn writes_the_kind_it_reads_back() {
        let grant: ClientSentFrame = ClientSentFrame::Credit {
            credit: THREE_FRAMES,
            id: RequestId::Number(1),
        };

        assert_eq!(
            serde_json::to_value(grant).expect("a credit grant serializes"),
            json!({"kind": "credit", "credit": 3, "id": 1})
        );
    }
}
