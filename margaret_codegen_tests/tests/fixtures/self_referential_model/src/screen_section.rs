use margaret_macros::model;

#[model(table = "screen_sections")]
#[derive(Clone)]
pub struct ScreenSection {
    #[column(primary_key)]
    pub id: uuid::Uuid,
    #[column]
    #[foreign_key]
    pub parent: Option<Box<ScreenSection>>,
}
