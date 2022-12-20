/**
 * @file   tui.rs
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Hope store - We sell hopes and dreams.
 *         Terminal user interface
 * @date   2022-12-20
 *
 * @copyright Copyright (c) 2022
 */
use std::{error::Error, io::Write};
use crate::hope::*;
use crate::command::*;
use crate::service::*;

fn read_input(label: &str) -> String {
    let mut input = String::new();
    print!("{}", label);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn admin_create_supplier() -> Result<(), Box<dyn Error>> {
    let name = read_input("name: ");

    let street = read_input("street: ");
    let postcode = read_input("postcode: ");
    let city = read_input("city: ");
    let country = read_input("country: ");
    let telephone = read_input("telephone nr: ");
    let address_command = AddAddressCommand {
        street,
        postcode,
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
    let name = read_input("name: ");
    let description = read_input("description: ");

    let quantity = read_input("quntity: ").parse::<i32>()?;
    let price = read_input("price: ").parse::<f64>()?;
    let supplier_id = read_input("supplier id: ").parse::<i32>()?;

    let add_cmd = AddProductCommand {
        supplier_id,
        name,
        description,
        quantity,
        price,
    };
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "create product Not implemented",
    )))
}

fn admin_home(admin: &mut Admin) {
    let pepper: &'static str = r#"
 /_/_  _  _    __/__  __
/ //_//_//_' _\ / /_///_'
     /              Admin"#
        .trim_start_matches('\n');

    println!("{pepper}");
    println!("{}", admin.to_string());
    loop {
        println!();
        println!("Commands available");
        println!("  1. Add new supplier");
        println!("  2. Add new product");
        println!("  3. Edit product");
        println!("  4. Delete product");
        println!("  5. Search for product");
        println!("  6. Add new discount");
        println!("  7. Assign discount");
        println!("  8. View discount history");
        println!("  9. Confirm order");
        println!(" 10. List products with max orders");
        println!("  0. Log out");
        println!();

        let choice = read_input(" option: ");
        let result = match choice.as_str() {
             "1" => admin_create_supplier(),
             "2" => admin_create_product(),
             "3" => Ok(()),
             "4" => Ok(()),
             "5" => Ok(()),
             "6" => Ok(()),
             "7" => Ok(()),
             "8" => Ok(()),
             "9" => Ok(()),
            "10" => Ok(()),
             "0" => break,
              _  => Ok(()),
        };

        match result {
            Ok(_) => {},
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
    ClearCommand{}.run().unwrap();
}

fn admin_main() {
    // Admin login
    let mut admin = Admin::default();

    println!("Login as admin, enter the credentials");
    loop {
        let email = read_input(" email: ");
        let password = read_input(" password: ");
        let login = LoginAdminCommand{email, password};

        match login.run(&mut admin){
            Ok(()) => break,
            Err(error) => {
                println!("{}", error);
                let input = read_input("Do you want to try again? Y/n: ");
                if input.eq_ignore_ascii_case("n") || !input.eq_ignore_ascii_case("y") || input.is_empty() {
                    break;
                }
            },
        };
    }

    if !admin.is_login() {
        ClearCommand{}.run().unwrap();
        return;
    }

    // Admin home
    ClearCommand{}.run().unwrap();
    admin_home(&mut admin);
}

fn list_all_products() -> Result<(), Box<dyn Error>> {
    let list_cmd = ListProductsCommand{};
    let str = list_cmd.run()?;
    println!("{}", str);
    Ok(())
}

fn customer_main() {
    let mut customer = Customer::default();
    loop {
        let email = read_input("email: ");
        let password = read_input("password: ");
        let login = LoginCustomerCommand{email, password};

        match login.run(&mut customer){
            Ok(()) => break,
            Err(error) => {
                println!("{}", error);
                let input = read_input("Do you want to try again? Y/n: ");
                if input.eq_ignore_ascii_case("n") || !input.eq_ignore_ascii_case("y") || input.is_empty() {
                    break;
                }
            },
        };
    }

    if !customer.is_login() {
        return;
    }

    let banner_slanted: &'static str = r#"
    __  __                         __                
   / / / /___  ____  ___     _____/ /_____  ________ 
  / /_/ / __ \/ __ \/ _ \   / ___/ __/ __ \/ ___/ _ \
 / __  / /_/ / /_/ /  __/  (__  ) /_/ /_/ / /  /  __/
/_/ /_/\____/ .___/\___/  /____/\__/\____/_/   \___/ 
           /_/                     Hopes and dreams"#.trim_start_matches('\n');

    ClearCommand{}.run().unwrap();
    println!("{}", banner_slanted);
    println!("{}\n", customer.to_string());

    loop {

        println!("1. Browse product");
        println!("2. Search product");
        println!("3. Add to shopping cart");
        println!("4. Show shopping cart");
        println!("5. Show orders");
        println!("6. Delete an order");
        println!("0. Log out");

        let input = read_input(" option: ");
        let result = match input.as_str() {
            "1" => list_all_products(),
            "2" => Ok(()),
            "3" => Ok(()),
            "4" => Ok(()),
            "5" => Ok(()),
            "6" => Ok(()),
            "0" => break,
             _  => Ok(()),
        };

        match result {
            Ok(_) => {},
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
    println!("Logging out.....");
}

fn register_main() {
    let clear = ClearCommand{};
    clear.run().expect("");

    println!("Register as new customer");
    loop {
        let email = read_input("email: ");
        let password = read_input("password: ");
        let first_name = read_input("first name: ");
        let last_name = read_input("last name: ");
        let city = read_input("city: ");
        let street = read_input("street: ");
        let postcode = read_input("postcode: ");
        let country = read_input("country: ");
        let telephone = read_input("telephone nr: ");

        let signup_command = RegiserCustomerCommand {
            first_name,
            last_name,
            email,
            password,
            street,
            postcode,
            city,
            country,
            telephone,
        };
        let mut customer = Customer::default();
        match signup_command.run(&mut customer) {
            Ok(_) => break,
            Err(err) => {
                eprintln!("{}", err);
                let choice = read_input("Try again? Y/n: ");
                if choice == "n" {
                    break;
                }
            }
        }
    }
}

pub fn tui_main() {
    let mut err_msg = String::new();
    loop {
        ClearCommand{}.run().unwrap();
        println!("{}", BANNER_SPEED.trim_start_matches('\n'));

        println!("Log in as:");
        println!(" 1. Admin");
        println!(" 2. Customer");
        println!(" 3. Register as customer");
        println!(" 0. Exit");
        if !err_msg.is_empty() {
            eprintln!(" {}", err_msg);
            err_msg = String::new();
        }
        let choice = read_input(" option: ");

        match choice.as_str() {
            "1" => admin_main(),
            "2" => customer_main(),
            "3" => register_main(),
            "0" => break,
            _ => err_msg = "Invalid choice".to_string()
        }
    }
}
