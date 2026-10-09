use margaret::framework::macros::issues_resource_tokens;

#[issues_resource_tokens(notes, audience = "notes")]
pub struct NotesResource;
