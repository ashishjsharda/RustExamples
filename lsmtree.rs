use sled::{Db, IVec};

fn main() -> sled::Result<()> {
    // Open a new or existing database located in the file system at `/tmp/my_db`.
    let db = Db::start_default("/tmp/my_db")?;

    // Insert a new key-value pair into the database.
    db.insert(b"my_key", b"my_value")?;

    // Retrieve the value associated with the key "my_key".
    let result = db.get(b"my_key")?;

    // Print the value if it exists.
    if let Some(value) = result {
        println!("value: {:?}", value);
    }

    // Remove the key-value pair from the database.
    db.remove(b"my_key")?;

    Ok(())
}
