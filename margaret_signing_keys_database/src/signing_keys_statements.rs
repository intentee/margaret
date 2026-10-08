use margaret_model::qualified_framework_table::qualified_framework_table;

pub(crate) struct SigningKeysStatements {
    pub(crate) create: String,
    pub(crate) load: String,
    pub(crate) replace: String,
}

impl SigningKeysStatements {
    pub(crate) fn new() -> Self {
        let table = qualified_framework_table("signing_key_sets");

        Self {
            create: format!(
                "INSERT INTO {table} (name, generation, document) VALUES ($1, $2, $3) ON CONFLICT (name) DO NOTHING"
            ),
            load: format!("SELECT name, generation, document FROM {table} WHERE name = $1"),
            replace: format!(
                "UPDATE {table} SET generation = $3, document = $4 WHERE name = $1 AND generation = $2"
            ),
        }
    }
}
