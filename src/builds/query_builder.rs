use serde_json::{Value as JsonValue, json};
// use crate::methods::full_column_name;
use crate::nodes::{NodeAble, NodesType, NodeColumn, NodeWhere, NodeExcept, NodeWhereRaw};
use crate::traits::ModelAble;
use std::marker::PhantomData;

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
pub struct QueryBuilder<T: ModelAble> {
    nodes: Vec<NodesType>,
    _marker: PhantomData<T>
}

impl<T: ModelAble> QueryBuilder<T> {
    // pub fn new<T: ModelAble>() -> Self {
    pub fn new() -> QueryBuilder<T> {
        Self {
            // columns: vec![format!("`{}`.*", T::table_name())],
            nodes: vec![NodesType::Column(NodeColumn::new(json!(["*"])))],
            _marker: PhantomData
        }
    }
    pub fn except(&mut self, condition: JsonValue) -> &mut Self {
        if let JsonValue::Array(_) = &condition {
            self.nodes.push(NodesType::Except(NodeExcept::new(condition)));
        } else {
            panic!("Error: except only support json array, got: {:?}", condition);
        }
        self
    }
    pub fn r#where(&mut self, condition: JsonValue) -> &mut Self {
        self.nodes.push(NodesType::Where(NodeWhere::new(condition, false)));
        self
    }
    pub fn r#where_not(&mut self, condition: JsonValue) -> &mut Self {
        self.nodes.push(NodesType::Where(NodeWhere::new(condition, true)));
        self
    }
    pub fn r#where_raw(&mut self, raw_sql: &str, placeholder_values: JsonValue) -> &mut Self {
        let raw_sql_should_value_len = raw_sql.chars().filter(|char| char == &'?').count();
        if let JsonValue::Array(values) = &placeholder_values {
            if values.len() == raw_sql_should_value_len {
                self.nodes.push(NodesType::WhereRaw(NodeWhereRaw::new(raw_sql, placeholder_values)));
            } else {
                panic!("Error: where_raw param placeholder_values len incorrect, need len: {}, got len {}, got: {}", raw_sql_should_value_len, values.len(), placeholder_values);
            }
        } else {
            panic!("Error: where_raw param placeholder_values only support json array, got: {:?}", placeholder_values);
        }
        self
    }
    pub fn select(&mut self, condition: JsonValue) -> &mut Self {
        if let JsonValue::Array(_) = &condition {
            self.nodes.push(NodesType::Column(NodeColumn::new(condition)));
        } else {
            panic!("Error: select only support json array, got: {:?}", condition);
        }
        self
    }
    pub fn to_sql(&self) -> String {
        let mut wheres_sql: Vec<String> = vec![];
        for node in &self.nodes {
            match node {
                NodesType::Where(node_where) => {
                    wheres_sql = [&wheres_sql[..], &node_where.to_sql(&T::table_name())].concat()
                },
                NodesType::WhereRaw(node_where_raw) => {
                    wheres_sql = [&wheres_sql[..], &node_where_raw.to_sql(&T::table_name())].concat()
                },
                NodesType::Except(node_except) => {
                    match node_except.get_condition() {
                        JsonValue::Array(json_array) => {
                            let columns: Vec<_> = json_array.into_iter().filter_map(|value| {
                                if let JsonValue::String(value) = value { Some(value) } else { None }
                            }).collect();
                            for val in columns {
                                match val.to_lowercase().as_ref() {
                                    "where" => { wheres_sql = vec![] },
                                    _ => {}
                                }
                            }
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        }
        // column_nodes
        let mut columns_sql: Vec<String> = vec!["*".to_string()];
        {
            let column_nodes = self.get_column_nodes();
            if let NodesType::Column(node_column) = column_nodes.last().unwrap() {
                columns_sql = node_column.to_sql(&T::table_name());
            }
        }
        // create sql
        let mut sql = format!("SELECT {} FROM `{}`", columns_sql.join(", "), T::table_name());
        if wheres_sql.len() > 0 {
            sql = format!("{} WHERE {}", sql, wheres_sql.join(" AND "));
        }
        sql
    }
    fn get_column_nodes(&self) -> Vec<&NodesType> {
        self.nodes.iter().filter(|&node| match node { NodesType::Column(_) => true, _ => false }).collect()
    }
    // fn get_where_nodes(&self) -> Vec<&NodesType> {
    //     self.nodes.iter().filter(|&node| match node { NodesType::Where(_) => true, _ => false }).collect()
    // }
    // fn get_except_nodes(&self) -> Vec<&NodesType> {
    //     self.nodes.iter().filter(|&node| match node { NodesType::Except(_) => true, _ => false }).collect()
    // }
}