use spiffe::spiffe_id::SpiffeId;

use margaret_peer_identity::peer_identity::PeerIdentity;

use crate::request::Request;
use crate::response::Response;

pub fn require_peer_spiffe_id(request: &Request) -> Result<&SpiffeId, Response> {
    match request.peer_identity() {
        PeerIdentity::Verified { spiffe_id } => Ok(spiffe_id),
        PeerIdentity::Anonymous | PeerIdentity::Unidentified => Err(Response::forbidden()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use http::Method;
    use spiffe::spiffe_id::SpiffeId;
    use tokio_util::sync::CancellationToken;

    use margaret_peer_identity::peer_identity::PeerIdentity;

    use super::require_peer_spiffe_id;
    use crate::request::Request;

    fn request_with_peer_identity(peer_identity: PeerIdentity) -> Request {
        Request::new(Method::GET, "/".to_string(), CancellationToken::new())
            .with_peer_identity(Arc::new(peer_identity))
    }

    #[test]
    fn yields_the_spiffe_id_of_a_verified_peer() {
        let spiffe_id =
            SpiffeId::new("spiffe://example.org/workload").expect("the SPIFFE ID parses");
        let request = request_with_peer_identity(PeerIdentity::Verified {
            spiffe_id: spiffe_id.clone(),
        });

        assert!(matches!(
            require_peer_spiffe_id(&request),
            Ok(verified_spiffe_id) if verified_spiffe_id == &spiffe_id
        ));
    }

    #[test]
    fn forbids_an_anonymous_peer() {
        let request = request_with_peer_identity(PeerIdentity::Anonymous);

        assert_eq!(
            require_peer_spiffe_id(&request)
                .expect_err("an anonymous peer is forbidden")
                .status(),
            403
        );
    }

    #[test]
    fn forbids_an_unidentified_peer() {
        let request = request_with_peer_identity(PeerIdentity::Unidentified);

        assert_eq!(
            require_peer_spiffe_id(&request)
                .expect_err("an unidentified peer is forbidden")
                .status(),
            403
        );
    }
}
