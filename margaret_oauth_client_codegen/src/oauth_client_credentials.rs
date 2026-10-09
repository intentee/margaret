use margaret_attributes::tag::Tag;
use margaret_oauth_vocabulary::resource_scope::ResourceScope;

pub enum OAuthClientCredentials<'declarations> {
    PerResource {
        resources: Vec<&'declarations Tag>,
        scopes: &'declarations [ResourceScope],
    },
    Targeted,
    Withheld,
}
