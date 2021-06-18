use serde_json::{Value as JsonValue};
use crate::nodes::NodeAble;
use crate::methods::full_column_name;

#[derive(Debug)]
pub struct NodeGroup {
    condition: JsonValue
}
impl NodeGroup {
    pub fn new(condition: JsonValue) -> Self {
        Self {
            condition
        }
    }
}
impl NodeAble for NodeGroup {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_sql(&self, table_name: &str) -> Vec<String> {
        let mut vec = vec![];
        match self.get_condition() {
            JsonValue::Array(values) => {
                vec = values.iter().filter_map(|value| {
                    match value {
                        JsonValue::String(value) => Some(full_column_name(value, table_name)),
                        _ => None
                    }
                }).collect();
            },
            _ => ()
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn to_sql() {
        let node_group = NodeGroup::new(json!(["name", "age", "orders.type"]));
        assert_eq!(node_group.to_sql("users"), vec!["`users`.`name`", "`users`.`age`", "orders.type"]);
    }
}