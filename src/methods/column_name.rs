use regex::Regex;

pub fn full_column_name(column_name: &str, table_name: &str) -> String {
    if column_name == "*" {
        format!("`{}`.{}", table_name, column_name)
    } else if Regex::new(r"\.").unwrap().is_match(column_name) {
        format!("{}", column_name)
    } else {
        format!("`{}`.`{}`", table_name, column_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_full_column_name() {
        assert_eq!(full_column_name("*", "users"), "`users`.*");
        assert_eq!(full_column_name("name", "users"), "`users`.`name`");
        assert_eq!(full_column_name("users.name", "users"), "users.name");
        assert_eq!(full_column_name("users.name", "orders"), "users.name");
    }
}