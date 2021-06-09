use orm_rs::traits::ModelAble;
use serde_json::json;

fn main() {
    struct Sheep {}
    impl ModelAble for Sheep {
        fn new() -> Self {
            Self {}
        }
    }
    let mut query = Sheep::query();
    let query = query.r#where(json!({
        "name": "zhangsan",
        "age": 18,
        "gender": ["male", "female"],
        "active": true,
        "profile": null
    })).r#where(json!({"x": 1})).select(vec!["name", "age"]);
    println!("sql: {}", query.to_sql());
}

