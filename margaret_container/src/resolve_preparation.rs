use margaret_attributes::canonical_path::CanonicalPath;
use margaret_serve_input_codegen::preparation::Preparation;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
use margaret_token_issuance_codegen::token_issuance_path::token_issuance_path;

use crate::container_error::ContainerError;
use crate::preparation_source::PreparationSource;

fn prepared_argument(
    source: &PreparationSource,
    token_issuance: &DeclaredTokenIssuance,
    provided: &CanonicalPath,
) -> Result<CanonicalPath, ContainerError> {
    match source {
        PreparationSource::Constant(path) => Ok(path.clone()),
        PreparationSource::TokenIssuance => match token_issuance {
            DeclaredTokenIssuance::Absent => Err(ContainerError::MissingTokenIssuance {
                provider: provided.to_string(),
            }),
            DeclaredTokenIssuance::Declared(_) => Ok(token_issuance_path()),
        },
    }
}

pub(crate) fn resolve_preparation(
    function: CanonicalPath,
    prepared: CanonicalPath,
    sources: &[PreparationSource],
    token_issuance: &DeclaredTokenIssuance,
    provided: &CanonicalPath,
) -> Result<Preparation, ContainerError> {
    Ok(Preparation {
        arguments: sources
            .iter()
            .map(|source| prepared_argument(source, token_issuance, provided))
            .collect::<Result<Vec<CanonicalPath>, ContainerError>>()?,
        function,
        prepared,
    })
}
