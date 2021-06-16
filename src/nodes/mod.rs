mod node_where;

pub use node_where::NodeWhere;
use serde_json::{Value as JsonValue};
use crate::methods::full_column_name;

pub trait NodeAble {
    fn new(condition: JsonValue) -> Self;
    fn get_condition(&self) -> &JsonValue;
    // fn to_sql(&self, table_name: &str) -> Result<String> {
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
                        _ => ()
                    }
                }
            },
            _ => ()
        }
        vec
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
                                vec.push(format!("{} IN [{}]", full_column_name, values.join(",")));
                            }
                        },
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
                            vec.push(format!("{} IS NULL", full_column_name));
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

// #[derive(Debug)]
// pub struct NodeGroup {
// }
// impl NodeGroup {
//     pub fn new() -> Self {
//         Self {
//         }
//     }
// }
// #[derive(Debug)]
// pub struct NodeHaving {
// }
// impl NodeHaving {
//     pub fn new() -> Self {
//         Self {
//         }
//     }
// }
//
// #[allow(dead_code)]
// pub struct NodeCount {
//     distinct: bool
// }
// impl NodeCount {
//     pub fn new(distinct: bool) -> Self {
//         Self {
//             distinct
//         }
//     }
// }
#[derive(Debug)]
pub enum NodesType {
    Where(NodeWhere),
    // Group(NodeGroup),
    // Having(NodeHaving),
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn get_full_column_names() {
        let node_where = NodeWhere::new(json!({
            "active": true,
            "age": 18,
            "gender": ["male", "female"],
            "profile": null
        }));
        assert_eq!(node_where.get_full_column_names("users"), vec!["`users`.`active`", "`users`.`age`", "`users`.`gender`", "`users`.`profile`"]);
    }
    #[test]
    fn to_sql() {
        let node_where = NodeWhere::new(json!({
            "active": true,
            "age": 18,
            "gender": ["male", "female"],
            "profile": null
        }));
        assert_eq!(node_where.to_sql("users").join(" AND "), "`users`.`active` = 1 AND `users`.`age` = 18 AND `users`.`gender` IN ['male','female'] AND `users`.`profile` IS NULL");
    }
}