use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Map;
use serde_json::Value;

use margaret_claims_merge::merge_claims::merge_claims;
use margaret_http::no_store::no_store;
use margaret_http::response::Response;

use crate::answers_userinfo_grants::AnswersUserinfoGrants;
use crate::provider_error::ProviderError;
use crate::provides_userinfo_claims::ProvidesUserinfoClaims;
use crate::userinfo_claims::UserinfoClaims;
use crate::userinfo_grant::UserinfoGrant;

const SUBJECT_MEMBER: &str = "sub";

pub(crate) struct TypedUserinfoAnswer<TProvider> {
    pub(crate) provider: Arc<TProvider>,
}

#[async_trait]
impl<TProvider: ProvidesUserinfoClaims> AnswersUserinfoGrants for TypedUserinfoAnswer<TProvider> {
    async fn answered(&self, grant: &UserinfoGrant) -> Result<Response, ProviderError> {
        match self
            .provider
            .claims(grant)
            .await
            .map_err(ProviderError::UserinfoClaimsProvider)?
        {
            UserinfoClaims::Found(claims) => merge_claims(
                Map::from_iter([(
                    SUBJECT_MEMBER.to_string(),
                    Value::String(grant.subject.to_string()),
                )]),
                &claims,
            )
            .map_err(ProviderError::UserinfoClaims)
            .map(|members| no_store(Response::json(200, &Value::Object(members)))),
            UserinfoClaims::SubjectUnknown => Ok(Response::not_found()),
        }
    }
}
