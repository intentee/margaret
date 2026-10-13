use margaret_sql::comparison::Comparison;
use margaret_sql::direction::Direction;

use crate::scan_direction::ScanDirection;

pub struct Ascending;

impl ScanDirection for Ascending {
    const CURSOR_COMPARISON: Comparison = Comparison::GreaterOrEqual;
    const DIRECTION: Direction = Direction::Ascending;
}
