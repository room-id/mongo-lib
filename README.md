# MongoLib

A simple Rust library to connect to a MongoDB database and perform basic operations with JSON objects using `serde_json::Value`.

## Features

* Connect to a MongoDB collection via connection URI, database, and collection name.
* Insert single or multiple JSON objects.
* Query documents and return results as JSON array.
* Update one or many documents with optional upsert.
* Delete one or many documents.

### Add to `Cargo.toml`

For implementing the library in projects:

```toml
[dependencies]
mongo_lib = { git = "ssh://git@github.com/room-id/mongo-lib.git", tag = "v0.1.1"}
```


For developing the library itself, create a new project and add the following to your Cargo.toml:
```toml
[dependencies]
mongo_lib = { path = "../mongo-lib" } # Or wherever the library is located
```

> The GitHub workflow automatically bumps the _PATCH_ version number: 0.2.0 => 0.2.1 when a PR is merged.
> Make sure to _pull_ when the action is completed 
> If a _MAJOR_ or _MINOR_ version bump is needed, update `Cargo.toml` manually. The action should be okay with it.
> 
> 

## Call lib

```rust
use mongo_lib::MongoLib;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize library
    let lib = MongoLib::new(("mongodb://localhost:27017", "location", "locations")).await?;

    // Insert single document
    lib.insert_json(json!({ "name": "Test", "value": 123 })).await?;

    // Insert multiple documents
    lib.insert_json(json!([
        { "name": "Doc1", "value": 1 },
        { "name": "Doc2", "value": 2 }
    ])).await?;

    // Query documents
    let results = lib.query_json(json!({ "name": "Test" })).await?;
    println!("Results: {}", results);

    // Update one document with upsert
    let updated = lib.update_one_json(json!({ "name": "Test" }), json!({ "value": 456 }), true).await?;
    println!("Updated: {}", updated);

    // Delete one document
    let deleted = lib.delete_one_json(json!({ "name": "Test" })).await?;
    println!("Deleted: {}", deleted);

    Ok(())
}
```
