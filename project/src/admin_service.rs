use crate::command::AdminCommand;
use crate::command::Command;
use crate::db::connect_db;
use crate::hope::*;
use chrono::NaiveDateTime;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::error::Error;
use std::{io, io::ErrorKind};

pub struct RegisterAdminCommand {
    pub email: String,
    pub password: String,
}
impl Command<Admin> for RegisterAdminCommand {
    fn run(&self) -> Result<Admin, Box<dyn Error>> {
        let mut db = connect_db()?;
        let add_admin = db.query(
            "INSERT INTO admin(email, password) VALUES ($1, $2)
             RETURNING id",
            &[&self.email, &self.password],
        )?;
        if add_admin.len() == 1 {
            let id: i32 = add_admin[0].get("id");
            Ok(Admin::new(id, self.email.to_owned()))
        } else {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Failed to create admin account check database",
            )));
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

pub struct AddSupplierCommand {
    pub name: String,
    pub description: String,
    pub org_num: String,
    pub street: String,
    pub postcode: String,
    pub city: String,
    pub country: String,
    pub telephone: String,
}
impl AdminCommand<Supplier> for AddSupplierCommand {
    fn run(&self, admin: &mut Admin) -> Result<Supplier, Box<dyn Error>> {
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

        let add_supplier = action.query(
            "INSERT INTO supplier(
                admin_id,
                address_id,
                name,
                description,
                orgnum
            ) VALUES ($1, $2, $3, $4, $5)
            RETURNING *",
            &[
                &admin.id(),
                &address_id,
                &self.name,
                &self.description,
                &self.org_num,
            ],
        )?;

        if add_supplier.len() == 1 {
            action.commit()?;
            let supplier = &add_supplier[0];
            Ok(Supplier {
                id: supplier.get("id"),
            })
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
impl AdminCommand<()> for AddProductCommand {
    fn run(&self, _: &mut Admin) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let price = Decimal::from_f64(self.price);

        let ok_insert = db.execute(
            "INSERT INTO product(supplier_id, name, description, quantity, price)
             VALUES ($1, $2, $3, $4, $5)",
            &[
                &self.supplier_id,
                &self.name,
                &self.description,
                &self.quantity,
                &price,
            ],
        )?;

        if ok_insert == 1 {
            Ok(())
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to create product",
            )))
        }
    }
}

pub struct EditProductQuantityCommand {
    pub product_id: i32,
    pub quantity: i32,
}
impl AdminCommand<()> for EditProductQuantityCommand {
    fn run(&self, _: &mut Admin) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        db.execute(
            "UPDATE product SET quantity = $1 WHERE id = $2",
            &[&self.quantity, &self.product_id],
        )?;
        Ok(())
    }
}

pub struct DeleteProductCommand {
    pub product_id: i32,
}
impl AdminCommand<()> for DeleteProductCommand {
    fn run(&self, _: &mut Admin) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;

        db.execute(
            "DELETE FROM product CASCADE WHERE id = $1",
            &[&self.product_id],
        )?;
        Ok(())
    }
}

pub struct AddNewDiscountCommand {
    pub code: String,
    pub name: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
impl AdminCommand<()> for AddNewDiscountCommand {
    fn run(&self, _: &mut Admin) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        db.execute(
            "INSERT INTO discount(code, name, start_date, end_date) 
            VALUES ($1, $2, $3, $4)",
            &[&self.code, &self.name, &self.start, &self.end],
        )?;
        Ok(())
    }
}

pub struct AssignDiscountCommand {
    pub discount_id: i32,
    pub product_id: i32,
    pub factor: f64,
}
impl AdminCommand<()> for AssignDiscountCommand {
    fn run(&self, _: &mut Admin) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let factor = Decimal::from_f64(self.factor).unwrap();
        db.execute(
            "INSERT INTO discount_item(discount_id, product_id, factor) 
            VALUES ($1, $2, $3)",
            &[&self.discount_id, &self.product_id, &factor],
        )?;
        Ok(())
    }
}

pub struct ViewDiscountHistoryCommand {}
impl AdminCommand<String> for ViewDiscountHistoryCommand {
    fn run(&self, admin: &mut Admin) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;
        let mut str = "id, code, name, start, end\n".to_string();

        let discount_rows = db.query("SELECT * FROM discount", &[])?;
        for row in discount_rows {
            let discount_id: i32 = row.get("id");
            let name: String = row.get("name");
            let code: String = row.get("code");
            let start: NaiveDateTime = row.get("start_date");
            let end: NaiveDateTime = row.get("end_date");
            str.push_str(&format!("{discount_id}, {name}, {code}, {start}, {end}\n"));
        }

        Ok(str)
    }
}

pub struct ViewUnconfirmedOrdersCommand {}
impl AdminCommand<String> for ViewUnconfirmedOrdersCommand {
    fn run(&self, _: &mut Admin) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;
        let order_rows = db.query("SELECT * FROM orders WHERE confirmed_by_admin IS NULL", &[])?;

        let mut str: String = "id, customer, created, status\n".to_string();
        for order in order_rows {
            let id: i32 = order.get("id");
            let customer: i32 = order.get("customer_id");
            let created: NaiveDateTime = order.get("created");
            let status: String = order.get("status");

            str.push_str(&format!("{id}, {customer}, {created}, {status}\n"));
        }
        Ok(str)
    }
}

pub struct ViewConfirmedOrdersCommand {}
impl AdminCommand<String> for ViewConfirmedOrdersCommand {
    fn run(&self, _: &mut Admin) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;
        let order_rows = db.query("SELECT * FROM orders WHERE confirmed_by_admin IS NOT NULL", &[])?;

        let mut str: String = "id, customer, created, status\n".to_string();
        for order in order_rows {
            let id: i32 = order.get("id");
            let customer: i32 = order.get("customer_id");
            let created: NaiveDateTime = order.get("created");
            let status: String = order.get("status");

            str.push_str(&format!("{id}, {customer}, {created}, {status}\n"));
        }
        Ok(str)
    }
}

pub struct ConfirmOrderCommand {
    pub order_id: i32,
}
impl AdminCommand<()> for ConfirmOrderCommand {
    fn run(&self, admin: &mut Admin) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;

        db.execute(
            "UPDATE orders SET confirmed_by_admin=$1, status='confirmed'
             WHERE id=$2",
            &[&admin.id(), &self.order_id],
        )?;
        Ok(())
    }
}

pub struct ListProductsMaxOrderCommand {}
impl AdminCommand<String> for ListProductsMaxOrderCommand {
    fn run(&self, _: &mut Admin) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;
        let mut str: String = String::new();
        let order_rows = db.query("SELECT p.name, p.description, p.price, o.id as order_id, o.customer_id, o.confirmed_by_admin, o.created, o.status, sum(oi.quantity)::INT as total_quantity
            FROM order_item oi
            JOIN product p ON oi.product_id = p.id
            JOIN orders o ON oi.order_id = o.id
            WHERE o.created >= current_date - INTERVAL '1 month' AND o.confirmed_by_admin IS NOT NULL
            GROUP BY p.id, p.name, p.description, p.price, o.id, o.customer_id, o.confirmed_by_admin, o.created, o.status
            ORDER BY total_quantity DESC;
            ", &[])?;
        str.push_str(&format!(
            "Product id, Product name, Order id, total quantity \n"
        ));
        for row in order_rows {
            let product_name: String = row.get(0);
            let product_id: i32 = row.get(3);
            let order_id: i32 = row.get(4);
            let total_quantity: i32 = row.get("total_quantity");
            str.push_str(&format!(
                "{product_id}, {product_name}, {order_id}, {total_quantity}\n"
            ));
        }
        Ok(str)
    }
}

pub struct DiscountHistoryCommand {}
impl AdminCommand<String> for DiscountHistoryCommand {
    fn run(&self, _: &mut Admin) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;
        let discount_item_rows = db.query(
            "SELECT product_id, discount_id, factor FROM discount_item
             INNER JOIN discount ON discount_id = id",
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
