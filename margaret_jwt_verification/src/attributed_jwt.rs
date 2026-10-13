use std::marker::PhantomData;
use std::ops::ControlFlow;

use serde_json::Value;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::parameter_value::ParameterValue;
use margaret_registered_claims::registered_claims::RegisteredClaims;

use crate::jwt_profile::JwtProfile;
use crate::jwt_profiling::JwtProfiling;
use crate::profiled_jwt::ProfiledJwt;

pub struct AttributedJwt<'token> {
    pub(crate) application: Value,
    pub(crate) jws: CompactJws<'token>,
    pub(crate) registered: RegisteredClaims,
}

impl AttributedJwt<'_> {
    #[must_use]
    pub fn algorithm(&self) -> &ParameterValue<JwsAlgorithm> {
        self.jws.alg()
    }

    #[must_use]
    pub fn profile<TProfile: JwtProfile>(&self) -> JwtProfiling<'_, TProfile> {
        match TProfile::TOKEN_TYPE.check(self.jws.typ()) {
            ControlFlow::Continue(()) => JwtProfiling::Profiled(ProfiledJwt {
                jwt: self,
                profile: PhantomData,
            }),
            ControlFlow::Break(rejection) => JwtProfiling::Rejected(rejection),
        }
    }
}
