use rust_decimal::Decimal;
use sqlx::query;
use sqlx::query_as;
use uuid::Uuid;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::start_database::start_database;

const INSERT_LINE_ITEM: &str = "INSERT INTO line_items (id, price, discount, weight_kg, volume_litres) VALUES ($1, $2, $3, $4, $5)";

#[tokio::test]
async fn numeric_and_float_columns_keep_their_values() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let line_item_id = Uuid::new_v4();
    let price = Decimal::from_str_exact("1234567890.12").expect("the price literal parses");
    let discount = Decimal::from_str_exact("0.1235").expect("the discount literal parses");
    let weight_kg: f32 = 2.5;
    let volume_litres: f64 = 0.333_333_333_333_333_3;

    query(INSERT_LINE_ITEM)
        .bind(line_item_id)
        .bind(price)
        .bind(discount)
        .bind(weight_kg)
        .bind(volume_litres)
        .execute(pool)
        .await
        .expect("the line item row is inserted");

    let (stored_price, stored_discount, stored_weight_kg, stored_volume_litres): (
        Decimal,
        Option<Decimal>,
        f32,
        f64,
    ) = query_as("SELECT price, discount, weight_kg, volume_litres FROM line_items WHERE id = $1")
        .bind(line_item_id)
        .fetch_one(pool)
        .await
        .expect("the line item row is read back");

    assert_eq!(stored_price, price);
    assert_eq!(stored_discount, Some(discount));
    assert_eq!(stored_weight_kg.to_bits(), weight_kg.to_bits());
    assert_eq!(stored_volume_litres.to_bits(), volume_litres.to_bits());
}

#[tokio::test]
async fn a_nullable_numeric_column_accepts_no_value() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let line_item_id = Uuid::new_v4();

    query(INSERT_LINE_ITEM)
        .bind(line_item_id)
        .bind(Decimal::ONE)
        .bind(None::<Decimal>)
        .bind(1.0_f32)
        .bind(1.0_f64)
        .execute(pool)
        .await
        .expect("the line item row is inserted without a discount");

    let (stored_discount,): (Option<Decimal>,) =
        query_as("SELECT discount FROM line_items WHERE id = $1")
            .bind(line_item_id)
            .fetch_one(pool)
            .await
            .expect("the line item row is read back");

    assert_eq!(stored_discount, None);
}

#[tokio::test]
async fn the_declared_numeric_precision_rejects_an_oversized_value() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let error = query(INSERT_LINE_ITEM)
        .bind(Uuid::new_v4())
        .bind(Decimal::from_str_exact("12345678901.00").expect("the price literal parses"))
        .bind(None::<Decimal>)
        .bind(1.0_f32)
        .bind(1.0_f64)
        .execute(pool)
        .await
        .expect_err("a value wider than the declared precision is rejected");

    assert_eq!(
        error
            .as_database_error()
            .expect("Postgres reports a database error")
            .code()
            .expect("the database error carries an SQLSTATE"),
        "22003"
    );
}

#[tokio::test]
async fn the_declared_numeric_scale_rounds_a_longer_fraction() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let line_item_id = Uuid::new_v4();

    query(INSERT_LINE_ITEM)
        .bind(line_item_id)
        .bind(Decimal::from_str_exact("1.005").expect("the price literal parses"))
        .bind(None::<Decimal>)
        .bind(1.0_f32)
        .bind(1.0_f64)
        .execute(pool)
        .await
        .expect("the line item row is inserted");

    let (stored_price,): (Decimal,) = query_as("SELECT price FROM line_items WHERE id = $1")
        .bind(line_item_id)
        .fetch_one(pool)
        .await
        .expect("the line item row is read back");

    assert_eq!(
        stored_price,
        Decimal::from_str_exact("1.01").expect("the rounded literal parses")
    );
}
