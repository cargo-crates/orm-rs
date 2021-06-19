use serde_json::{json, Value as JsonValue};
use crate::nodes::NodeAble;

#[derive(Clone, Debug)]
pub struct NodeBool {
    condition: JsonValue,
    value: bool,
    r#type: String
}
impl NodeBool {
    fn new(value: bool, r#type: &str) -> Self {
        Self {
            condition: json!([]),
            value,
            r#type: r#type.to_string()
        }
    }
    pub fn get_type(&self) -> &str {
        &self.r#type
    }
    pub fn new_distinct() -> Self {
        Self::new(true, "distinct")
    }
}
impl NodeAble for NodeBool {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_value(&self, _table_name: &str) -> Vec<String> {
        vec!["DISTINCT".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use serde_json::json;
    #[test]
    fn to_value() {
        let node_bool = NodeBool::new_distinct();
        assert_eq!(node_bool.to_value("users"), vec!["DISTINCT"]);
    }
}