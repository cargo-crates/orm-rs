mod node_where;

pub use node_where::NodeWhere;


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