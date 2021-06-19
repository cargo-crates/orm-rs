use serde_json::{Value as JsonValue};
use crate::nodes::NodeAble;
use crate::methods::full_column_name;

#[derive(Clone, Debug)]
pub struct NodeColumn {
    condition: JsonValue
}
impl NodeColumn {
    pub fn new(condition: JsonValue) -> Self {
        Self {
            condition
        }
    }
    fn get_full_column_names(&self, table_name: &str) -> Vec<String> {
        let mut vec = vec![];
        match self.get_condition() {
            JsonValue::Object(condition) => {
                for key in condition.keys() {
                    let full_column_name = full_column_name(key, table_name);
                    vec.push(full_column_name);
                }
            },
            JsonValue::Array(condition) => {
                for json_value in condition.iter() {
                    match json_value {
                        JsonValue::Object(json_value) => {
                            for key in json_value.keys() {
                                let full_column_name = full_column_name(key, table_name);
                                vec.push(full_column_name);
                            }
                        },
                        JsonValue::String(key) => {
                            let full_column_name = full_column_name(key, table_name);
                            vec.push(full_column_name);
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
impl NodeAble for NodeColumn {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_value(&self, table_name: &str) -> Vec<String> {
        self.get_full_column_names(table_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn get_full_column_names() {
        let node_column = NodeColumn::new(json!(["name", "age"]));
        assert_eq!(node_column.get_full_column_names("users"), vec!["`users`.`name`", "`users`.`age`"]);
    }
    #[test]
    fn to_value() {
        let node_column = NodeColumn::new(json!(["name", "age"]));
        assert_eq!(node_column.to_value("users"), vec!["`users`.`name`", "`users`.`age`"]);
    }
}