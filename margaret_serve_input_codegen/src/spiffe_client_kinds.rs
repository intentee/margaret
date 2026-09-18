use std::collections::BTreeSet;

use crate::serve_input::ServeInput;
use crate::spiffe_client_kind::SpiffeClientKind;

#[must_use]
pub fn spiffe_client_kinds(inputs: &[ServeInput]) -> BTreeSet<SpiffeClientKind> {
    inputs
        .iter()
        .filter_map(ServeInput::spiffe_client_kind)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::spiffe_client_kinds;
    use crate::serve_input::ServeInput;
    use crate::spiffe_client_kind::SpiffeClientKind;

    #[test]
    fn collects_every_distinct_spiffe_client() {
        assert_eq!(
            spiffe_client_kinds(&[
                ServeInput::SpiffeWebSocketClient,
                ServeInput::SpiffeHttpClient,
                ServeInput::SpiffeWebSocketClient,
            ]),
            BTreeSet::from([SpiffeClientKind::Http, SpiffeClientKind::WebSocket])
        );
    }

    #[test]
    fn collects_nothing_when_no_input_is_a_spiffe_client() {
        assert!(spiffe_client_kinds(&[]).is_empty());
    }
}
