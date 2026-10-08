use rust_decimal::Decimal;
use uuid::Uuid;

use margaret::framework::macros::model;

#[model(table = "line_items")]
pub struct LineItem {
    #[column(primary_key)]
    pub id: Uuid,
    #[column(precision = 12, scale = 2)]
    pub price: Decimal,
    #[column(precision = 5, scale = 4)]
    pub discount: Option<Decimal>,
    #[column]
    pub weight_kg: f32,
    #[column]
    pub volume_litres: f64,
}
