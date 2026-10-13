use crate::consent_request::ConsentRequest;

pub struct ConsentPageProps<'page, TRoutes> {
    pub consent: &'page ConsentRequest,
    pub decision_url: &'page str,
    pub routes: &'page TRoutes,
}
