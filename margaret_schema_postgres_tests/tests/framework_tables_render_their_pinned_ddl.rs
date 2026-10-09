use margaret_model::render_postgres::render_postgres;
use margaret_model::schema::Schema;

const PINNED_FRAMEWORK_TABLES: &str = include_str!("framework_tables.sql");

#[test]
fn framework_tables_render_their_pinned_ddl() {
    assert_eq!(
        format!(
            "{}\n",
            render_postgres(&Schema {
                table_sets: &[
                    margaret_authorization_grants::margaret::tables::TABLES,
                    margaret_client_assertions::margaret::tables::TABLES,
                    margaret_signing_keys::margaret::tables::TABLES,
                ],
            })
        ),
        PINNED_FRAMEWORK_TABLES
    );
}
