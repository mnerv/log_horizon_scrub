use crate::admin_service::*;
use crate::command::*;
use crate::common_service::*;
use crate::customer_service::*;
use crate::hope::*;
use chrono::NaiveDateTime;
use std::{error::Error, io::Write};

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

fn read_input(label: &str) -> String {
    let mut input = String::new();
    print!("{}", label);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_date(label: &str) -> Result<NaiveDateTime, Box<dyn Error>> {
    let date_str = read_input(label);
    let parse_from_str = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S");
    if let Ok(parsed) = parse_from_str {
        return Ok(parsed);
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to parsed date",
    )))
}

fn admin_create_supplier(admin: &mut Admin) -> Result<(), Box<dyn Error>> {
    let name = read_input("name: ");
    let description = read_input("description: ");
    let org_num = read_input("organization number: ");

    let street = read_input("street: ");
    let postcode = read_input("postcode: ");
    let city = read_input("city: ");
    let country = read_input("country: ");
    let telephone = read_input("telephone nr: ");

    let cmd = AddSupplierCommand {
        name,
        description,
        org_num,
        street,
        postcode,
        city,
        country,
        telephone,
    };
    cmd.run(admin)?;
    Ok(())
}

fn admin_create_product(admin: &mut Admin) -> Result<(), Box<dyn Error>> {
    let name = read_input("name: ");
    let description = read_input("description: ");

    let quantity = read_input("quntity: ").parse::<i32>()?;
    let price = read_input("price: ").parse::<f64>()?;
    let supplier_id = read_input("supplier id: ").parse::<i32>()?;

    let cmd = AddProductCommand {
        supplier_id,
        name,
        description,
        quantity,
        price,
    };
    cmd.run(admin)?;
    Ok(())
}

fn admin_edit_product_quantity(admin: &mut Admin) -> Result<(), Box<dyn Error>> {
    let quantity = read_input("quantity: ").parse::<i32>()?;
    let product_id = read_input("product id: ").parse::<i32>()?;
    let edit_cmd = EditProductQuantityCommand {
        product_id,
        quantity,
    };
    edit_cmd.run(admin)?;
    Ok(())
}

fn delete_product(admin: &mut Admin) -> Result<(), Box<dyn Error>> {
    let product_id = read_input("product id: ").parse::<i32>()?;
    let delete_cmd = DeleteProductCommand { product_id };
    delete_cmd.run(admin)?;
    Ok(())
}

fn add_new_discount(admin: &mut Admin) -> Result<(), Box<dyn Error>> {
    let code = read_input("code: ");
    let name = read_input("name: ");
    let start = read_date("start: ")?;
    let end = read_date("end: ")?;

    let add_cmd = AddNewDiscountCommand {
        code,
        name,
        start,
        end,
    };
    add_cmd.run(admin)?;
    Ok(())
}

fn view_discount_history(admin: &mut Admin) -> Result<(), Box<dyn Error>> {
    let view_cmd = ViewDiscountHistoryCommand {};
    let str = view_cmd.run(admin)?;
    println!("{}", str);
    Ok(())
}

fn assign_discount(admin: &mut Admin) -> Result<(), Box<dyn Error>> {
    let discount_id = read_input("discount id: ").parse::<i32>()?;
    let product_id = read_input("product id: ").parse::<i32>()?;
    let factor = read_input("factor: ").parse::<f64>()?;
    let assign_cmd = AssignDiscountCommand {
        discount_id,
        product_id,
        factor,
    };
    assign_cmd.run(admin)?;
    Ok(())
}

fn view_unconfirmed_orders(admin: &mut Admin) -> Result<(), Box<dyn Error>> {
    let view_cmd = ViewUnconfirmedOrdersCommand {};
    let str = view_cmd.run(admin)?;
    println!("{}", str);
    Ok(())
}

fn confirm_order(admin: &mut Admin) -> Result<(), Box<dyn Error>> {
    let order_id: i32 = read_input("order id: ").parse::<i32>()?;
    let cmd = ConfirmOrderCommand { order_id };
    cmd.run(admin)?;
    Ok(())
}

fn list_product_max_order(admin: &mut Admin) -> Result<(), Box<dyn Error>> {
    let cmd = ListProductsMaxOrderCommand {};
    let str = cmd.run(admin)?;
    println!("{}", str);
    Ok(())
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
        println!("  5. List product");
        println!("  6. Search for product");
        println!("  7. Add new discount");
        println!("  8. Assign discount");
        println!("  9. View discounted products");
        println!(" 10. View discount history");
        println!(" 11. List unconfirmed order");
        println!(" 12. Confirm order");
        println!(" 13. List products with max orders");
        println!("  0. Log out");
        println!();

        let choice = read_input(" option: ");
        let result = match choice.as_str() {
            "1" => admin_create_supplier(admin),
            "2" => admin_create_product(admin),
            "3" => admin_edit_product_quantity(admin),
            "4" => delete_product(admin),
            "5" => list_all_products(),
            "6" => search_product(),
            "7" => add_new_discount(admin),
            "8" => assign_discount(admin),
            "9" => show_discounted_products(),
            "10" => view_discount_history(admin),
            "11" => view_unconfirmed_orders(admin),
            "12" => confirm_order(admin),
            "13" => list_product_max_order(admin),
            "0" => break,
            _ => Ok(()),
        };

        match result {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
    ClearCommand {}.run().unwrap();
}

fn admin_main() {
    // Admin login
    let mut admin = Admin::default();

    println!("Login as admin, enter the credentials");
    loop {
        let email = read_input(" email: ");
        let password = read_input(" password: ");
        let login = LoginAdminCommand { email, password };

        match login.run(&mut admin) {
            Ok(()) => break,
            Err(error) => {
                println!("{}", error);
                let input = read_input("Do you want to try again? Y/n: ");
                if input.eq_ignore_ascii_case("n")
                    || !input.eq_ignore_ascii_case("y")
                    || input.is_empty()
                {
                    break;
                }
            }
        };
    }

    if !admin.is_login() {
        ClearCommand {}.run().unwrap();
        return;
    }

    // Admin home
    ClearCommand {}.run().unwrap();
    admin_home(&mut admin);
}

fn list_all_products() -> Result<(), Box<dyn Error>> {
    let list_cmd = ListProductsCommand {};
    let str = list_cmd.run()?;
    println!("{}", str);
    Ok(())
}

fn add_to_cart(customer: &mut Customer) -> Result<(), Box<dyn Error>> {
    let product_id = read_input("Product id:").parse::<i32>().unwrap();
    let quantity = read_input("Quantity:").parse::<i32>().unwrap();

    let add_cmd = AddToCart {
        product_id,
        quantity,
    };

    add_cmd.run(customer)?;
    Ok(())
}

fn show_cart(customer: &mut Customer) -> Result<(), Box<dyn Error>> {
    let show_cmd = ShowCartCommand {};
    let str = show_cmd.run(customer)?;
    println!("{}", str);
    Ok(())
}

fn show_orders(customer: &mut Customer) -> Result<(), Box<dyn Error>> {
    let show_cmd = ShowOrdersCommand {};
    let str = show_cmd.run(customer)?;
    println!("{}", str);
    Ok(())
}

fn checkout(customer: &mut Customer) -> Result<(), Box<dyn Error>> {
    let checkout_cmd = CheckoutCommand {
        date_time: NaiveDateTime::from(chrono::offset::Local::now().naive_utc()),
    };
    checkout_cmd.run(customer)?;
    Ok(())
}

fn search_product() -> Result<(), Box<dyn Error>> {
    let search_str = read_input("search: ");
    let search_cmd = SearchProductCommand { search_str };
    let str = search_cmd.run()?;
    println!("{}", str);
    Ok(())
}

fn delete_order(customer: &mut Customer) -> Result<(), Box<dyn Error>> {
    let order_id = read_input("Order id:").parse::<i32>().unwrap();
    let delete_cmd = DeleteOrderCommand { order_id };
    delete_cmd.run(customer)?;
    Ok(())
}

fn show_discounted_products() -> Result<(), Box<dyn Error>> {
    let show_cmd = ShowDiscountedProductsCommand {};
    let str = show_cmd.run()?;
    println!("{}", str);
    Ok(())
}

fn customer_main() {
    let mut customer = Customer::default();
    loop {
        let email = read_input("email: ");
        let password = read_input("password: ");
        let login = LoginCustomerCommand { email, password };

        match login.run(&mut customer) {
            Ok(()) => break,
            Err(error) => {
                println!("{}", error);
                let input = read_input("Do you want to try again? Y/n: ");
                if input.eq_ignore_ascii_case("n")
                    || !input.eq_ignore_ascii_case("y")
                    || input.is_empty()
                {
                    break;
                }
            }
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
           /_/                     Hopes and dreams"#
        .trim_start_matches('\n');

    ClearCommand {}.run().unwrap();
    println!("{}", banner_slanted);
    println!("{}\n", customer.to_string());

    loop {
        println!("1. Browse product");
        println!("2. Search product");
        println!("3. Show discounted products");
        println!("4. Add to shopping cart");
        println!("5. Show shopping cart");
        println!("6. Show orders");
        println!("7. Delete an order");
        println!("8. Checkout");
        println!("0. Log out");

        let input = read_input("option: ");
        let result = match input.as_str() {
            "1" => list_all_products(),
            "2" => search_product(),
            "3" => show_discounted_products(),
            "4" => add_to_cart(&mut customer),
            "5" => show_cart(&mut customer),
            "6" => show_orders(&mut customer),
            "7" => delete_order(&mut customer),
            "8" => checkout(&mut customer),
            "0" => break,
            _ => Ok(()),
        };

        match result {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
    println!("Logging out.....");
}

fn register_main() {
    let clear = ClearCommand {};
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
        ClearCommand {}.run().unwrap();
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
        let choice = read_input("option: ");

        match choice.as_str() {
            "1" => admin_main(),
            "2" => customer_main(),
            "3" => register_main(),
            "0" => break,
            _ => err_msg = "Invalid choice".to_string(),
        }
    }
}
