use margaret::framework::macros::oauth_scope;

#[oauth_scope(name = "artifacts:read")]
pub struct ArtifactsReadScope;
