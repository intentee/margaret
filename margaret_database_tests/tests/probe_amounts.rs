use margaret_database::executor::Executor;

use crate::select_probe_amounts::select_probe_amounts;

pub async fn probe_amounts(executor: &impl Executor) -> Vec<i64> {
    executor
        .rows(&select_probe_amounts())
        .await
        .expect("the probe amounts are read")
        .iter()
        .map(|row| row.get(0))
        .collect()
}
