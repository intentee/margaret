use margaret::framework::macros::issues_resource_tokens;

#[issues_resource_tokens(reports, audience = "reports")]
pub struct ReportsResource;
