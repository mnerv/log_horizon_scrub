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
mod service;
mod hope;

use hope::LockStatus;

use crate::command::Command;
use crate::service::*;
use crate::hope::Hope;
use crate::hope::HopeMode;

fn read_input(label: &str) -> Result<String ,Box<dyn Error>> {
    let mut input = String::new();
    print!("{}", label);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn run_command<C: Command>(store: &mut Hope, mut command: C) {
    match command.run(store) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err)
    }
}

fn admin_home (client: &mut Hope)-> Result<(), Box<dyn Error>>{
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

fn admin(store: &mut Hope) -> Result<(), Box<dyn Error>> {
    // Login
    loop {
        let email = read_input("email: ")?;
        let password = read_input("password: ")?;
        let login_command = LoginCommand{
            mode: HopeMode::Admin,
            email, password,
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
    Ok(())
}

fn main() {
    let pepper: &'static str = r#"
 /_/_  _  _    __/__  __
/ //_//_//_' _\ / /_///_'
     /                   "#.trim_start_matches('\n');
    let slanted: &'static str = r#"
    __  __                         __                
   / / / /___  ____  ___     _____/ /_____  ________ 
  / /_/ / __ \/ __ \/ _ \   / ___/ __/ __ \/ ___/ _ \
 / __  / /_/ / /_/ /  __/  (__  ) /_/ /_/ / /  /  __/
/_/ /_/\____/ .___/\___/  /____/\__/\____/_/   \___/ 
           /_/                     Hopes and dreams"#.trim_start_matches('\n');
    let speed: &'static str = r#"
______  __                              _____                   
___  / / /__________________     _________  /__________________ 
__  /_/ /_  __ \__  __ \  _ \    __  ___/  __/  __ \_  ___/  _ \
_  __  / / /_/ /_  /_/ /  __/    _(__  )/ /_ / /_/ /  /   /  __/
/_/ /_/  \____/_  .___/\___/     /____/ \__/ \____//_/    \___/ 
               /_/                 Hopes and dreams"#.trim_start_matches('\n');

    let mut store = Hope::new();
    loop {
        println!("Log in as:");
        println!("1. Admin");
        println!("2. Customer");
        println!("0. Exit");
        let choice = read_input("Input: ").expect("Can't read input");

        match choice.as_str() {
            "1" => admin(&mut store).expect(""),
            "2" => customer(&mut store).expect(""),
            "0" => break,
            _ => {
                println!("Invalid choice!!!");
            }
        }
    }
    println!("Goodbye cruel world...");
}
