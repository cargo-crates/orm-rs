mod node_column;
mod node_filter;
mod node_filter_raw;
mod node_group;
mod node_except;

pub use node_column::NodeColumn;
pub use node_filter::NodeFilter;
pub use node_filter_raw::NodeFilterRaw;
pub use node_group::NodeGroup;
pub use node_except::NodeExcept;
use serde_json::{Value as JsonValue};

pub trait NodeAble {
    fn get_condition(&self) -> &JsonValue;
    fn to_sql(&self, table_name: &str) -> Vec<String>;
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
    Column(NodeColumn),
    Filter(NodeFilter),
    FilterRaw(NodeFilterRaw),
    Except(NodeExcept),
    Group(NodeGroup)
    // Having(NodeHaving),
}