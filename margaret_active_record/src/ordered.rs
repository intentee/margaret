use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::value::Value;

pub trait Ordered: Value {}

impl Ordered for i32 {}
impl Ordered for i64 {}
impl Ordered for f32 {}
impl Ordered for f64 {}
impl Ordered for String {}
impl Ordered for Vec<u8> {}
impl Ordered for Uuid {}
impl Ordered for DateTime<Utc> {}
impl Ordered for Decimal {}
