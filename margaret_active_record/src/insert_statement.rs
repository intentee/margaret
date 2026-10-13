use margaret_model::table::Table;
use margaret_sql::conflict_action::ConflictAction;
use margaret_sql::insert::Insert;
use margaret_sql::insert_source::InsertSource;
use margaret_sql::insert_value::InsertValue;
use margaret_sql::render_insert::render_insert;
use margaret_sql::returning::Returning;
use margaret_sql::statement::Statement;

use crate::base_alias::BASE_ALIAS;
use crate::table_source::table_source;

pub(crate) struct InsertStatement {
    pub(crate) conflict: ConflictAction,
    pub(crate) returning: Returning,
    pub(crate) source: InsertSource,
}

impl InsertStatement {
    pub(crate) fn render(self, table: &'static Table, values: Vec<InsertValue>) -> Statement {
        let Self {
            conflict,
            returning,
            source,
        } = self;

        render_insert(&Insert {
            conflict,
            returning,
            source,
            target: table_source(table, BASE_ALIAS),
            values,
        })
    }
}
