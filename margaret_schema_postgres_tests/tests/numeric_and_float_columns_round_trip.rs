use rust_decimal::Decimal;
use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret::framework::database::database::Database;
use margaret_schema_postgres_fixture::line_item::LineItem;

use crate::started_with_fixture::started_with_fixture;

fn decimal(literal: &str) -> Decimal {
    Decimal::from_str_exact(literal).expect("the decimal literal parses")
}

fn line_item(price: Decimal, discount: Option<Decimal>) -> LineItem {
    LineItem {
        id: Uuid::new_v4(),
        price,
        discount,
        weight_kg: 2.5,
        volume_litres: 0.333_333_333_333_333_3,
    }
}

async fn inserted(database: &Database, item: &LineItem) {
    item.insert()
        .run(database)
        .await
        .expect("the line item is inserted");
}

async fn stored(database: &Database, id: Uuid) -> Lookup<LineItem> {
    LineItem::query()
        .id
        .eq(id)
        .find(database)
        .await
        .expect("the line item is read back")
}

#[tokio::test]
async fn numeric_and_float_columns_keep_their_values() {
    let started = started_with_fixture().await;
    let database = started.database.as_ref();
    let item = line_item(decimal("1234567890.12"), Some(decimal("0.1235")));

    inserted(database, &item).await;

    assert_eq!(stored(database, item.id).await, Lookup::Found(item));
}

#[tokio::test]
async fn a_nullable_numeric_column_accepts_no_value() {
    let started = started_with_fixture().await;
    let database = started.database.as_ref();
    let item = line_item(Decimal::ONE, None);

    inserted(database, &item).await;

    assert_eq!(stored(database, item.id).await, Lookup::Found(item));
}

#[tokio::test]
async fn the_declared_numeric_precision_rejects_an_oversized_value() {
    let started = started_with_fixture().await;

    assert!(matches!(
        line_item(decimal("12345678901.00"), None)
            .insert()
            .run(started.database.as_ref())
            .await,
        Err(ActiveRecordError::NumericValueOutOfRange {
            statement: StatementKind::Insert,
            table: "line_items",
            ..
        })
    ));
}

#[tokio::test]
async fn the_declared_numeric_scale_rounds_a_longer_fraction() {
    let started = started_with_fixture().await;
    let database = started.database.as_ref();
    let item = line_item(decimal("1.005"), None);

    inserted(database, &item).await;

    assert_eq!(
        stored(database, item.id).await,
        Lookup::Found(LineItem {
            price: decimal("1.01"),
            ..item
        })
    );
}
