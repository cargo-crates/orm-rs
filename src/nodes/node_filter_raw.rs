use serde_json::{Value as JsonValue};
use crate::nodes::NodeAble;
#[derive(Clone, Debug)]
pub struct NodeFilterRaw {
    raw_sql: String,
    // placeholder values
    condition: JsonValue,
    r#type: String
}
impl NodeFilterRaw {
    fn new(raw_sql: &str, condition: JsonValue, r#type: &str) -> Self {
        Self {
            raw_sql: raw_sql.to_string(),
            condition,
            r#type: r#type.to_string()
        }
    }
    pub fn new_where(raw_sql: &str, condition: JsonValue) -> Self {
        Self::new(raw_sql, condition, "where")
    }
    pub fn new_having(raw_sql: &str, condition: JsonValue) -> Self {
        Self::new(raw_sql, condition, "having")
    }
    pub fn get_type(&self) -> &str {
        &self.r#type
    }
}
impl NodeAble for NodeFilterRaw {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_value(&self, _table_name: &str) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn to_value() {
        // raw
        let node_filter_raw = NodeFilterRaw::new_where("name LIKE 'N%'", json!([]));
        assert_eq!(node_filter_raw.to_value("users"), vec!["name LIKE 'N%'"]);
        // str
        let node_filter_raw = NodeFilterRaw::new_where("name LIKE ?", json!(["N%"]));
        assert_eq!(node_filter_raw.to_value("users"), vec!["name LIKE 'N%'"]);
        // num
        let node_filter_raw = NodeFilterRaw::new_where("age = ?", json!([18]));
        assert_eq!(node_filter_raw.to_value("users"), vec!["age = 18"]);
        // bool
        let node_filter_raw = NodeFilterRaw::new_where("active = ?", json!([false]));
        assert_eq!(node_filter_raw.to_value("users"), vec!["active = 0"]);
        // array num
        let node_filter_raw = NodeFilterRaw::new_where("gender IN ?", json!([[1, 2]]));
        assert_eq!(node_filter_raw.to_value("users"), vec!["gender IN (1, 2)"]);
        // array string
        let node_filter_raw = NodeFilterRaw::new_where("gender IN ?", json!([["male", "female"]]));
        assert_eq!(node_filter_raw.to_value("users"), vec!["gender IN ('male', 'female')"]);
        // having
        let node_filter_raw = NodeFilterRaw::new_having("count(*) > ?", json!([3]));
        assert_eq!(node_filter_raw.to_value("users"), vec!["count(*) > 3"]);
    }
}