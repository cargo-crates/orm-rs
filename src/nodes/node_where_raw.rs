use serde_json::{Value as JsonValue};
use crate::nodes::NodeAble;
#[derive(Debug)]
pub struct NodeWhereRaw {
    raw_sql: String,
    // placeholder values
    condition: JsonValue
}
impl NodeAble for NodeWhereRaw {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_sql(&self, _table_name: &str) -> Vec<String> {
        let mut values: Vec<JsonValue> = vec![];
        match self.get_condition() {
            JsonValue::Array(json_array) => {
                values = json_array.to_vec();
                // for json_item in json_array {
                //     match json_item {
                //         JsonValue::String(value) => {
                //             values.push(value)
                //         }
                //         _ => ()
                //     }
                // }
            },
            _ => {}
        }
        let sql = self.raw_sql.chars().map(|char|
            match char {
                '?' => {
                    match values.remove(0).into() {
                        JsonValue::String(value) => format!("'{}'", value),
                        JsonValue::Number(value) => format!("{}", value),
                        JsonValue::Bool(value) => {
                            let value = if value {1} else {0};
                            format!("{}", value)
                        },
                        JsonValue::Array(values) => {
                            let values:Vec<String> = values.iter().map(|v| {
                                match v {
                                    JsonValue::String(value) => format!("'{}'", value),
                                    JsonValue::Number(value) => format!("{}", value),
                                    JsonValue::Bool(value) => {
                                        let value = if *value {1} else {0};
                                        format!("{}", value)
                                    },
                                    _ => panic!("Error: 类型不支持")
                                }
                            }).collect();
                            format!("({})", values.join(", "))
                        },
                        _ => panic!("Error: 类型不支持")
                    }
                },
                _ => char.to_string()
            }).collect();
        vec![sql]
    }
}
impl NodeWhereRaw {
    pub fn new(raw_sql: &str, condition: JsonValue) -> Self {
        Self {
            raw_sql: raw_sql.to_string(),
            condition
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn to_sql() {
        // raw
        let node_where_raw = NodeWhereRaw::new("name LIKE 'N%'", json!([]));
        assert_eq!(node_where_raw.to_sql("users"), vec!["name LIKE 'N%'"]);
        // str
        let node_where_raw = NodeWhereRaw::new("name LIKE ?", json!(["N%"]));
        assert_eq!(node_where_raw.to_sql("users"), vec!["name LIKE 'N%'"]);
        // num
        let node_where_raw = NodeWhereRaw::new("age = ?", json!([18]));
        assert_eq!(node_where_raw.to_sql("users"), vec!["age = 18"]);
        // bool
        let node_where_raw = NodeWhereRaw::new("active = ?", json!([false]));
        assert_eq!(node_where_raw.to_sql("users"), vec!["active = 0"]);
        // array num
        let node_where_raw = NodeWhereRaw::new("gender IN ?", json!([[1, 2]]));
        assert_eq!(node_where_raw.to_sql("users"), vec!["gender IN (1, 2)"]);
        // array string
        let node_where_raw = NodeWhereRaw::new("gender IN ?", json!([["male", "female"]]));
        assert_eq!(node_where_raw.to_sql("users"), vec!["gender IN ('male', 'female')"]);
    }
}