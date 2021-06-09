use crate::builds::query_builder::QueryBuilder;
use crate::methods::{table_name};

pub trait ModelAble: Sized {
    fn new() -> Self;
    fn table_name() -> String {
        table_name::<Self>()
    }
    fn query() -> QueryBuilder {
        QueryBuilder::new::<Self>()
    }
}
// pub trait TableExt {
// }
// impl<T> TableExt for T where T: Table {
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//     }
// }
