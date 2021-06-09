use std::any::type_name;
use inflector::{string::{demodulize, pluralize}, cases::snakecase};

pub fn table_name<T>() -> String where T: ?Sized {
    // eg: arel::UserTable
    let full_namespace = type_name::<T>();
    // eg: UserTable
    let struct_name = demodulize::demodulize(&full_namespace);
    // eg: user_table
    let snake_struct_name = snakecase::to_snake_case(&struct_name);
    // eg: user_tables
    pluralize::to_plural(&snake_struct_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // User
        struct User {}
        assert_eq!(table_name::<User>(), "users");
        // UserRole
        struct UserRole {}
        assert_eq!(table_name::<UserRole>(), "user_roles");
    }
}