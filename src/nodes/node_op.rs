use serde_json::{json, Value as JsonValue};
use crate::nodes::NodeAble;
use crate::methods::full_column_name;

#[derive(Debug)]
pub struct NodeOp {
    condition: JsonValue,
    field: String,
    r#type: String
}
impl NodeOp {
    fn new(field: &str, r#type: &str) -> Self {
        Self {
            condition: json!([]),
            field: field.to_string(),
            r#type: r#type.to_string()
        }
    }
    pub fn get_field(&self) -> &str {
        &self.field
    }
    pub fn get_type(&self) -> &str {
        &self.r#type
    }
    pub fn new_count() -> Self {
        Self::new("*", "count")
    }
    pub fn new_sum(field: &str) -> Self {
        Self::new(field, "sum")
    }
    pub fn new_avg(field: &str) -> Self {
        Self::new(field, "avg")
    }
    pub fn new_min(field: &str) -> Self {
        Self::new(field, "min")
    }
    pub fn new_max(field: &str) -> Self {
        Self::new(field, "max")
    }
}
impl NodeAble for NodeOp {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_value(&self, table_name: &str) -> Vec<String> {
        let mut vec = vec![];
        vec.push(format!("{}", full_column_name(self.get_field(), table_name)));
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use serde_json::json;
    #[test]
    fn to_value() {
        let node_op = NodeOp::new_count();
        assert_eq!(node_op.to_value("users"), vec!["`users`.*"]);
    }
}