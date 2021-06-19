use serde_json::{Value as JsonValue};
use regex::Regex;
use crate::nodes::NodeAble;
use crate::methods::full_column_name;

#[derive(Debug)]
pub struct NodeOrder {
    condition: JsonValue
}
impl NodeOrder {
    pub fn new(condition: JsonValue) -> Self {
        Self {
            condition
        }
    }
}
impl NodeAble for NodeOrder {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_value(&self, table_name: &str) -> Vec<String> {
        let mut vec = vec![];
        match self.get_condition() {
            JsonValue::Object(map) => {
                let keys = map.keys();
                for column in keys {
                    if let Some(json_value) = map.get(column) {
                        match json_value {
                            JsonValue::String(value) => {
                                vec.push(format!("{} {}", full_column_name(column, table_name), value.to_uppercase()));
                            },
                            _ => ()
                        }
                    }
                }
            },
            JsonValue::Array(values) => {
                for value in values {
                    match value {
                        JsonValue::String(string) => {
                            if Regex::new(r"(?i)\s(ASC|DESC)$").unwrap().is_match(string) {
                                vec.push(format!("{}", string));
                            } else {
                                vec.push(format!("{} ASC", string));
                            }
                        },
                        _ => ()
                    }
                }
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
    fn to_value() {
        let node_order = NodeOrder::new(json!(["name", "age DESC"]));
        assert_eq!(node_order.to_value("users"), vec!["name ASC", "age DESC"]);
        let node_order = NodeOrder::new(json!({
            "age": "desc",
            "name": "asc"
        }));
        assert_eq!(node_order.to_value("users"), vec!["`users`.`age` DESC", "`users`.`name` ASC"]);
    }
}