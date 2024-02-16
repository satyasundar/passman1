use rusqlite::{params, Connection};
use rpassword::read_password_from_tty;
use std::error::Error;

const DATABASE_FILE: &str = "passwords.db";

fn main() -> Result<(), rusqlite::Error> {
    println!("Hello, passman1!");
    
    //create a new database connection
    // match open_database_connection() {
    //     Ok(conn) => {
    //         println!("Database connection established !!");
    //     }
    //     Err(err) => {
    //         eprintln!("Error: {} ", err);
    //     }
    // }
    let conn = Connection::open(DATABASE_FILE)?;

    //Initialize the databse schema
    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY,
            service TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )", [],
    )?;

    loop {
        println!("1. Store Password");
        println!("2. Retrieve Password");
        println!("3. Exit");

        let choice: u32  = read_password_from_tty(Some("Enter your choice: "))
            .expect("Failed to read input")
            .trim()
            .parse()
            .unwrap_or(0);
        
        match choice {
            1 => store_password(&conn)?,
            2 => retrieve_password(&conn)?,
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
    
    Ok(())
}

fn store_password(conn: &Connection) -> Result<(), rusqlite::Error> {
    println!("Enter Service name: ");
    let service = read_password_from_tty(Some("")).expect("Failed to read input");
    println!("Enter username: ");
    let username = read_password_from_tty(Some("")).expect("Failed to read input");
    println!("Enter password: ");
    let password = read_password_from_tty(Some("")).expect("Failed to read input");

    conn.execute(
        "INSERT INTO passwords (service, username, password) VALUES (?1, ?2, ?3)",
        params![service, username, password],
    )?;

    println!("Password stored successfully!");
    Ok(())
}

fn retrieve_password(conn: &Connection) -> Result<(), rusqlite::Error> {
    println!("Enter Service Name:");
    let service = read_password_from_tty(Some("")).expect("Failed to read input");
    let mut stmt = conn.prepare("SELECT username, password FROM passwords WHERE service = ?1")?;
    let mut rows = stmt.query(params![service])?;

    match rows.next()? {
        Some(row) => {
            let username: String = row.get(0)?;
            let password: String = row.get(1)?;
            println!("Username: {}", username);
            println!("Password: {}", password);
        }
        None => println!("Password not found!"),
    }
    
    Ok(())
}

fn open_database_connection() -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(DATABASE_FILE)?;
    Ok(conn)
}
