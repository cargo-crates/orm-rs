use serde_json::{Value as JsonValue};
use crate::nodes::NodeAble;

#[derive(Debug)]
pub struct NodeWhere {
    condition: JsonValue
}
impl NodeAble for NodeWhere {
    fn new(condition: JsonValue) -> Self {
        Self {
            condition
        }
    }
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
}