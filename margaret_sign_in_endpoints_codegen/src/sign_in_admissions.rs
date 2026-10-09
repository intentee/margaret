use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::sign_in_admission_declaration::SignInAdmissionDeclaration;
use crate::sign_in_client::sign_in_client;
use crate::sign_in_endpoints_codegen_error::SignInEndpointsCodegenError;

pub(crate) fn sign_in_admissions(
    index: &AttributeIndex,
) -> Result<Vec<SignInAdmissionDeclaration>, SignInEndpointsCodegenError> {
    index
        .select_framework_attribute(FrameworkAttribute::AdmitsSignIn)
        .map(|matched| {
            let admission = matched.item().canonical_path();

            matched.args()?.interpret(|reader| {
                Ok(SignInAdmissionDeclaration {
                    admission: admission.clone(),
                    client: sign_in_client(reader, admission)?,
                })
            })
        })
        .collect()
}
