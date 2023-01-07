/**
 * @file   service.rs
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Hope store commands
 * @date   2022-12-20
 *
 * @copyright Copyright (c) 2022
 */
use chrono::NaiveDateTime;
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
impl CustomerCommand<()> for RegiserCustomerCommand {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;

        let ok_insert = db.execute(
            "CALL insert_customer($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            &[
                &self.first_name,
                &self.last_name,
                &self.email,
                &self.password,
                &self.street,
                &self.postcode,
                &self.city,
                &self.country,
                &self.telephone,
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
impl AdminCommand<()> for LoginAdminCommand {
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
impl CustomerCommand<()> for LoginCustomerCommand {
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
    pub name: String,
    pub description: String,
    pub orgnum: String,
    pub street: String,
    pub postcode: String,
    pub city: String,
    pub country: String,
    pub telephone: String,
}
impl Command<()> for AddSupplierCommand {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let ok_insert = db.execute(
            "CALL insert_supplier($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            &[
                &self.admin_id,
                &self.name,
                &self.description,
                &self.orgnum,
                &self.street,
                &self.postcode,
                &self.city,
                &self.country,
                &self.telephone,
            ],
        )?;

        if ok_insert == 1 {
            Ok(())
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
             VALUES ($1, $2, $3, $4)",
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
            str.push_str(&format!("{}, {},{}, {}, {}, {}\n", product.id, product.supplier_id, product.name, product.description, product.quantity, product.price));
        }
        Ok(str)
    }
}

pub struct AddToCart {
    pub product_id: i32,
    pub quantity: i32,
}
impl CustomerCommand<()> for AddToCart {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;

        let cart_row = db.query_one(
            "SELECT cart.id FROM cart INNER JOIN customer ON customer.id = $1",
            &[&customer.id()],
        );

        let mut cart_id: i32 = 0;
        if let Ok(cart) = cart_row {
            cart_id = cart.get("id");
        } else {
            db.execute(
                "INSERT INTO cart(customer_id, updated) VALUES ($1, CURRENT_TIMESTAMP)",
                &[&customer.id()],
            )?;
        }

        // Check the order_id, product_id primary keys
        let cart_item_exist = db.query_one(
            "SELECT product_id, quantity FROM cart_item WHERE cart_id = $1 AND product_id = $2",
            &[&cart_id, &self.product_id],
        );

        let product = db.query_one("SELECT * FROM product WHERE id = $1", &[&self.product_id])?;

        let is_ok: bool;

        if let Ok(cart_item) = cart_item_exist {
            let product_id: i32 = cart_item.get(0);
            let quantity: i32 = cart_item.get(1);

            let new_quantity = quantity + self.quantity;

            let product_quantity: i32 = product.get("quantity");
            if new_quantity > product_quantity {
                return Err(Box::new(io::Error::new(
                    ErrorKind::Other,
                    format!(
                        "Can't add more than current available stock: in stock {}",
                        product_quantity
                    ),
                )));
            }

            // FIXME: Add and Remove the quantity and also check if it is possible to do that with in stock quntity.
            db.execute(
                "UPDATE cart_item SET quantity = $1 WHERE cart_id = $2 AND product_id = $3",
                &[&new_quantity, &cart_id, &product_id],
            )?;
            is_ok = true;
        } else {
            db.execute(
                "INSERT INTO cart_item VALUES ($1, $2, $3)",
                &[&cart_id, &self.product_id, &self.quantity],
            )?;
            is_ok = true;
        }

        if is_ok {
            db.execute(
                "UPDATE cart SET updated = CURRENT_TIMESTAMP WHERE id = $1",
                &[&cart_id],
            )?;
            Ok(())
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to add product to cart",
            )))
        }
    }
}

pub struct ShowCartCommand {}
impl CustomerCommand<String> for ShowCartCommand {
    fn run(&self, customer: &mut Customer) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;
        let cart_row = db.query_one(
            "SELECT id FROM cart WHERE customer_id=$1",
            &[&customer.id()],
        )?;
        let cart_id: i32 = cart_row.get(0);
        let cart = db.query("SELECT * FROM cart_item WHERE cart_id=$1", &[&cart_id])?;

        // FIXME: Maybe return a better looking formatting
        let mut str: String = "id, name, price, quantity, sum\n".to_string();
        for row in cart {
            let product_id: i32 = row.get("product_id");
            let quantity: i32 = row.get("quantity");
            let product_row = db.query_one(
                "SELECT name, CAST(price AS DOUBLE PRECISION) as price FROM product WHERE id=$1",
                &[&product_id],
            )?;

            let name: String = product_row.get("name");
            let price: f64 = product_row.get("price");
            let sum = price * f64::from(quantity);

            str.push_str(&format!(
                "{product_id}, {name}, {price}, {quantity}, {sum}\n"
            ));
        }
        Ok(str)
    }
}

pub struct CheckoutCommand {}
impl CustomerCommand<()> for CheckoutCommand {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;

        let cart_row = db.query_one(
            "SELECT id FROM cart WHERE customer_id = $1",
            &[&customer.id()],
        )?;

        let cart_id: i32 = cart_row.get(0);

        let order_row = db.query_one(
            "INSERT INTO orders (customer_id, created, status)
             VALUES ($1, CURRENT_TIMESTAMP, $2)
             RETURNING id",
            &[&customer.id(), &"pending"],
        );

        if let Ok(order) = order_row {
            let order_id: i32 = order.get("id");
            let items = db.query("SELECT * FROM cart_item WHERE cart_id = $1", &[&cart_id])?;

            let mut transaction = db.transaction()?;
            let insert_item = transaction.prepare(
                "INSERT INTO order_item(order_id, product_id, quantity) VALUES ($1, $2, $3)",
            )?;

            let product_q = transaction.prepare("SELECT * FROM product WHERE id = $1")?;
            let dec_quantity =
                transaction.prepare("UPDATE product SET quantity = $1 WHERE id = $2")?;

            for item in items {
                let product_id: i32 = item.get("product_id");
                let quantity: i32 = item.get("quantity");
                let product_row = transaction.query_one(&product_q, &[&product_id])?;
                let product_quantity: i32 = product_row.get("quantity");

                let new_quantity = product_quantity - quantity;
                if new_quantity < 0 {
                    transaction.rollback()?;
                    return Err(Box::new(io::Error::new(
                        ErrorKind::Other,
                        "Failed to checkout cart",
                    )));
                }
                transaction.execute(&dec_quantity, &[&new_quantity, &product_id])?;
                transaction.execute(&insert_item, &[&order_id, &product_id, &quantity])?;
            }

            transaction.execute("DELETE FROM cart_item WHERE cart_id = $1", &[&cart_id])?;
            transaction.commit()?;

            Ok(())
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to checkout cart",
            )))
        }
    }
}

pub struct DeleteOrderCommand {
    pub order_id: i32,
}
impl CustomerCommand<()> for DeleteOrderCommand {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;

        let order_row = db.query_one(
            "SELECT id FROM orders WHERE customer_id = $1 AND id = $2 AND confirmed_by_admin IS NULL",
            &[&customer.id(), &self.order_id],
        )?;

        let order_id: i32 = order_row.get(0);

        let order_item_rows =
            db.query("SELECT * FROM order_item WHERE order_id = $1", &[&order_id])?;

        for row in order_item_rows {
            let product_id: i32 = row.get("product_id");
            let quantity: i32 = row.get("quantity");

            let product_row =
                db.query_one("SELECT * FROM product WHERE id = $1", &[&product_id])?;
            let product_quantity: i32 = product_row.get("quantity");

            let new_quantity = product_quantity + quantity;

            db.execute(
                "UPDATE product SET quantity = $1 WHERE id = $2",
                &[&new_quantity, &product_id],
            )?;
        }

        db.execute(
            "DELETE FROM orders WHERE customer_id = $1 AND id = $2 AND confirmed_by_admin IS NULL",
            &[&customer.id(), &self.order_id],
        )?;

        Ok(())
    }
}

pub struct ShowOrdersCommand {}
impl CustomerCommand<String> for ShowOrdersCommand {
    fn run(&self, customer: &mut Customer) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;

        //let order_call_rows = db.query("CALL show_orders($1)", &[&customer.id()])?;
        let order_rows = db.query(
            "SELECT orders.id, status, CAST(SUM(product.price * order_item.quantity) AS DOUBLE PRECISION) AS price, orders.created FROM orders
             INNER JOIN order_item ON orders.id=order_item.order_id
             INNER JOIN product ON order_item.product_id = product.id
             WHERE customer_id=$1
             GROUP BY orders.id",
            &[&customer.id()],
        )?;

        let mut str: String = "id, date, name, status, price\n".to_string();
        for order in order_rows {
            let id: i32 = order.get("id");
            let status: String = order.get("status");
            let price: f64 = order.get("price");
            let date: NaiveDateTime = order.get("created");

            str.push_str(&format!("{id}, {date}, {status}, {price}\n"));
        }
        Ok(str)
    }
}

pub struct SearchProductCommand {
    pub search_str: String,
}
impl Command<String> for SearchProductCommand {
    fn run(&self) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;

        let product_rows = db.query(
            "SELECT
                product.id,
                product.name,
                product.description,
                product.quantity,
                CAST(product.price AS DOUBLE PRECISION) as price,
                supplier.name
             FROM product
             INNER JOIN supplier ON supplier.id = product.supplier_id
             WHERE LOWER(product.name) LIKE LOWER('%'||$1||'%') OR LOWER(supplier.name) LIKE LOWER('%'||$1||'%')",
            &[&self.search_str],
        )?;
        let mut str: String = "id, name, description, quantity, price, supplier\n".to_string();
        for product in product_rows {
            let id: i32 = product.get(0);
            let name: String = product.get(1);
            let description: String = product.get(2);
            let quantity: i32 = product.get(3);
            let price: f64 = product.get(4);
            let supplier: String = product.get(5);

            str.push_str(&format!(
                "{id}, {name}, {description}, {quantity}, {price}, {supplier}\n"
            ));
        }
        Ok(str)
    }
}
