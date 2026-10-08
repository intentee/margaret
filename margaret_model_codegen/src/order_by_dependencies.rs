use std::collections::BTreeMap;
use std::collections::BTreeSet;

use margaret_toposort::topological_order::topological_order;

use crate::model::Model;
use crate::model_codegen_error::ModelCodegenError;

pub(crate) fn order_by_dependencies(models: Vec<Model>) -> Result<Vec<Model>, ModelCodegenError> {
    let mut dependencies: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut by_table: BTreeMap<String, Model> = BTreeMap::new();

    for model in models {
        dependencies.insert(
            model.table.clone(),
            model
                .foreign_keys
                .iter()
                .map(|foreign_key| foreign_key.references_table.clone())
                .filter(|referenced| referenced != &model.table)
                .collect(),
        );
        by_table.insert(model.table.clone(), model);
    }

    let order =
        topological_order(&dependencies).map_err(|cycle| ModelCodegenError::ForeignKeyCycle {
            path: cycle.path.join(" -> "),
        })?;

    Ok(order
        .iter()
        .filter_map(|table| by_table.remove(table))
        .collect())
}
