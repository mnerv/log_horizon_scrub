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

struct Login {
    id: i32, 
    email: String
}

fn add_supplier(client: &mut Client, admin: &Login)-> Result<(), Box<dyn Error>>{
    Ok(())
}

fn login_page(client: &mut Client) -> Result<Login, Box<dyn Error>> {
    loop {
        let email = read_input("email: ")?;
        let password = read_input("password: ")?;
        let row = client.query("SELECT id, email FROM admin 
                                WHERE email=$1 AND password=$2 ", &[&email, &password])?;
        if row.len() == 1 {
           let id: i32 = row[0].get("id");
           println!("Login complete id: {}", id); 
           return Ok(Login { id, email });
        }else{
           println!("Login failed try again");
           panic!("Login failed TODO handle this");
        }
    }
}

fn admin_home (client: &mut Client, admin: &Login)-> Result<(), Box<dyn Error>>{
    loop {
        println!(" 1. Add new supplier");
        println!(" 2. Add new product");
        println!(" 3. Edit product");
        println!(" 4. Delete product");
        println!(" 5. Search for product");
        println!(" 6. Add new discount");
        println!(" 7. Assign discount");
        println!(" 8. View discount history");
        println!(" 9. Confirm order");
        println!("10. List products with max orders");
        println!(" 0. Log out");
        let choice = read_input("Input: ")?;
        match choice.as_str() {
            "1"  => {}
            "2"  => {}
            "3"  => {}
            "4"  => {}
            "5"  => {}
            "6"  => {}
            "7"  => {}
            "8"  => {}
            "9"  => {}
            "10" => {}
            "0"  => {
                println!("Logging out...");
                break;
            }
            _ =>{}
        }
    }
    Ok(())
}


fn admin(client: &mut Client) -> Result<(), Box<dyn Error>>{
    let login = login_page(client)?;
    admin_home(client, &login)?;
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

    println!("    Welcome to Hope store!");
    println!("Here we sell hopes and dreams :)");

    loop {
        println!("Log in as:");
        println!("1. Admin");
        println!("2. Customer");
        println!("0. Exit");
        let choice = read_input("Input: ")?;

        match choice.as_str() {
            "1" => {
                admin(&mut client)?;
            },
            "2" => {
            }
            "0" => {
                println!("Goodbye cruel world...");
                break;
            }
            _ => {
                println!("Invalid choice!!!");
            }
        }
    }
    client.close()?;
    Ok(())
}
