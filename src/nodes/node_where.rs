use serde_json::{Value as JsonValue};
use crate::nodes::NodeAble;
use crate::methods::full_column_name;

#[derive(Debug)]
pub struct NodeWhere {
    condition: JsonValue,
    is_not: bool
}
impl NodeWhere {
    pub fn new(condition: JsonValue, is_not: bool) -> Self {
        Self {
            condition,
            is_not
        }
    }
}
impl NodeAble for NodeWhere {
    fn get_condition(&self) -> &JsonValue {
        &self.condition
    }
    fn to_sql(&self, table_name: &str) -> Vec<String> {
        let mut vec = vec![];
        if let JsonValue::Object(map_value) = self.get_condition() {
            for key in map_value.keys() {
                let full_column_name = full_column_name(key, table_name);
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
                                if self.is_not {
                                    vec.push(format!("{} NOT IN ({})", full_column_name, values.join(", ")));
                                } else {
                                    vec.push(format!("{} IN ({})", full_column_name, values.join(", ")));
                                }
                            }
                        },
                        JsonValue::String(value) => {
                            if self.is_not {
                                vec.push(format!("{} != '{}'", full_column_name, value));
                            } else {
                                vec.push(format!("{} = '{}'", full_column_name, value));
                            }
                        },
                        JsonValue::Number(value) => {
                            if self.is_not {
                                vec.push(format!("{} != {}", full_column_name, value));
                            } else {
                                vec.push(format!("{} = {}", full_column_name, value));
                            }
                        },
                        JsonValue::Bool(value) => {
                            let value = if *value {1} else {0};
                            if self.is_not {
                                vec.push(format!("{} != {}", full_column_name, value));
                            } else {
                                vec.push(format!("{} = {}", full_column_name, value));
                            }
                        },
                        JsonValue::Null => {
                            if self.is_not {
                                vec.push(format!("{} IS NOT NULL", full_column_name));
                            } else {
                                vec.push(format!("{} IS NULL", full_column_name));
                            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn to_sql() {
        let node_where = NodeWhere::new(json!({
            "active": true,
            "age": 18,
            "gender": ["male", "female"],
            "profile": null
        }), false);
        assert_eq!(node_where.to_sql("users").join(" AND "), "`users`.`active` = 1 AND `users`.`age` = 18 AND `users`.`gender` IN ('male', 'female') AND `users`.`profile` IS NULL");

        let node_where = NodeWhere::new(json!({
            "active": true,
            "age": 18,
            "gender": ["male", "female"],
            "profile": null
        }), true);
        assert_eq!(node_where.to_sql("users").join(" AND "), "`users`.`active` != 1 AND `users`.`age` != 18 AND `users`.`gender` NOT IN ('male', 'female') AND `users`.`profile` IS NOT NULL");
    }
}