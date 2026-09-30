use uuid::Uuid;

use crate::code_admission::CodeAdmission;
use crate::refresh_issuance::RefreshIssuance;

pub struct CodeRedemptionRequest<'request> {
    pub admission: CodeAdmission<'request>,
    pub family: Uuid,
    pub refresh: RefreshIssuance,
}
