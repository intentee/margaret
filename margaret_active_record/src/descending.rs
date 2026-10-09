use margaret_sql::comparison::Comparison;
use margaret_sql::direction::Direction;

use crate::scan_direction::ScanDirection;

pub struct Descending;

impl ScanDirection for Descending {
    const CURSOR_COMPARISON: Comparison = Comparison::LessOrEqual;
    const DIRECTION: Direction = Direction::Descending;
}
