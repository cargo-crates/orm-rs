use orm_rs::traits::ModelAble;
use serde_json::json;

#[derive(Clone, Debug)]
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
        let query_clone = query.clone();
        let query = query.r#where(json!({
            "name": "zhangsan",
            "age": 18,
            "gender": ["male", "female"],
            "active": true,
            "profile": null
        })).r#where(json!({"x": 1}));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users` WHERE `users`.`active` = 1 AND `users`.`age` = 18 AND `users`.`gender` IN ('male', 'female') AND `users`.`name` = 'zhangsan' AND `users`.`profile` IS NULL AND `users`.`x` = 1");
        assert_eq!(query_clone.to_sql(), "SELECT `users`.* FROM `users`");
    }

    #[test]
    fn test_where_raw() {
        let mut query = User::query();
        let query = query.where_raw("name LIKE ? AND gender in ?", json!(["%王%", ["male", "female"]]));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users` WHERE name LIKE '%王%' AND gender in ('male', 'female')");
    }

    #[test]
    fn test_group() {
        let mut query = User::query();
        let query = query.group(json!(["age", "gender"]));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users` GROUP BY `users`.`age`, `users`.`gender`");
    }

    #[test]
    fn test_having() {
        let mut query = User::query();
        let query = query
            .r#where(json!({"active": true}))
            .group(json!(["age", "gender"]))
            .having(json!({"gender": ["male", "female"]}))
            .having_not(json!({"age": 0}))
            .having_raw("count(*) > ?", json!([3]));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users` WHERE `users`.`active` = 1 GROUP BY `users`.`age`, `users`.`gender` HAVING `users`.`gender` IN ('male', 'female') AND `users`.`age` != 0 AND count(*) > 3");
    }

    #[test]
    fn test_order() {
        let mut query = User::query();
        let query = query.order(json!({
            "name": "desc"
        }));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users` ORDER BY `users`.`name` DESC");
        query.order(json!(["gender", "age DESC"]));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users` ORDER BY `users`.`name` DESC, gender ASC, age DESC");
        query.except(json!(["order"]));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users`");
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
            .group(json!(["gender"]))
            .except(json!(["where", "group"]))
            .r#where(json!({"y": 2}))
            .where_not(json!({
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

    #[test]
    fn test_distinct() {
        let mut query = User::query();
        let query = query.distinct();
        assert_eq!(query.to_sql(), "SELECT DISTINCT `users`.* FROM `users`");
    }

    #[test]
    fn test_paginate() {
        let mut query = User::query();
        let query = query.paginate(1, 10);
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users` LIMIT 10 OFFSET 0");
        query.except(json!(["offset"]));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users` LIMIT 10");
        query.except(json!(["limit"]));
        assert_eq!(query.to_sql(), "SELECT `users`.* FROM `users`");
    }

    #[test]
    fn test_count() {
        let mut query = User::query();
        let query = query.count();
        assert_eq!(query.to_sql(), "SELECT COUNT(`users`.*) FROM `users`");

        let mut query = User::query();
        let query = query.select(json!(["name", "id"])).distinct().count();
        assert_eq!(query.to_sql(), "SELECT COUNT(DISTINCT `users`.`name`, `users`.`id`) FROM `users`");
    }

    #[test]
    fn test_sum_avg_min_max() {
        let mut query = User::query();
        let query = query.sum("id");
        assert_eq!(query.to_sql(), "SELECT SUM(`users`.`id`) FROM `users`");

        let mut query = User::query();
        let query = query.select(json!(["id", "name"])).distinct().sum("id");
        assert_eq!(query.to_sql(), "SELECT SUM(DISTINCT `users`.`id`) FROM `users`");

        let mut query = User::query();
        let query = query.avg("id");
        assert_eq!(query.to_sql(), "SELECT AVG(`users`.`id`) FROM `users`");

        let mut query = User::query();
        let query = query.min("id");
        assert_eq!(query.to_sql(), "SELECT MIN(`users`.`id`) FROM `users`");

        let mut query = User::query();
        let query = query.max("id");
        assert_eq!(query.to_sql(), "SELECT MAX(`users`.`id`) FROM `users`");
    }

    #[test]
    fn test_update_all() {
        let mut query = User::query();
        query.r#where(json!({
            "name": "lisi"
        })).update_all(json!({ "gender": "male" }));
        assert_eq!(query.to_sql(), "UPDATE `users` SET `users`.`gender` = 'male' WHERE `users`.`name` = 'lisi'");
    }
}