use spiffe::spiffe_id::SpiffeId;

use margaret::framework::http::request::Request;
use margaret::framework::identity::authenticated_user_presence::AuthenticatedUserPresence;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::authenticated_users::mesh_caller::MeshCaller;
use crate::auth::headers::mesh_role::MESH_ROLE;

#[singleton]
#[infers_authenticated_user(user_model = MeshCaller)]
pub struct MeshCallerProvider;

impl MeshCallerProvider {
    #[must_use]
    #[infer_from_request]
    pub fn infer_mesh_caller(
        &self,
        request: &Request,
        peer: &SpiffeId,
    ) -> AuthenticatedUserPresence<MeshCaller> {
        match request.inputs.server.header(&MESH_ROLE) {
            Some(role) => AuthenticatedUserPresence::Authenticated(MeshCaller {
                role: role.to_string(),
                spiffe_id: peer.to_string(),
            }),
            None => AuthenticatedUserPresence::Anonymous,
        }
    }
}
