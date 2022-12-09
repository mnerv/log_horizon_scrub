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
use std::io;
use std::io::Write;
fn admin(mut client: Client) -> Result<(), Box<dyn Error>>{

/* admin
 *
 * TODO:
 * Login 
 * add supplier 
 * add product
 * edit product 
 * delete product 
 * list products - search
 * add discounts 
 * assign discounts 
 * show discount history
 * confirm order 
 * see a list of products with maximum orders in each mont
 */
   Ok(()) 
}

fn read_input(label: &str) -> Result<String ,Box<dyn Error>>{
    let mut input = String::new();

    print!("{}", label);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let host = dotenv::var("PG_HOST")?;
    let user = dotenv::var("PG_USER")?;
    let password = dotenv::var("PG_PASSWORD")?;
    let db       = dotenv::var("PG_DB")?;
    let schema   = dotenv::var("PG_SCHEMA")?;
    let mut client = Client::connect(&format!("host={host} user={user} password='{password}' dbname={db}"), NoTls)?;

    client.execute(&format!("SET SCHEMA '{schema}'"), &[])?; 

    let mut email = String::new();
    let mut password = String::new();

    println!("    Welcome to Hope store!");
    println!("Here we sell hopes and dreams :)");
    
    email = read_input("email: ")?;
    password = read_input("password: ")?;

    let row = client.query("SELECT id, email , password FROM admin 
                           WHERE email=$1 AND password=$2 ", &[&email.trim(), &password.trim()])?;
    println!("{}", row.len());
    if row.len() > 0 {
        let id: i32 = row[0].get("id");
        println!("Login complete id: {}", id); 
    }else{
        println!("Login failed");
        panic!("yoooooooo this incident will be reported");
    }

    client.close()?;
    Ok(())
}
