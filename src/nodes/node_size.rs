use serde_json::{json, Value as JsonValue};
use crate::nodes::NodeAble;

#[derive(Clone, Debug)]
pub struct NodeSize {
    condition: JsonValue,
    value: usize,
    r#type: String
}
impl NodeSize {
    fn new(value: usize, r#type: &str) -> Self {
        Self {
            condition: json!([]),
            value,
            r#type: r#type.to_string()
        }
    }
    pub fn get_type(&self) -> &str {
        &self.r#type
    }
    pub fn new_limit(value: usize) -> Self {
        Self::new(value, "limit")
    }
    pub fn new_offset(value: usize) -> Self { Self::new(value, "offset") }
}
impl NodeAble for NodeSize {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_value(&self, _table_name: &str) -> Vec<String> {
        match self.get_type() {
            "limit" => vec![format!("LIMIT {}", self.value)],
            "offset" => vec![format!("OFFSET {}", self.value)],
            _ => vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use serde_json::json;
    #[test]
    fn to_value() {
        let node_size = NodeSize::new_limit(5);
        assert_eq!(node_size.to_value("users"), vec!["LIMIT 5"]);
        let node_size = NodeSize::new_offset(5);
        assert_eq!(node_size.to_value("users"), vec!["OFFSET 5"]);
    }
}