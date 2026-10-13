use margaret_http::cookie_changes::CookieChanges;

pub struct SessionUserInference<TOutcome> {
    pub cookie_changes: CookieChanges,
    pub outcome: TOutcome,
}
