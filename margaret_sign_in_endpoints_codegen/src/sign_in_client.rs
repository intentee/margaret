use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;

use crate::sign_in_endpoints_codegen_error::SignInEndpointsCodegenError;

pub(crate) fn sign_in_client(
    reader: &mut AttributeArgumentsReader,
    anchor: &CanonicalPath,
) -> Result<Tag, SignInEndpointsCodegenError> {
    let written = reader.take_path("client")?.ok_or_else(|| {
        SignInEndpointsCodegenError::MissingSignInClient {
            anchor: anchor.to_string(),
        }
    })?;

    Tag::from_path(&written).ok_or_else(|| SignInEndpointsCodegenError::MalformedSignInClient {
        anchor: anchor.to_string(),
    })
}
