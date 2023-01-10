use crate::command::Command;
use crate::command::CustomerCommand;
use crate::db::connect_db;
use crate::hope::*;

use chrono::NaiveDateTime;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::error::Error;
use std::{io, io::ErrorKind};

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
        let mut action = db.transaction()?;

        let added_address = action.query(
            "INSERT INTO address(
                street,
                postcode,
                city,
                country,
                telephone
            ) VALUES ($1, $2, $3, $4, $5)
            RETURNING *",
            &[
                &self.street,
                &self.postcode,
                &self.city,
                &self.country,
                &self.telephone,
            ],
        )?;

        if added_address.len() > 1 {
            return Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to create address for supplier",
            )));
        }

        let address_id: i32 = added_address[0].get("id");

        let add_customer = action.query(
            "INSERT INTO customer(address_id, first_name, last_name, email, password)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *",
            &[
                &address_id,
                &self.first_name,
                &self.last_name,
                &self.email,
                &self.password,
            ],
        )?;

        if add_customer.len() == 1 {
            action.commit()?;
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

pub struct AddToCart {
    pub product_id: i32,
    pub quantity: i32,
}
impl CustomerCommand<()> for AddToCart {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;

        let cart_row = db.query_one(
            "SELECT cart.id FROM cart WHERE customer_id = $1",
            &[&customer.id()],
        );

        let cart_id: i32;
        if let Ok(cart) = cart_row {
            cart_id = cart.get("id");
        } else {
            let cart = db.query_one(
                "INSERT INTO cart(customer_id, updated) VALUES ($1, CURRENT_TIMESTAMP)
                 RETURNING id",
                &[&customer.id()],
            )?;
            cart_id = cart.get("id");
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
        );

        // FIXME: Maybe return a better looking formatting
        let mut str: String = "id, name, price, quantity, sum\n".to_string();

        if let Err(_) = cart_row {
            return Ok(str);
        }

        let cart_id: i32;
        if let Ok(cart) = cart_row {
            cart_id = cart.get("id");
        } else {
            return Err(Box::new(io::Error::new(
                ErrorKind::Other,
                format!("Failed view cart: {}", cart_row.unwrap_err()),
            )));
        }

        let cart = db.query("SELECT * FROM cart_item WHERE cart_id=$1", &[&cart_id])?;

        for row in cart {
            let product_id: i32 = row.get("product_id");
            let quantity: i32 = row.get("quantity");
            let product_row = db.query_one(
                "SELECT name, price FROM product WHERE id=$1",
                &[&product_id],
            )?;

            let discount_row = db.query_one(
                "SELECT factor FROM discount_item
                 INNER JOIN discount ON discount_id = id
                 WHERE product_id=$1 AND CURRENT_TIMESTAMP BETWEEN start_date AND end_date",
                &[&product_id],
            );
            let mut discount_factor: Decimal = Decimal::from_f64(1.0).unwrap();
            if let Ok(discount) = discount_row {
                discount_factor = discount.get("factor");
            } else {
                //let err = discount_row.unwrap_err();
                //eprintln!("{}", err);
            }

            let name: String = product_row.get("name");
            let price: Decimal = product_row.get("price");
            let unit_price: Decimal = price * discount_factor;
            let sum = unit_price * Decimal::from_i32(quantity).unwrap();

            str.push_str(&format!(
                "{product_id}, {name}, {price}, {quantity}, {sum}\n"
            ));
        }
        Ok(str)
    }
}

pub struct CheckoutCommand {
    pub date_time: NaiveDateTime,
}
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
             VALUES ($1, $2, $3)
             RETURNING id",
            &[&customer.id(), &self.date_time, &"pending"],
        );

        if let Ok(order) = order_row {
            let order_id: i32 = order.get("id");
            let items = db.query("SELECT * FROM cart_item WHERE cart_id = $1", &[&cart_id])?;

            let mut transaction = db.transaction()?;
            let insert_item = transaction.prepare(
                "INSERT INTO order_item(order_id, product_id, quantity, unit_price) VALUES ($1, $2, $3, $4)",
            )?;

            let product_q = transaction.prepare("SELECT * FROM product WHERE id = $1")?;
            let dec_quantity =
                transaction.prepare("UPDATE product SET quantity = $1 WHERE id = $2")?;

            for item in items {
                let product_id: i32 = item.get("product_id");
                let quantity: i32 = item.get("quantity");
                let product_row = transaction.query_one(&product_q, &[&product_id])?;
                let price: Decimal = product_row.get("price");
                let product_quantity: i32 = product_row.get("quantity");

                let discount_row = transaction.query_one(
                    "SELECT factor FROM discount_item
                    INNER JOIN discount ON discount_id = id
                    WHERE product_id=$1 AND CURRENT_TIMESTAMP BETWEEN start_date AND end_date",
                    &[&product_id],
                );
                let mut discount_factor: Decimal = Decimal::from_f64(1.0).unwrap();
                if let Ok(discount) = discount_row {
                    discount_factor = discount.get("factor");
                }

                let unit_price: Decimal = price * discount_factor;

                let new_quantity = product_quantity - quantity;
                if new_quantity < 0 {
                    transaction.rollback()?;
                    return Err(Box::new(io::Error::new(
                        ErrorKind::Other,
                        "Failed to checkout cart",
                    )));
                }
                transaction.execute(&dec_quantity, &[&new_quantity, &product_id])?;
                transaction.execute(
                    &insert_item,
                    &[&order_id, &product_id, &quantity, &unit_price],
                )?;
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
            "SELECT orders.id, status, SUM(order_item.unit_price * order_item.quantity) AS price, orders.created FROM orders
             INNER JOIN order_item ON orders.id=order_item.order_id
             INNER JOIN product ON order_item.product_id = product.id
             WHERE customer_id=$1
             GROUP BY orders.id",
            &[&customer.id()],
        )?;

        let mut str: String = "id, date, name, status, price with applied discount\n".to_string();
        for order in order_rows {
            let id: i32 = order.get("id");
            let status: String = order.get("status");
            let price: Decimal = order.get("price");
            let date: NaiveDateTime = order.get("created");

            str.push_str(&format!("{id}, {date}, {status}, {price}\n"));
        }
        Ok(str)
    }
}

pub struct ShowOrderProductsCommand {
    pub order_id: i32
}
impl CustomerCommand<String> for ShowOrderProductsCommand {
    fn run(&self, customer: &mut Customer) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;

        let order_items = db.query(
            "SELECT
                product.id,
                product.name,
                order_item.quantity,
                order_item.unit_price,
                (order_item.quantity * order_item.unit_price) as total
            FROM order_item
            INNER JOIN product ON product.id = order_item.product_id
            WHERE order_item.order_id = $1",
            &[&self.order_id]
        )?;

        let mut str: String = "id, name, quantity, unit price, total".to_string();
        let mut total: Decimal = Decimal::from_f64(0.0).unwrap();
        for order in order_items {
            let id: i32 = order.get(0);
            let name: String = order.get(1);
            let quantity: i32 = order.get(2);
            let unit_price: Decimal = order.get(3);
            let sub_total: Decimal = order.get(4);
            total += sub_total;

            str.push_str(&format!("{id}, {name}, {quantity}, {unit_price}, {sub_total}\n"));
        }
        str.push_str(&format!("order total: {total}\n"));
        Ok(str)
    }
}

pub struct DiscountedProductsCommand {}
impl Command<String> for DiscountedProductsCommand {
    fn run(&self) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;
        let discount_item_rows = db.query(
            "SELECT product_id, discount_id, factor FROM discount_item
             INNER JOIN discount ON discount_id = id
             WHERE CURRENT_TIMESTAMP BETWEEN start_date AND end_date",
            &[],
        )?;

        let mut str: String =
            "product id, name , discount id, discount name, discount percentage\n".to_string();
        for row in discount_item_rows {
            let product_id: i32 = row.get("product_id");
            let discount_id: i32 = row.get("discount_id");

            let product_row = db.query_one(
                "SELECT product.id, product.name, price FROM product WHERE id = $1",
                &[&product_id],
            )?;

            let product_id: i32 = product_row.get("id");
            let product_name: String = product_row.get("name");
            let product_price: Decimal = product_row.get("price");
            let discount_row =
                db.query_one("SELECT * FROM discount WHERE id = $1", &[&discount_id])?;

            let discount_id: i32 = discount_row.get("id");
            let discount_name: String = discount_row.get("name");
            let discount_rate: Decimal = row.get("factor");

            str.push_str(&format!(
                "{product_id}, {product_name}, {product_price}, {discount_id}, {discount_name}, {discount_rate}\n"
            ));
        }
        Ok(str)
    }
}
