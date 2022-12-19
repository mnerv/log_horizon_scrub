use std::any::Any;
use std::{error::Error, io::Write};

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
mod command;
mod hope;
mod service;

use command::CustomerCommand;
use hope::*;

use crate::command::Command;
use crate::service::*;

fn read_input(label: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    print!("{}", label);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn admin_create_supplier() -> Result<(), Box<dyn Error>> {
    let name = read_input("name: ")?;

    let street = read_input("street: ")?;
    let city = read_input("city: ")?;
    let country = read_input("country: ")?;
    let telephone = read_input("telephone nr: ")?;
    let address_command = AddAddressCommand {
        street,
        city,
        country,
        telephone,
    };

    //let add_supplier = AddSupplierCommand {
    //    admin_id,
    //    address_id: maybe_address.unwrap().to_owned(),
    //    name,
    //};
    //run_command(add_supplier);
    Ok(())
}

fn admin_create_product() -> Result<(), Box<dyn Error>> {
    let name = read_input("name: ")?;
    let quantity = read_input("quntity: ")?;
    let price = read_input("price: ")?;
    let supplier_id_str = read_input("supplier id: ")?;
    let supplier_id = supplier_id_str.parse::<i32>()?;

    let add_cmd = AddProductCommand {
        supplier_id,
        name,
        quantity,
        price,
    };

    //let succeded = run_command(add_cmd);
    Ok(())
}

fn admin_home() -> Result<(), Box<dyn Error>> {
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
            "1" => admin_create_supplier()?,
            "2" => admin_create_product()?,
            "3" => {}
            "4" => {}
            "5" => {}
            "6" => {}
            "7" => {}
            "8" => {}
            "9" => {}
            "10" => {}
            "0" => {
                println!("Logging out...");
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

fn admin_main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn customer_main() -> Result<(), Box<dyn Error>> {
    let email = read_input("email: ").unwrap();
    let password = read_input("password: ").unwrap();

    let login = LoginCustomerCommand{email, password};
    let mut customer = Customer::default();
    login.run(&mut customer)?;

    loop {
        println!("1. Browse product");
        println!("2. Search product");
        println!("3. Add to shopping cart");
        println!("4. Show shopping cart");
        println!("5. Show orders");
        println!("6. Delete an order");
        break;
    }
    Ok(())
}

fn register_main() {
    let clear = ClearCommand{};
    clear.run().expect("");

    println!("Register as new customer");
    let email = read_input("email: ").unwrap();
    let password = read_input("password: ").unwrap();
    let first_name = read_input("first name: ").unwrap();
    let last_name = read_input("last name: ").unwrap();
    let city = read_input("city: ").unwrap();
    let street = read_input("street: ").unwrap();
    let country = read_input("country: ").unwrap();
    let telephone = read_input("telephone nr: ").unwrap();

    let signup_command = RegiserCustomerCommand {
        first_name,
        last_name,
        email,
        password,
        street,
        city,
        country,
        telephone,
    };
    let mut customer = Customer::default();
    signup_command.run(&mut customer).expect("");
}

fn main() {
    let pepper: &'static str = r#"
 /_/_  _  _    __/__  __
/ //_//_//_' _\ / /_///_'
     /                   "#
        .trim_start_matches('\n');
    let slanted: &'static str = r#"
    __  __                         __                
   / / / /___  ____  ___     _____/ /_____  ________ 
  / /_/ / __ \/ __ \/ _ \   / ___/ __/ __ \/ ___/ _ \
 / __  / /_/ / /_/ /  __/  (__  ) /_/ /_/ / /  /  __/
/_/ /_/\____/ .___/\___/  /____/\__/\____/_/   \___/ 
           /_/                     Hopes and dreams"#
        .trim_start_matches('\n');
    let speed: &'static str = r#"
______  __                              _____                   
___  / / /__________________     _________  /__________________ 
__  /_/ /_  __ \__  __ \  _ \    __  ___/  __/  __ \_  ___/  _ \
_  __  / / /_/ /_  /_/ /  __/    _(__  )/ /_ / /_/ /  /   /  __/
/_/ /_/  \____/_  .___/\___/     /____/ \__/ \____//_/    \___/ 
               /_/                 Hopes and dreams"#
        .trim_start_matches('\n');

    loop {
        println!("Log in as:");
        println!("1. Admin");
        println!("2. Customer");
        println!("3. Register as customer");
        println!("0. Exit");
        let choice = read_input("Input: ").expect("Can't read input");

        match choice.as_str() {
            "1" => admin_main().expect(""),
            "2" => customer_main().expect(""),
            "3" => register_main(),
            "0" => break,
            _ => {
                println!("Invalid choice!!!");
            }
        }
    }
    println!("Goodbye cruel world...");
}
