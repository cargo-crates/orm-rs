use serde_json::Value as JsonValue;
use crate::methods::full_column_name;
use crate::nodes::{NodesType, NodeWhere};
use crate::traits::ModelAble;

use std::vec::Vec;
// use std::collections::HashMap;

#[allow(dead_code)]
enum ClearableStatement {
    With, Select, Columns, HintComments, Where, Union, Join, Group, Order, Having, Limit, Offset, Counter, Counters,
}
#[allow(dead_code)]
enum LockModes {
    ForShare, ForUpdate
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct QueryBuilder {
    table_name: String,
    columns: Vec<String>,
    wheres: Vec<NodesType>,
    groups: Vec<NodesType>,
    havings: Vec<NodesType>,
}

impl QueryBuilder {
    pub fn new<T: ModelAble>() -> Self {
        let table_name = T::table_name();
        Self {
            table_name: table_name.clone(),
            columns: vec![format!("`{}`.*", table_name.clone())],
            wheres: vec![],
            groups: vec![],
            havings: vec![]
        }
    }
    pub fn except(&mut self, columns: JsonValue) -> &mut Self {
        if let JsonValue::Array(columns) = columns {
            let columns: Vec<_> = columns.into_iter().filter_map(|value| {
                if let JsonValue::String(value) = value {
                    Some(value)
                } else {
                    None
                }
            }).collect();
            for val in &columns {
                match val.as_ref() {
                    "where" => self.wheres = vec![],
                    _ => {}
                }
            }
        }
        self
    }
    pub fn r#where(&mut self, condition: JsonValue) -> &mut Self {
        self.wheres.push(NodesType::Where(NodeWhere::new(condition)));
        self
    }
    pub fn select(&mut self, columns: JsonValue) -> &mut Self {
        if let JsonValue::Array(columns) = columns {
            self.columns = columns.into_iter().filter_map(|value| {
                if let JsonValue::String(value) = value {
                    return Some(full_column_name(&value, &self.table_name));
                }
                None
            }).collect();
        }
        self
    }
    pub fn to_sql(&self) -> String {
        let mut sql = format!("SELECT {} FROM `{}`", self.columns.join(", "), self.table_name);
        if self.wheres.len() > 0 {
            let where_sql: Vec<String> = self.wheres.iter().filter_map(|val| {
                let NodesType::Where(node_where) = val;
                Some(node_where.to_sql(&self.table_name))
            }).map(|value| value.unwrap()).collect();
            sql = format!("{} WHERE {}", sql, where_sql.join(" AND "))
        }
        sql
    }
}