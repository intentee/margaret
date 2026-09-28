use crate::presented_jws::PresentedJws;

pub(crate) enum PresentedToken<'request> {
    Absent,
    Bearer(PresentedJws<'request>),
    MalformedAuthorization,
    NotBearer,
}
