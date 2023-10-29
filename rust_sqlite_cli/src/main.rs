extern crate clap;
extern crate rusqlite;

use clap::{App, Arg, SubCommand};
use rusqlite::{params, Connection, Result};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Rust SQLite CLI")
        .version("0.1.0")
        .author("Your Name")
        .about("CLI for SQLite operations")
        .subcommand(
            SubCommand::with_name("create")
                .about("Create a new user")
                .arg(Arg::with_name("name").takes_value(true).required(true))
                .arg(Arg::with_name("age").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("read")
                .about("Read user data")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Update a user's age")
                .arg(Arg::with_name("name").takes_value(true).required(true))
                .arg(Arg::with_name("age").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete a user")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        )
        .get_matches();

    let conn = establish_connection()?;
    init_db(&conn)?;

    match matches.subcommand() {
        ("create", Some(sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            let age: i32 = sub_matches.value_of("age").unwrap().parse()?;
            create_user(&conn, name, age)?;
        }
        ("read", Some(sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            read_user(&conn, name)?;
        }
        ("update", Some(sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            let age: i32 = sub_matches.value_of("age").unwrap().parse()?;
            update_user(&conn, name, age)?;
        }
        ("delete", Some(sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            delete_user(&conn, name)?;
        }
        _ => println!("Invalid command. Use --help for usage information."),
    }

    Ok(())
}

fn establish_connection() -> Result<Connection, Box<dyn Error>> {
    Connection::open("rust_sqlite_cli.db").map_err(|err| Box::new(err) as Box<dyn Error>)
}

fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER)",
        params![],
    )?;
    Ok(())
}

fn create_user(conn: &Connection, name: &str, age: i32) -> Result<(), Box<dyn Error>> {
    let rows = conn.execute(
        "INSERT INTO user (name, age) VALUES (?, ?)",
        params![name, age],
    )?;
    if rows > 0 {
        println!("User {} created with age {}.", name, age);
    } else {
        println!("Failed to create user {}. User might already exist.", name);
    }
    Ok(())
}

fn read_user(conn: &Connection, name: &str) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT age FROM user WHERE name = ?")?;
    let user_age = stmt.query_row(params![name], |row| row.get::<_, i32>(0));

    match user_age {
        Ok(age) => {
            println!("User {} is {} years old.", name, age);
            Ok(())
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("User {} not found.", name);
            Ok(())
        }
        Err(e) => Err(Box::new(e) as Box<dyn Error>),
    }
}

fn update_user(conn: &Connection, name: &str, age: i32) -> Result<(), Box<dyn Error>> {
    let rows = conn.execute("UPDATE user SET age = ? WHERE name = ?", params![age, name])?;
    if rows > 0 {
        println!("User {}'s age updated to {}.", name, age);
    } else {
        println!("User {} not found. No update occurred.", name);
    }
    Ok(())
}

fn delete_user(conn: &Connection, name: &str) -> Result<(), Box<dyn Error>> {
    let rows = conn.execute("DELETE FROM user WHERE name = ?", params![name])?;
    if rows > 0 {
        println!("User {} deleted.", name);
    } else {
        println!("User {} not found. No deletion occurred.", name);
    }
    Ok(())
}
