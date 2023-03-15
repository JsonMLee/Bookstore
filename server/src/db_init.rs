use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to the SQLite database file
    #[arg(short, long, default_value_t = {"database.db".to_string()})]
    database: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("[DEBUG] using {:?}", args);

    let manager = SqliteConnectionManager::file(args.database);
    let pool = r2d2::Pool::new(manager).unwrap();

    let conn = pool.get().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS BOOKS (
            ID INT PRIMARY KEY NOT NULL,
            TITLE CHAR(100) NOT NULL,
            TOPIC CHAR(50) NOT NULL,
            STOCK INT NOT NULL,
            PRICE FLOAT NOT NULL);
         )",
        [],
    )?;

    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO books (id, title, topic, stock, price) VALUES (?, ?, ?, ?, ?)",
    )?;

    stmt.execute(params![
        53477,
        "Achieve Less Bugs and More Hugs in CSCI 339",
        "distributed systems",
        20,
        29.99
    ])?;
    stmt.execute(params![
        53573,
        "Distributed Systems for Dummies",
        "distributed systems",
        20,
        24.99
    ])?;
    stmt.execute(params![
        12365,
        "Surviving College",
        "college life",
        20,
        19.99
    ])?;
    stmt.execute(params![
        12498,
        "Cooking for the Impatient Undergraduate",
        "college life",
        20,
        15.99
    ])?;

    println!("Database initialized successfully!");
    Ok(())
}
