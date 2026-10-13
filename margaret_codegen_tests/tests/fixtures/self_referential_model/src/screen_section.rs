use margaret::framework::active_record::key::Key;
use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

#[model(table = "screen_sections")]
#[index(name = "screen_sections_by_parent", fields = [parent, id])]
#[derive(Clone)]
pub struct ScreenSection {
    #[column(primary_key)]
    pub id: uuid::Uuid,
    #[column]
    #[foreign_key(on_delete = OnDelete::SetNull)]
    pub parent: Option<Key<ScreenSection>>,
}
