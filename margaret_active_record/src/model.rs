use std::future::Future;

use margaret_database::executor::Executor;
use margaret_sql::assignment::Assignment;
use margaret_sql::assignments::Assignments;
use margaret_sql::delete::Delete;
use margaret_sql::render_delete::render_delete;
use margaret_sql::render_update::render_update;
use margaret_sql::returning::Returning;
use margaret_sql::update::Update;

use crate::active_record_error::ActiveRecordError;
use crate::base_alias::BASE_ALIAS;
use crate::deletion::Deletion;
use crate::encoded_values::encoded_values;
use crate::executed_affected::executed_affected;
use crate::field_set::FieldSet;
use crate::insert::Insert;
use crate::primary_key_clause::primary_key_clause;
use crate::record::Record;
use crate::record_encoding::RecordEncoding;
use crate::saving::Saving;
use crate::statement_kind::StatementKind;
use crate::table_source::table_source;

fn deletion(deleted: u64) -> Deletion {
    match deleted {
        0 => Deletion::Missing,
        _ => Deletion::Deleted,
    }
}

pub trait Model: Record {
    type Conditions: FieldSet;
    type Query: FieldSet;

    fn delete<Executing: Executor>(
        &self,
        executor: &Executing,
    ) -> impl Future<Output = Result<Deletion, ActiveRecordError>> + Send {
        let statement = primary_key_clause::<Self>(self.primary_key())
            .condition(BASE_ALIAS, Self::TABLE)
            .map(|condition| {
                render_delete(&Delete {
                    condition,
                    returning: Returning::Nothing,
                    target: table_source(Self::TABLE, BASE_ALIAS),
                })
            });

        async move {
            executed_affected(statement, StatementKind::Delete, Self::TABLE, executor)
                .await
                .map(deletion)
        }
    }

    #[must_use]
    fn insert(&self) -> Insert<'_, Self> {
        Insert::new(self)
    }

    #[must_use]
    fn query() -> Self::Query {
        FieldSet::fields()
    }

    fn save<Executing: Executor>(
        &self,
        executor: &Executing,
    ) -> impl Future<Output = Result<Saving, ActiveRecordError>> + Send {
        let statement =
            encoded_values(&RecordEncoding::new(self), Self::TABLE).and_then(|values| {
                primary_key_clause::<Self>(self.primary_key())
                    .condition(BASE_ALIAS, Self::TABLE)
                    .map(|condition| {
                        render_update(&Update {
                            assignments: Assignments {
                                first: Assignment {
                                    columns: Self::TABLE
                                        .columns
                                        .iter()
                                        .map(|column| column.name)
                                        .collect(),
                                    values,
                                },
                                rest: Vec::new(),
                            },
                            condition,
                            returning: Returning::Nothing,
                            target: table_source(Self::TABLE, BASE_ALIAS),
                        })
                    })
            });

        async move {
            executed_affected(statement, StatementKind::Update, Self::TABLE, executor)
                .await
                .map(|saved| match saved {
                    0 => Saving::Missing,
                    _ => Saving::Saved,
                })
        }
    }
}
