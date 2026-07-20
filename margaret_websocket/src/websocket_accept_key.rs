use tokio_tungstenite::tungstenite::handshake::derive_accept_key;

fn contains_token(value: Option<&str>, token: &str) -> bool {
    match value {
        Some(text) => text
            .split(',')
            .any(|element| element.trim().eq_ignore_ascii_case(token)),
        None => false,
    }
}

#[must_use]
pub fn websocket_accept_key(
    upgrade: Option<&str>,
    connection: Option<&str>,
    version: Option<&str>,
    key: Option<&str>,
) -> Option<String> {
    if !contains_token(upgrade, "websocket") {
        return None;
    }

    if !contains_token(connection, "upgrade") {
        return None;
    }

    if version != Some("13") {
        return None;
    }

    Some(derive_accept_key(key?.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::websocket_accept_key;

    #[test]
    fn derives_the_accept_key_for_a_valid_handshake() {
        assert_eq!(
            websocket_accept_key(
                Some("websocket"),
                Some("keep-alive, Upgrade"),
                Some("13"),
                Some("dGhlIHNhbXBsZSBub25jZQ==")
            ),
            Some("s3pPLMBiTxaQ9kYGzzhZRbK+xOo=".to_owned())
        );
    }

    #[test]
    fn rejects_a_request_without_the_websocket_upgrade_token() {
        assert_eq!(
            websocket_accept_key(
                Some("h2c"),
                Some("Upgrade"),
                Some("13"),
                Some("dGhlIHNhbXBsZSBub25jZQ==")
            ),
            None
        );
    }

    #[test]
    fn rejects_a_request_without_an_upgrade_header() {
        assert_eq!(
            websocket_accept_key(
                None,
                Some("Upgrade"),
                Some("13"),
                Some("dGhlIHNhbXBsZSBub25jZQ==")
            ),
            None
        );
    }

    #[test]
    fn rejects_a_request_without_the_upgrade_connection_token() {
        assert_eq!(
            websocket_accept_key(
                Some("websocket"),
                Some("keep-alive"),
                Some("13"),
                Some("dGhlIHNhbXBsZSBub25jZQ==")
            ),
            None
        );
    }

    #[test]
    fn rejects_a_request_with_an_unsupported_version() {
        assert_eq!(
            websocket_accept_key(
                Some("websocket"),
                Some("Upgrade"),
                Some("8"),
                Some("dGhlIHNhbXBsZSBub25jZQ==")
            ),
            None
        );
    }

    #[test]
    fn rejects_a_request_without_a_key() {
        assert_eq!(
            websocket_accept_key(Some("websocket"), Some("Upgrade"), Some("13"), None),
            None
        );
    }
}
