mod node_column;
mod node_bool;
mod node_size;
mod node_filter;
mod node_filter_raw;
mod node_group;
mod node_order;
mod node_except;
mod node_op;
mod node_update;

pub use node_column::NodeColumn;
pub use node_bool::NodeBool;
pub use node_size::NodeSize;
pub use node_filter::NodeFilter;
pub use node_filter_raw::NodeFilterRaw;
pub use node_group::NodeGroup;
pub use node_order::NodeOrder;
pub use node_except::NodeExcept;
pub use node_op::NodeOp;
pub use node_update::NodeUpdate;
use serde_json::{Value as JsonValue};

pub trait NodeAble {
    fn get_condition(&self) -> &JsonValue;
    fn to_value(&self, table_name: &str) -> Vec<String>;
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
#[derive(Clone, Debug)]
pub enum NodesType {
    Bool(NodeBool),
    Size(NodeSize),
    Column(NodeColumn),
    Filter(NodeFilter),
    FilterRaw(NodeFilterRaw),
    Except(NodeExcept),
    Group(NodeGroup),
    Order(NodeOrder),
    Op(NodeOp),
    Update(NodeUpdate)
}

impl NodesType {}