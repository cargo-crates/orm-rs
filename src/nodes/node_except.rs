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
        let mut vec = vec![];
        match self.get_condition() {
            JsonValue::Array(json_array) => {
                let columns: Vec<_> = json_array.into_iter().filter_map(|value| {
                    if let JsonValue::String(value) = value { Some(value) } else { None }
                }).collect();
                for val in columns {
                    match val.to_lowercase().as_ref() {
                        "where" => { vec.push("where".to_string()) },
                        "group" => { vec.push("group".to_string()) },
                        "having" => { vec.push("having".to_string()) },
                        "order" => { vec.push("order".to_string()) },
                        "distinct" => { vec.push("distinct".to_string()) },
                        _ => {}
                    }
                }
            }
            _ => ()
        }
        vec
    }
}