use persistence::Persisted;

#[derive(Persisted)]
#[model(table = "notes")]
struct Note;
