### An ORM For Help Create Sql

### Usage
```rust
use orm_rs::traits::ModelAble;
use serde_json::json;

#[derive(Clone, Debug)]
struct User {}
impl ModelAble for User {
    fn new() -> Self {
        Self {}
    }
}
 // where
let query = User::query();
let sql = query.r#where(json!({
            "name": "zhangsan",
            "age": 18,
            "gender": ["male", "female"],
            "active": true,
            "profile": null
        })).to_sql();
println("sql: {}", sql);
```

Finished

* select
* distinct
* where
* where_not
* where_raw
* group
* having
* having_not
* having_raw
* order
* limit
* offset
* paginate
* count
* sum
* avg
* min
* max
* except
