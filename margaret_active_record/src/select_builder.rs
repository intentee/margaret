use margaret_sql::comparison::Comparison;
use margaret_sql::condition::Condition;
use margaret_sql::expression::Expression;
use margaret_sql::join::Join;
use margaret_sql::table_alias::TableAlias;

use crate::field_columns::field_columns;
use crate::field_span::FieldSpan;
use crate::joined_source::JoinedSource;
use crate::loadable::Loadable;
use crate::primary_key_columns::primary_key_columns;
use crate::record::Record;
use crate::record_columns::record_columns;
use crate::table_source::table_source;

fn column_expressions(columns: &[&'static str], alias: TableAlias) -> Vec<Expression> {
    columns
        .iter()
        .map(|column| Expression::Column { alias, column })
        .collect()
}

pub struct SelectBuilder {
    pub(crate) columns: Vec<Expression>,
    pub(crate) joins: Vec<Join>,
    next_alias: usize,
}

impl SelectBuilder {
    pub(crate) fn new(first_alias: TableAlias) -> Self {
        Self {
            columns: Vec::new(),
            joins: Vec::new(),
            next_alias: first_alias.position + 1,
        }
    }

    pub fn belongs_to<Related: Loadable, Parent: Record>(
        &mut self,
        parent: JoinedSource,
        key: FieldSpan,
    ) {
        let joined = JoinedSource {
            alias: self.alias(),
            context: Related::JOIN.within(parent.context),
        };

        self.join::<Related::Root>(
            joined,
            parent.alias,
            &field_columns(Parent::TABLE, key.start, key.width),
        );
        Related::select(self, joined);
    }

    pub fn primary_key<Keyed: Record>(&mut self, source: JoinedSource) {
        self.columns.extend(column_expressions(
            &primary_key_columns::<Keyed>(),
            source.alias,
        ));
    }

    pub fn record<Selected: Record>(&mut self, source: JoinedSource) {
        self.columns
            .extend(record_columns(Selected::TABLE, source.alias));
    }

    pub(crate) fn alias(&mut self) -> TableAlias {
        let alias = TableAlias {
            position: self.next_alias,
        };

        self.next_alias += 1;
        alias
    }

    pub(crate) fn join<Joined: Record>(
        &mut self,
        joined: JoinedSource,
        parent_alias: TableAlias,
        parent_columns: &[&'static str],
    ) {
        let on = Condition::RowCompare {
            comparison: Comparison::Equal,
            left: column_expressions(&primary_key_columns::<Joined>(), joined.alias),
            right: column_expressions(parent_columns, parent_alias),
        };
        let source = table_source(Joined::TABLE, joined.alias);

        self.joins.push(joined.context.join(on, source));
    }
}
