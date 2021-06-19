use serde_json::{Value as JsonValue, json};
// use crate::methods::full_column_name;
use crate::nodes::{NodeAble, NodesType, NodeBool, NodeSize, NodeColumn, NodeFilter, NodeExcept, NodeFilterRaw, NodeGroup, NodeOrder, NodeOp};
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
#[derive(Clone, Debug)]
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
    pub fn r#where(&mut self, condition: JsonValue) -> &mut Self {
        self.nodes.push(NodesType::Filter(NodeFilter::new_where(condition)));
        self
    }
    pub fn where_not(&mut self, condition: JsonValue) -> &mut Self {
        self.nodes.push(NodesType::Filter(NodeFilter::new_where_not(condition)));
        self
    }
    pub fn having(&mut self, condition: JsonValue) -> &mut Self {
        self.nodes.push(NodesType::Filter(NodeFilter::new_having(condition)));
        self
    }
    pub fn having_not(&mut self, condition: JsonValue) -> &mut Self {
        self.nodes.push(NodesType::Filter(NodeFilter::new_having_not(condition)));
        self
    }
    pub fn where_raw(&mut self, raw_sql: &str, placeholder_values: JsonValue) -> &mut Self {
        let raw_sql_should_value_len = raw_sql.chars().filter(|char| char == &'?').count();
        if let JsonValue::Array(values) = &placeholder_values {
            if values.len() == raw_sql_should_value_len {
                self.nodes.push(NodesType::FilterRaw(NodeFilterRaw::new_where(raw_sql, placeholder_values)));
            } else {
                panic!("Error: where_raw param placeholder_values len incorrect, need len: {}, got len {}, got: {}", raw_sql_should_value_len, values.len(), placeholder_values);
            }
        } else {
            panic!("Error: where_raw param placeholder_values only support json array, got: {:?}", placeholder_values);
        }
        self
    }
    pub fn having_raw(&mut self, raw_sql: &str, placeholder_values: JsonValue) -> &mut Self {
        let raw_sql_should_value_len = raw_sql.chars().filter(|char| char == &'?').count();
        if let JsonValue::Array(values) = &placeholder_values {
            if values.len() == raw_sql_should_value_len {
                self.nodes.push(NodesType::FilterRaw(NodeFilterRaw::new_having(raw_sql, placeholder_values)));
            } else {
                panic!("Error: having_raw param placeholder_values len incorrect, need len: {}, got len {}, got: {}", raw_sql_should_value_len, values.len(), placeholder_values);
            }
        } else {
            panic!("Error: having_raw param placeholder_values only support json array, got: {:?}", placeholder_values);
        }
        self
    }
    pub fn group(&mut self, condition: JsonValue) -> &mut Self {
        if let JsonValue::Array(_) = &condition {
            self.nodes.push(NodesType::Group(NodeGroup::new(condition)));
        } else {
            panic!("Error: group only support json array, got: {:?}", condition);
        }
        self
    }
    pub fn order(&mut self, condition: JsonValue) -> &mut Self {
        match &condition {
            JsonValue::Array(_) => self.nodes.push(NodesType::Order(NodeOrder::new(condition))),
            JsonValue::Object(_) => self.nodes.push(NodesType::Order(NodeOrder::new(condition))),
            _ => panic!("Error: order only support json array, got: {:?}", condition)
        }
        self
    }
    pub fn except(&mut self, condition: JsonValue) -> &mut Self {
        if let JsonValue::Array(_) = &condition {
            self.nodes.push(NodesType::Except(NodeExcept::new(condition)));
        } else {
            panic!("Error: except only support json array, got: {:?}", condition);
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
    pub fn distinct(&mut self) -> &mut Self {
        self.nodes.push(NodesType::Bool(NodeBool::new_distinct()));
        self
    }
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.nodes.push(NodesType::Size(NodeSize::new_limit(limit)));
        self
    }
    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.nodes.push(NodesType::Size(NodeSize::new_offset(offset)));
        self
    }
    pub fn paginate(&mut self, page: usize, per_page: usize) -> &mut Self {
        let offset = (page - 1) * per_page;
        self.limit(per_page).offset(offset);
        self
    }
    pub fn count(&mut self) -> &mut Self {
        self.nodes.push(NodesType::Op(NodeOp::new_count()));
        self
    }
    pub fn sum(&mut self, field: &str) -> &mut Self {
        self.nodes.push(NodesType::Op(NodeOp::new_sum(field)));
        self
    }
    pub fn avg(&mut self, field: &str) -> &mut Self {
        self.nodes.push(NodesType::Op(NodeOp::new_avg(field)));
        self
    }
    pub fn min(&mut self, field: &str) -> &mut Self {
        self.nodes.push(NodesType::Op(NodeOp::new_min(field)));
        self
    }
    pub fn max(&mut self, field: &str) -> &mut Self {
        self.nodes.push(NodesType::Op(NodeOp::new_max(field)));
        self
    }

    pub fn to_sql(&self) -> String {
        let mut wheres_sql: Vec<String> = vec![];
        let mut havings_sql: Vec<String> = vec![];
        let mut groups_sql: Vec<String> = vec![];
        let mut orders_sql: Vec<String> = vec![];
        let mut limits_sql: Vec<String> = vec![];
        let mut offsets_sql: Vec<String> = vec![];
        let mut is_distinct = false;
        for node in &self.nodes {
            match node {
                NodesType::Filter(node_filter) => {
                    match node_filter.get_type() {
                        "where" => wheres_sql = [&wheres_sql[..], &node_filter.to_value(&T::table_name())].concat(),
                        "having" => havings_sql = [&havings_sql[..], &node_filter.to_value(&T::table_name())].concat(),
                        _ => ()
                    }
                },
                NodesType::FilterRaw(node_filter_raw) => {
                    match node_filter_raw.get_type() {
                        "where" => wheres_sql = [&wheres_sql[..], &node_filter_raw.to_value(&T::table_name())].concat(),
                        "having" => havings_sql = [&havings_sql[..], &node_filter_raw.to_value(&T::table_name())].concat(),
                        _ => ()
                    }
                },
                NodesType::Group(node_group) => {
                  groups_sql = [&groups_sql[..], &node_group.to_value(&T::table_name())].concat()
                },
                NodesType::Order(node_order) => {
                    orders_sql = [&orders_sql[..], &node_order.to_value(&T::table_name())].concat()
                },
                NodesType::Except(node_except) => {
                    let columns = node_except.to_value(&T::table_name());
                    for val in &columns {
                        match val.to_lowercase().as_ref() {
                            "where" => { wheres_sql = vec![] },
                            "group" => { groups_sql = vec![] },
                            "having" => { havings_sql = vec![] },
                            "order" => { orders_sql = vec![] },
                            "limit" => { limits_sql = vec![] },
                            "offset" => { offsets_sql = vec![] },
                            "distinct" => { is_distinct = false },
                            _ => {}
                        }
                    }
                },
                NodesType::Size(node_size) => {
                    match node_size.get_type() {
                        "limit" => limits_sql = [&limits_sql[..], &node_size.to_value(&T::table_name())].concat(),
                        "offset" => offsets_sql = [&offsets_sql[..], &node_size.to_value(&T::table_name())].concat(),
                        _ => ()
                    }
                },
                NodesType::Bool(node_bool) => {
                  match node_bool.get_type() {
                      "distinct" => is_distinct = true,
                      _ => ()
                  }
                },
                _ => ()
            }
        }
        // column_nodes
        let mut columns_sql: Vec<String> = vec!["*".to_string()];
        {
            let column_nodes = self.get_column_nodes();
            if let NodesType::Column(node_column) = column_nodes.last().unwrap() {
                columns_sql = node_column.to_value(&T::table_name());
            }
        }
        // op_nodes
        let op_nodes = self.get_op_nodes();
        // create sql
        let mut sql = "SELECT".to_string();
        let mut distinct_sql = "";
        if is_distinct {
            distinct_sql = "DISTINCT ";
        }
        match op_nodes.last() {
            Some(node_type) => {
                if let NodesType::Op(node_op) = node_type {
                    match node_op.get_type() {
                        "count" => sql = format!("{} COUNT({}{})", sql, distinct_sql, columns_sql.join(", ")),
                        "sum" => sql = format!("{} SUM({}{})", sql, distinct_sql, node_op.to_value(&T::table_name()).join(", ")),
                        "avg" => sql = format!("{} AVG({}{})", sql, distinct_sql, node_op.to_value(&T::table_name()).join(", ")),
                        "min" => sql = format!("{} MIN({}{})", sql, distinct_sql, node_op.to_value(&T::table_name()).join(", ")),
                        "max" => sql = format!("{} MAX({}{})", sql, distinct_sql, node_op.to_value(&T::table_name()).join(", ")),
                        _ => ()
                    }
                }
            },
            None => {
                sql = format!("{} {}{}", sql, distinct_sql, columns_sql.join(", "))
            }
        }
        sql = format!("{} FROM `{}`", sql, T::table_name());
        if wheres_sql.len() > 0 {
            sql = format!("{} WHERE {}", sql, wheres_sql.join(" AND "));
        }
        if groups_sql.len() > 0 {
            sql = format!("{} GROUP BY {}", sql, groups_sql.join(", "));
            if havings_sql.len() > 0 {
                sql = format!("{} HAVING {}", sql, havings_sql.join(" AND "));
            }
        }
        if orders_sql.len() > 0 {
            sql = format!("{} ORDER BY {}", sql, orders_sql.join(", "));
        }
        if limits_sql.len() > 0 {
            if let Some(limit) = limits_sql.last() {
                sql = format!("{} {}", sql, limit)
            }
        }
        if offsets_sql.len() > 0 {
            if let Some(offset) = offsets_sql.last() {
                sql = format!("{} {}", sql, offset)
            }
        }
        sql
    }
    fn get_column_nodes(&self) -> Vec<&NodesType> {
        self.nodes.iter().filter(|&node| match node { NodesType::Column(_) => true, _ => false }).collect()
    }
    fn get_op_nodes(&self) -> Vec<&NodesType> {
        self.nodes.iter().filter(|&node| match node { NodesType::Op(_) => true, _ => false }).collect()
    }
    // fn get_where_filters(&self) -> Vec<&NodesType> {
    //     self.nodes.iter().filter(|&node| match node { NodesType::Filter(_) => true, _ => false }).collect()
    // }
    // fn get_except_nodes(&self) -> Vec<&NodesType> {
    //     self.nodes.iter().filter(|&node| match node { NodesType::Except(_) => true, _ => false }).collect()
    // }
}