use orm_rs::traits::ModelAble;
use serde_json::json;

struct User {}

impl ModelAble for User {
    fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod query {
    use super::*;
    #[test]
    fn test_where() {
        let mut query = User::query();
        let query = query.r#where(json!({
        "name": "zhangsan",
        "age": 18,
        "gender": ["male", "female"],
        "active": true,
        "profile": null
    })).r#where(json!({"x": 1}));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users` WHERE `users`.`active` = 1 AND `users`.`age` = 18 AND `users`.`gender` IN ('male', 'female') AND `users`.`name` = 'zhangsan' AND `users`.`profile` IS NULL AND `users`.`x` = 1");
    }

    #[test]
    fn test_except() {
        let mut query = User::query();
        let query = query.r#where(json!({
            "name": "zhangsan",
            "age": 18,
            "gender": ["male", "female"],
            "active": true,
            "profile": null
        })).r#where(json!({"x": 1}))
            .except(json!(["where"]))
            .r#where(json!({"y": 2}))
            .r#where_not(json!({
                "z2": "abc",
                "age": 18,
                "z1": [1, 2],
                "inactive": false,
                "address": null,

            }));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users` WHERE `users`.`y` = 2 AND `users`.`address` IS NOT NULL AND `users`.`age` != 18 AND `users`.`inactive` != 0 AND `users`.`z1` NOT IN (1, 2) AND `users`.`z2` != 'abc'");
    }

    #[test]
    fn test_select() {
        let mut query = User::query();
        let query = query.select(json!(["name", "age", "users.address"]));
        assert_eq!(query.to_sql(), "SELECT `users`.`name`, `users`.`age`, users.address FROM `users`");
    }
}