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
use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
    const HOST: &'static str = "localhost";
    const USER: &'static str = "miku";
    const PASSWORD: &'static str = "hatsune";
    const DATABASE: &'static str = "mikudb";
    let client_param = format!("host={} user={} password='{}' dbname={}", HOST, USER, PASSWORD, DATABASE);
    let mut client = Client::connect(&client_param, NoTls)?;

    // client.batch_execute("
    //     CREATE TABLE person(
    //         id INT NOT NULL UNIQUE,
    //         name VARCHAR(30)
    //     )
    // ")?;

    // client.batch_execute("
    //     INSERT INTO person VALUES
    //     (1, 'Eric'),
    //     (2, 'Pratchaya');
    // ")?;


    for row in client.query("SELECT id, name FROM person", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        println!("found person: {} {}", id, name);
    }

    Ok(())
}
