use serde_json::{Value as JsonValue, Result};
use crate::methods::full_column_name;

#[derive(Debug)]
pub struct NodeWhere {
    condition: JsonValue
}
impl NodeWhere {
    pub fn new(condition: JsonValue) -> Self {
        Self {
            condition
        }
    }
    pub fn to_sql(&self, table_name: &str) -> Result<String> {
        let mut vec = vec![];
        if let JsonValue::Object(map_value) = &self.condition {
            for key in map_value.keys() {
                let column = full_column_name(key, table_name);
                if let Some(json_value) = map_value.get(key) {
                    match json_value {
                        JsonValue::Array(value) => {
                            let mut values = vec![];
                            for json_value in value.iter() {
                                match json_value {
                                    JsonValue::String(value) => {
                                        values.push(format!("'{}'", value));
                                    },
                                    JsonValue::Number(value) => {
                                        values.push(format!("{}", value));
                                    },
                                    _ => ()
                                }
                            }
                            if values.len() > 0 {
                                vec.push(format!("{} IN [{}]", column, values.join(",")));
                            }
                        },
                        JsonValue::String(value) => {
                            vec.push(format!("{} = '{}'", column, value));
                        },
                        JsonValue::Number(value) => {
                            vec.push(format!("{} = {}", column, value));
                        },
                        JsonValue::Bool(value) => {
                            let value = if *value {1} else {0};
                            vec.push(format!("{} = {}", column, value));
                        },
                        JsonValue::Null => {
                            vec.push(format!("{} IS NULL", column));
                        },
                        _ => ()
                    }
                }
            }
        }
        Ok(vec.join(" and "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn it_works() {
        let node_where = NodeWhere::new(json!({
            "active": true,
            "age": 18,
            "gender": ["male", "female"],
            "profile": null
        }));
        assert_eq!(node_where.to_sql("users").unwrap(), "`users`.`active` = 1 and `users`.`age` = 18 and `users`.`gender` IN ['male','female'] and `users`.`profile` IS NULL");
    }
}