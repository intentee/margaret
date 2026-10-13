use crate::key_set_refresh::KeySetRefresh;

pub(crate) enum RefreshProgress {
    Finished(KeySetRefresh),
    Pending,
}
