use margaret::framework::macros::issues_resource_tokens;

#[issues_resource_tokens(attachments, audience = "attachments")]
pub struct AttachmentsResource;
