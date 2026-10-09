use crate::resolved_session::ResolvedSession;
use crate::session_unavailability::SessionUnavailability;

pub enum SessionResolution {
    Resolved(ResolvedSession),
    Unavailable(SessionUnavailability),
}
