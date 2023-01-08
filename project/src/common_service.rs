use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use std::io;
use std::{error::Error, io::Write};

use crate::command::Command;
use crate::db::connect_db;

pub struct ClearCommand;
impl Command<()> for ClearCommand {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        // https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html#deletion
        print!("\u{001b}[2J\u{001b}[H");
        io::stdout().flush()?;
        Ok(())
    }
}

pub struct ListProductsCommand;
impl Command<String> for ListProductsCommand {
    fn run(&self) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;

        let mut str = String::new();
        str.push_str(&"id, supplier_id, name, description, quantity, price\n");
        for row in db.query(
            "SELECT id, supplier_id, name, description, quantity, price FROM product",
            &[],
        )? {
            let id: i32 = row.get(0);
            let supplier_id: i32 = row.get(1);
            let name: String = row.get(2);
            let description: String = row.get(3);
            let quantity: i32 = row.get(4);
            let price: Decimal = row.get(5);

            str.push_str(&format!(
                "{}, {}, {}, {}, {}, {}\n",
                id, supplier_id, name, description, quantity, price
            ));
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
                product.price,
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
            let price: Decimal = product.get(4);
            let supplier: String = product.get(5);

            str.push_str(&format!(
                "{id}, {name}, {description}, {quantity}, {price}, {supplier}\n"
            ));
        }
        Ok(str)
    }
}

pub struct ShowDiscountedProductsCommand {}
impl Command<String> for ShowDiscountedProductsCommand {
    //FIXME: this is a mess
    fn run(&self) -> Result<String, Box<dyn Error>> {
        let mut db = connect_db()?;
        let discount_item_rows = db.query(
            "SELECT product_id, discount_id, factor FROM discount_item",
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
