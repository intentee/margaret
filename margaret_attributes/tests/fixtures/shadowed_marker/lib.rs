use margaret::framework::macros::model;

pub fn index() {}

#[model(table = "notes")]
struct Note {
    #[index]
    title: String,
}
