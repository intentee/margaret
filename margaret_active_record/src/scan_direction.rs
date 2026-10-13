use margaret_sql::comparison::Comparison;
use margaret_sql::direction::Direction;

pub trait ScanDirection: 'static {
    const CURSOR_COMPARISON: Comparison;
    const DIRECTION: Direction;
}
