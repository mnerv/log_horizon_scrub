use std::fs::read;
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
use std::{error::Error, io::Write};

mod command;
mod hope;
mod service;

use hope::LockStatus;

use crate::command::Command;
use crate::hope::Hope;
use crate::hope::HopeMode;
use crate::service::*;

fn read_input(label: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    print!("{}", label);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn run_command<C: Command>(
    store: &mut Hope,
    mut command: C,
) -> Result<Option<Box<(dyn std::any::Any + 'static)>>, Box<(dyn std::error::Error + 'static)>> {
    return command.run(store);
}

fn admin_create_supplier(store: &mut Hope) -> Result<(), Box<dyn Error>> {
    let admin_id = store.user.id();

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

    let maybe_id = run_command(store, address_command)?;

    let address_id = match maybe_id {
        Some(value) => match value.downcast_ref::<i32>() {
            Some(v) => v.to_owned(),
            _ => 0,
        },
        None => 0,
    };

    let add_supplier = AddSupplierCommand {
        admin_id,
        address_id,
        name,
    };
    run_command(store, add_supplier)?;
    Ok(())
}

fn admin_home(store: &mut Hope) -> Result<(), Box<dyn Error>> {
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
            "1" => {}
            "2" => {}
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

fn admin(store: &mut Hope) -> Result<(), Box<dyn Error>> {
    // Login
    loop {
        let email = read_input("email: ")?;
        let password = read_input("password: ")?;
        let login_command = LoginCommand {
            mode: HopeMode::Admin,
            email,
            password,
        };
        run_command(store, login_command);

        if store.status == LockStatus::LogIn {
            break;
        }
    }

    admin_home(store)?;
    Ok(())
}

fn customer(store: &mut Hope) -> Result<(), Box<dyn Error>> {
    loop {
        println!("1. Browse product");
        println!("2. Search product");
        println!("3. Add to shopping cart");
        println!("4. Show shopping cart");
        println!("5. Show orders");
        println!("6. Delete an order");
    }
    Ok(())
}

fn signup(store: &mut Hope) -> Result<(), Box<dyn Error>> {
    let email = read_input("email: ")?;
    let password = read_input("password: ")?;
    let first_name = read_input("first name: ")?;
    let last_name = read_input("last name: ")?;
    let city = read_input("city: ")?;
    let street = read_input("street: ")?;
    let country = read_input("country: ")?;
    let telephone = read_input("telephone nr: ")?;

    let signup_command = SignupCommand {
        first_name,
        last_name,
        email,
        password,
        street,
        city,
        country,
        telephone,
    };

    run_command(store, signup_command);
    Ok(())
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

    let mut store = Hope::new();
    loop {
        println!("Log in as:");
        println!("1. Admin");
        println!("2. Customer");
        println!("3. Sign up");
        println!("0. Exit");
        let choice = read_input("Input: ").expect("Can't read input");

        match choice.as_str() {
            "1" => admin(&mut store).expect(""),
            "2" => customer(&mut store).expect(""),
            "3" => signup(&mut store).expect(""),
            "0" => break,
            _ => {
                println!("Invalid choice!!!");
            }
        }
    }
    println!("Goodbye cruel world...");
}
