use std::sync::Arc;

use margaret_sql::comparison::Comparison;

use crate::clause::Clause;
use crate::primary_key_columns::primary_key_columns;
use crate::record::Record;

pub(crate) fn primary_key_clause<Keyed: Record>(primary_key: Keyed::PrimaryKey) -> Clause {
    Clause::Compare {
        columns: primary_key_columns::<Keyed>(),
        comparison: Comparison::Equal,
        value: Arc::new(primary_key),
    }
}
