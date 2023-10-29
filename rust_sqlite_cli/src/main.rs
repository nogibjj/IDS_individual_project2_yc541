extern crate rusqlite;

use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // create/open SQLite database
    let conn = Connection::open("rust_sqlite_cli.db")?;

    // create a new table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             age INTEGER
         )",
        params![],
    )?;

    // Create
    conn.execute("INSERT INTO user (name, age) VALUES (?, ?)", params!["Alice", 30])?;

    // Read
    let mut stmt = conn.prepare("SELECT id, name, age FROM user")?;
    let user_iter = stmt.query_map(params![], |row| {
        Ok((
            row.get(0)?, // id
            row.get(1)?, // name
            row.get(2)?, // age
        ))
    })?;

    for user in user_iter {
        println!("{:?}", user.unwrap());
    }

    // Update
    conn.execute("UPDATE user SET age = ? WHERE name = ?", params![31, "Alice"])?;

    // Delete
    conn.execute("DELETE FROM user WHERE name = ?", params!["Alice"])?;

    Ok(())
}
