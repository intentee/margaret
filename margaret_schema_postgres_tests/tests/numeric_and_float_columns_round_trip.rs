use rust_decimal::Decimal;
use tokio_postgres::error::SqlState;
use uuid::Uuid;

use margaret_database_tests::apply_schema::apply_schema;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_schema_postgres_fixture::margaret::schema::schema;

const INSERT_LINE_ITEM: &str = "INSERT INTO line_items (id, price, discount, weight_kg, volume_litres) VALUES ($1, $2, $3, $4, $5)";

#[tokio::test]
async fn numeric_and_float_columns_keep_their_values() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");
    let line_item_id = Uuid::new_v4();
    let price = Decimal::from_str_exact("1234567890.12").expect("the price literal parses");
    let discount = Decimal::from_str_exact("0.1235").expect("the discount literal parses");
    let weight_kg: f32 = 2.5;
    let volume_litres: f64 = 0.333_333_333_333_333_3;

    client
        .execute(
            INSERT_LINE_ITEM,
            &[&line_item_id, &price, &discount, &weight_kg, &volume_litres],
        )
        .await
        .expect("the line item row is inserted");

    let line_item = client
        .query_one(
            "SELECT price, discount, weight_kg, volume_litres FROM line_items WHERE id = $1",
            &[&line_item_id],
        )
        .await
        .expect("the line item row is read back");

    assert_eq!(line_item.get::<_, Decimal>("price"), price);
    assert_eq!(
        line_item.get::<_, Option<Decimal>>("discount"),
        Some(discount)
    );
    assert_eq!(
        line_item.get::<_, f32>("weight_kg").to_bits(),
        weight_kg.to_bits()
    );
    assert_eq!(
        line_item.get::<_, f64>("volume_litres").to_bits(),
        volume_litres.to_bits()
    );
}

#[tokio::test]
async fn a_nullable_numeric_column_accepts_no_value() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");
    let line_item_id = Uuid::new_v4();

    client
        .execute(
            INSERT_LINE_ITEM,
            &[
                &line_item_id,
                &Decimal::ONE,
                &None::<Decimal>,
                &1.0_f32,
                &1.0_f64,
            ],
        )
        .await
        .expect("the line item row is inserted without a discount");

    assert_eq!(
        client
            .query_one(
                "SELECT discount FROM line_items WHERE id = $1",
                &[&line_item_id],
            )
            .await
            .expect("the line item row is read back")
            .get::<_, Option<Decimal>>("discount"),
        None
    );
}

#[tokio::test]
async fn the_declared_numeric_precision_rejects_an_oversized_value() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");

    let rejection = client
        .execute(
            INSERT_LINE_ITEM,
            &[
                &Uuid::new_v4(),
                &Decimal::from_str_exact("12345678901.00").expect("the price literal parses"),
                &None::<Decimal>,
                &1.0_f32,
                &1.0_f64,
            ],
        )
        .await
        .expect_err("a value wider than the declared precision is rejected");

    assert_eq!(
        rejection.code(),
        Some(&SqlState::NUMERIC_VALUE_OUT_OF_RANGE)
    );
}

#[tokio::test]
async fn the_declared_numeric_scale_rounds_a_longer_fraction() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");
    let line_item_id = Uuid::new_v4();

    client
        .execute(
            INSERT_LINE_ITEM,
            &[
                &line_item_id,
                &Decimal::from_str_exact("1.005").expect("the price literal parses"),
                &None::<Decimal>,
                &1.0_f32,
                &1.0_f64,
            ],
        )
        .await
        .expect("the line item row is inserted");

    assert_eq!(
        client
            .query_one(
                "SELECT price FROM line_items WHERE id = $1",
                &[&line_item_id]
            )
            .await
            .expect("the line item row is read back")
            .get::<_, Decimal>("price"),
        Decimal::from_str_exact("1.01").expect("the rounded literal parses")
    );
}
