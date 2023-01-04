/**
 * @file   service.rs
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Hope store commands
 * @date   2022-12-20
 *
 * @copyright Copyright (c) 2022
 */
use postgres::{Client, NoTls};
use std::{error::Error, io::Write};
use std::{io, io::ErrorKind};

use crate::command::*;
use crate::hope::*;

pub fn connect_db() -> Result<Client, Box<dyn Error>> {
    let host = dotenv::var("PG_HOST").unwrap_or_else(|_| "localhost".to_string());
    let user = dotenv::var("PG_USER").unwrap_or_else(|_| "postgres".to_string());
    let pwd = dotenv::var("PG_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    let db = dotenv::var("PG_DB").unwrap_or_else(|_| "postgres".to_string());
    let schema = dotenv::var("PG_SCHEMA").unwrap_or_else(|_| "public".to_string());

    std::io::stdout().flush()?;
    let mut client = Client::connect(
        &format!("host={host} user={user} password='{pwd}' dbname={db}"),
        NoTls,
    )?;
    client.execute(&format!("SET SCHEMA '{}'", schema), &[])?;
    Ok(client)
}

pub struct ClearCommand;
impl Command<()> for ClearCommand {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        // https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html#deletion
        print!("\u{001b}[2J\u{001b}[H");
        io::stdout().flush()?;
        Ok(())
    }
}

pub struct AddAddressCommand {
    pub street: String,
    pub postcode: String,
    pub city: String,
    pub country: String,
    pub telephone: String,
}
impl Command<i32> for AddAddressCommand {
    fn run(&self) -> Result<i32, Box<dyn Error>> {
        let mut db = connect_db()?;
        let insert = db.query_one(
            "INSERT INTO address(street, postcode, city, country, telephone)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *",
            &[
                &self.street,
                &self.postcode,
                &self.city,
                &self.country,
                &self.telephone,
            ],
        );
        if let Ok(ok) = insert {
            let id: i32 = ok.get("id");
            Ok(id)
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to create address",
            )))
        }
    }
}

pub struct RegiserCustomerCommand {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub street: String,
    pub postcode: String,
    pub city: String,
    pub country: String,
    pub telephone: String,
}
impl CustomerCommand for RegiserCustomerCommand {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let address_command = AddAddressCommand {
            street: self.street.to_owned(),
            postcode: self.postcode.to_owned(),
            city: self.city.to_owned(),
            country: self.country.to_owned(),
            telephone: self.telephone.to_owned(),
        };
        let address_id = address_command.run()?;

        let ok_insert = db.execute(
            "INSERT INTO customer(address_id, first_name, last_name, email, password)
             VALUES ($1, $2, $3, $4, $5)",
            &[
                &address_id,
                &self.first_name,
                &self.last_name,
                &self.email,
                &self.password,
            ],
        )?;

        if ok_insert == 1 {
            let login = LoginCustomerCommand {
                email: self.email.to_string(),
                password: self.password.to_string(),
            };
            login.run(customer)?;
            Ok(())
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to create user",
            )))
        }
    }
}

pub struct LoginAdminCommand {
    pub email: String,
    pub password: String,
}
impl AdminCommand for LoginAdminCommand {
    fn run(&self, admin: &mut Admin) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let email = db.query_one("SELECT email FROM admin WHERE email = $1", &[&self.email]);
        if let Err(_) = email {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Login failed: No admin with email: ".to_string() + &self.email,
            )));
        }

        let password = db.query_one(
            "SELECT * FROM admin WHERE email = $1 AND password = $2",
            &[&self.email, &self.password],
        );

        if let Err(_) = password {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Login failed: Invalid password",
            )));
        }

        if let Ok(login) = password {
            let id: i32 = login.get("id");
            let email: String = login.get("email");
            admin.login(&Admin::new(id, email));
            Ok(())
        } else {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Login failed: Unknown error",
            )));
        }
    }
}

pub struct LoginCustomerCommand {
    pub email: String,
    pub password: String,
}
impl CustomerCommand for LoginCustomerCommand {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let email = db.query_one(
            "SELECT email FROM customer WHERE email = $1",
            &[&self.email],
        );
        if let Err(_) = email {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Login failed: Invalid email",
            )));
        }

        let password = db.query_one("SELECT c.id, c.first_name, c.last_name, c.email,
                                         a.id as address_id, a.street, a.city, a.country, a.telephone
                                     FROM customer as c INNER JOIN address as a ON c.address_id = a.id
                                     WHERE c.email = $1 AND c.password = $2",
                                      &[&self.email, &self.password]);

        if let Err(_) = password {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Login failed: Invalid password",
            )));
        }

        if let Ok(login) = password {
            let id: i32 = login.get("id");
            let first_name: String = login.get("first_name");
            let last_name: String = login.get("last_name");
            let email: String = login.get("email");
            let address_id: i32 = login.get("address_id");
            let street: String = login.get("street");
            let city: String = login.get("city");
            let country: String = login.get("country");
            let telephone: String = login.get("telephone");
            let address = Address::new(address_id, street, city, country, telephone);
            customer.login(&Customer::new(id, first_name, last_name, email, address));
            Ok(())
        } else {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Login failed: Unknown error",
            )));
        }
    }
}

pub struct AddSupplierCommand {
    pub admin_id: i32,
    pub address_id: i32,
    pub name: String,
}
impl Command<()> for AddSupplierCommand {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let ok_insert = db.execute(
            "INSERT INTO supplier(admin_id, address_id, name) VALUES ($1, $2, $3)",
            &[&self.admin_id, &self.address_id, &self.name],
        )?;
        if ok_insert == 1 {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Not implemented",
            )))
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to create supplier",
            )))
        }
    }
}

pub struct AddProductCommand {
    pub supplier_id: i32,
    pub name: String,
    pub description: String,
    pub quantity: i32,
    pub price: f64,
}
impl Command<()> for AddProductCommand {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;

        let ok_insert = db.execute(
            "INSERT INTO product(supplier_id, name, description, quantity, price)
             VALUES ($1,$2,$3,$4)",
            &[&self.supplier_id, &self.name, &self.quantity, &self.price],
        )?;
        if ok_insert == 1 {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Not implemented",
            )))
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to create product",
            )))
        }
    }
}

pub struct ListProductsCommand;
impl Command<String> for ListProductsCommand {
    fn run(&self) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;

        let mut str = String::new();
        str.push_str(&"id, supplier_id, name, description, quantity, price\n");
        for row in db.query("SELECT id, supplier_id, name, description, quantity, CAST(price AS DOUBLE PRECISION) as price FROM product", &[])?{
            let product = Product{
                id: row.get(0),
                supplier_id: row.get(1),
                name: row.get(2),
                description: row.get(3),
                quantity: row.get(4),
                price: row.get(5),
            };
            str.push_str(&format!("{} {} {} {} {} {}\n", product.id, product.supplier_id, product.name, product.description, product.quantity, product.price));
        }
        Ok(str)
    }
}

pub struct AddToCart {
    pub product_id: i32,
    pub quantity: i32,
}
impl CustomerCommand for AddToCart {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let list_id_row = db.query_one(
            "SELECT id FROM item_list
                                   WHERE customer_id=$1",
            &[&customer.id()],
        )?;
        let list_id: i32 = list_id_row.get(0);
        let cart = db.query_one(
            "INSERT INTO item_cart ($1, $2, $3) 
                                ON CONFLICT UPDATE",
            &[&list_id, &self.product_id, &self.quantity],
        );
        Ok(())
    }
}

pub struct ShowCartCommand {}
impl CustomerCommand for ShowCartCommand {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let list_id_row = db.query_one(
            "SELECT id FROM item_list
                                   WHERE customer_id=$1",
            &[&customer.id()],
        )?;
        let list_id: i32 = list_id_row.get(0);
        let cart = db.query("SELECT * FROM item_cart WHERE list_id=$1", &[&list_id])?;

        for row in cart {
            let product_id: i32 = row.get(1);
            let product_row = db.query_one("SELECT * FROM product WHERE id=$1", &[&product_id])?;
            let product = Product {
                id: product_row.get(0),
                supplier_id: product_row.get(1),
                name: product_row.get(2),
                description: product_row.get(3),
                quantity: product_row.get(4),
                price: product_row.get(5),
            };
            let str = format!(
                "{} {} {} {} {} {}\n",
                product.id,
                product.supplier_id,
                product.name,
                product.description,
                product.quantity,
                product.price
            );
            println!("{}", str);
        }
        Ok(())
    }
}
