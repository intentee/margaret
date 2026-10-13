use std::collections::HashMap;

use margaret::framework::http::forward::Forward;
use margaret_cluster_fixture::margaret::forwarders::public::Forwarder;

fn forward_to(name: &'static str, placeholders: &[(&str, &str)]) -> Forward {
    Forward::new(
        name,
        placeholders
            .iter()
            .map(|(placeholder, value)| ((*placeholder).to_string(), (*value).to_string()))
            .collect::<HashMap<String, String>>(),
    )
}

#[test]
fn forwards_to_every_named_public_route_by_its_name_and_placeholders() {
    assert_eq!(
        [
            Forwarder::default().get_asset("cluster.css".to_string()),
            Forwarder::default().get_external(),
            Forwarder::default().get_forwarded_note("forwarded".to_string()),
            Forwarder::default().get_health(),
            Forwarder::default().get_messages(),
            Forwarder::default().get_note("note".to_string()),
            Forwarder::default().get_note_view("viewed".to_string()),
            Forwarder::default().get_notes(),
            Forwarder::default().get_notes_caller(),
            Forwarder::default().get_profile(),
            Forwarder::default().get_sign_in(),
            Forwarder::default().get_sign_in_callback(),
            Forwarder::default().get_upload("upload".to_string()),
        ],
        [
            forward_to("get_asset", &[("asset_path", "cluster.css")]),
            forward_to("get_external", &[]),
            forward_to("get_forwarded_note", &[("note_id", "forwarded")]),
            forward_to("get_health", &[]),
            forward_to("get_messages", &[]),
            forward_to("get_note", &[("note", "note")]),
            forward_to("get_note_view", &[("note", "viewed")]),
            forward_to("get_notes", &[]),
            forward_to("get_notes_caller", &[]),
            forward_to("get_profile", &[]),
            forward_to("get_sign_in", &[]),
            forward_to("get_sign_in_callback", &[]),
            forward_to("get_upload", &[("upload", "upload")]),
        ]
    );
}
