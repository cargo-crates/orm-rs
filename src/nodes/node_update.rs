use serde_json::{Value as JsonValue};
use crate::nodes::NodeAble;
use crate::methods::full_column_name;

#[derive(Clone, Debug)]
pub struct NodeUpdate {
    condition: JsonValue
}
impl NodeUpdate {
    pub fn new(condition: JsonValue) -> Self {
        Self {
            condition
        }
    }
}
impl NodeAble for NodeUpdate {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_value(&self, table_name: &str) -> Vec<String> {
        let mut vec = vec![];
        if let JsonValue::Object(map_value) = self.get_condition() {
            for key in map_value.keys() {
                let full_column_name = full_column_name(key, table_name);
                if let Some(json_value) = map_value.get(key) {
                    match json_value {
                        JsonValue::String(value) => {
                            vec.push(format!("{} = '{}'", full_column_name, value));
                        },
                        JsonValue::Number(value) => {
                            vec.push(format!("{} = {}", full_column_name, value));
                        },
                        JsonValue::Bool(value) => {
                            let value = if *value {1} else {0};
                            vec.push(format!("{} = {}", full_column_name, value));
                        },
                        JsonValue::Null => {
                            vec.push(format!("{} = NULL", full_column_name));
                        },
                        _ => ()
                    }
                }
            }
        }
        // Ok(vec.join(" AND "))
        vec
    }
}