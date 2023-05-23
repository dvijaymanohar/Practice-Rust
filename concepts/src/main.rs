use rusqlite::{Connection, Result};

// Struct representing a person entity
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<()> {
    // Open a connection to the SQLite database
    let conn = Connection::open_in_memory()?;

    // Create the "person" table
    conn.execute(
        "CREATE TABLE person (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL,
                  age INTEGER NOT NULL
                  )",
        [],
    )?;

    // Insert a person into the database
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        &["Alice", "30"],
    )?;

    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        &["VMD", "39"],
    )?;

    // Query all persons from the database
    let mut stmt = conn.prepare("SELECT id, name, age FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    // Print the retrieved persons
    for person_result in person_iter {
        let person = person_result?;
        println!("{:?}", person);
    }

    Ok(())
}
