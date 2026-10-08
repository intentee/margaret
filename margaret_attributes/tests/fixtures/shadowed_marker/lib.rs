pub fn index() {}

#[model(table = "notes")]
struct Note {
    #[index]
    title: String,
}
