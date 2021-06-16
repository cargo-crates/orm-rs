use serde_json::{Value as JsonValue};
use crate::nodes::NodeAble;

#[derive(Debug)]
pub struct NodeExcept {
    condition: JsonValue
}
impl NodeExcept {
    pub fn new(condition: JsonValue) -> Self {
        Self {
            condition
        }
    }
}
impl NodeAble for NodeExcept {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_sql(&self, _table_name: &str) -> Vec<String> {
        vec![]
    }
}