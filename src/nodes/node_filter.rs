use serde_json::{Value as JsonValue};
use crate::nodes::NodeAble;
use crate::methods::full_column_name;

#[derive(Clone, Debug)]
pub struct NodeFilter {
    condition: JsonValue,
    is_not: bool,
    r#type: String
}
impl NodeFilter {
    fn new(condition: JsonValue, is_not: bool, r#type: &str) -> Self {
        Self {
            condition,
            is_not,
            r#type: r#type.to_string()
        }
    }
    pub fn new_where(condition: JsonValue) -> Self {
        Self::new(condition, false, "where")
    }
    pub fn new_where_not(condition: JsonValue) -> Self {
        Self::new(condition, true, "where")
    }
    pub fn new_having(condition: JsonValue) -> Self {
        Self::new(condition, false, "having")
    }
    pub fn new_having_not(condition: JsonValue) -> Self {
        Self::new(condition, true, "having")
    }
    pub fn get_type(&self) -> &str {
        &self.r#type
    }
}
impl NodeAble for NodeFilter {
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
    fn to_value() {
        let node_filter = NodeFilter::new_where(json!({
            "active": true,
            "age": 18,
            "gender": ["male", "female"],
            "profile": null
        }));
        assert_eq!(node_filter.to_value("users").join(" AND "), "`users`.`active` = 1 AND `users`.`age` = 18 AND `users`.`gender` IN ('male', 'female') AND `users`.`profile` IS NULL");

        let node_filter = NodeFilter::new_where_not(json!({
            "active": true,
            "age": 18,
            "gender": ["male", "female"],
            "profile": null
        }));
        assert_eq!(node_filter.to_value("users").join(" AND "), "`users`.`active` != 1 AND `users`.`age` != 18 AND `users`.`gender` NOT IN ('male', 'female') AND `users`.`profile` IS NOT NULL");
        // having
        let node_filter = NodeFilter::new_having(json!({
            "active": true,
            "age": 18,
            "gender": ["male", "female"],
            "profile": null
        }));
        assert_eq!(node_filter.to_value("users").join(" AND "), "`users`.`active` = 1 AND `users`.`age` = 18 AND `users`.`gender` IN ('male', 'female') AND `users`.`profile` IS NULL");

        let node_filter = NodeFilter::new_having_not(json!({
            "active": true,
            "age": 18,
            "gender": ["male", "female"],
            "profile": null
        }));
        assert_eq!(node_filter.to_value("users").join(" AND "), "`users`.`active` != 1 AND `users`.`age` != 18 AND `users`.`gender` NOT IN ('male', 'female') AND `users`.`profile` IS NOT NULL");
    }
}