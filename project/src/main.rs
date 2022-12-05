/**
 * @file   main.rs
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Hope store - We sell hopes and dreams.
 *         Database interface front application.
 * @date   2022-11-23
 *
 * @copyright Copyright (c) 2022
 */
use postgres::{Client, NoTls};
use std::error::Error;
//use std::io;
//use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let host = dotenv::var("PG_HOST")?;
    let user = dotenv::var("PG_USER")?;
    let password = dotenv::var("PG_PASSWORD")?;
    let db       = dotenv::var("PG_DB")?;
    let schema   = dotenv::var("PG_SCHEMA")?;
    let mut client = Client::connect(&format!("host={host} user={user} password='{password}' dbname={db}"), NoTls)?;

    //let mut name = String::new();
    //let mut password = String::new();

    //println!("    Welcome to Hope store!");
    //println!("Here we sell hopes and dreams :)");
    //print!("name: ");
    //io::stdout().flush()?;
    //io::stdin().read_line(&mut name)?;
    //print!("password: ");
    //io::stdout().flush()?;
    //io::stdin().read_line(&mut password)?;

    //println!("Login: {name}");

    client.execute(&format!("SET SCHEMA '{schema}'"), &[])?;
    for row in client.query("SELECT id, firstname, lastname FROM student", &[])? {
        let id: i32 = row.get(0);
        let firstname: &str = row.get(1);
        let lastname: &str = row.get(2);
        println!("found: {} {} {}", id, firstname, lastname);
    }

    client.close()?;
    Ok(())
}
