use postgres::{Client, NoTls};
use std::any::Any;
use std::{error::Error, io::Write};
use std::{io, io::ErrorKind};

use crate::command::Command;
use crate::hope::*;

pub fn connect_db() -> Result<Client, Box<dyn Error>> {
    let host = dotenv::var("PG_HOST")?;
    let user = dotenv::var("PG_USER")?;
    let pwd = dotenv::var("PG_PASSWORD")?;
    let db = dotenv::var("PG_DB")?;
    let schema = dotenv::var("PG_SCHEMA")?;

    std::io::stdout().flush()?;
    let mut client = Client::connect(
        &format!("host={host} user={user} password='{pwd}' dbname={db}"),
        NoTls,
    )?;
    client.execute(&format!("SET SCHEMA '{}'", schema), &[])?;
    Ok(client)
}

pub struct ClearCommand;
impl Command for ClearCommand {
    fn run(&mut self, _: &mut Hope) -> Result<Option<Box<dyn Any>>, Box<dyn Error>> {
        // https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html#deletion
        print!("\u{001b}[2J\u{001b}[H");
        io::stdout().flush()?;
        Ok(None)
    }
}

pub struct AddAddressCommand {
    pub street: String,
    pub city: String,
    pub country: String,
    pub telephone: String,
}
impl Command for AddAddressCommand {
    fn run(&mut self, store: &mut Hope) -> Result<Option<Box<dyn Any>>, Box<dyn Error>> {
        let mut db = connect_db()?;
        let ok_insert = db.execute(
            "INSERT INTO address(street, city, country, telephone)
                                 VALUES($1,$2,$3,$4)",
            &[&self.street, &self.city, &self.country, &self.telephone],
        )?;
        if ok_insert == 1 {
            let address_ids = db.query("SELECT id FROM address ORDER BY id DESC LIMIT 1", &[])?; // FIXME: Very dangerous not checking for empty
            let address_id: i32 = address_ids[0].get("id");
            Ok(Some(Box::new(address_id)))
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to create address",
            )))
        }
    }
}

pub struct SignupCommand {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub street: String,
    pub city: String,
    pub country: String,
    pub telephone: String,
}
impl Command for SignupCommand {
    fn run(&mut self, store: &mut Hope) -> Result<Option<Box<dyn Any>>, Box<dyn Error>> {
        let mut db = connect_db()?;
        let address = db.execute(
            "INSERT INTO address(street, city, country, telephone)
                                  VALUES($1,$2,$3,$4)",
            &[&self.street, &self.city, &self.country, &self.telephone],
        )?;

        let address_ids = db.query("SELECT id FROM address ORDER BY id DESC LIMIT 1", &[])?; // FIXME: Very dangerous not checking for empty
        let address_id: i32 = address_ids[0].get("id");
        let ok_insert = db.execute(
            "INSERT INTO customer(address_id, firstname, lastname, email, password)
             VALUES ($1,$2,$3,$4,$5)",
            &[
                &address_id,
                &self.first_name,
                &self.last_name,
                &self.email,
                &self.password,
            ],
        )?;
        if ok_insert == 1 {
            Ok(None)
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to create user",
            )))
        }
    }
}

pub struct LoginCommand {
    pub mode: HopeMode,
    pub email: String,
    pub password: String,
}
impl Command for LoginCommand {
    fn run(&mut self, store: &mut Hope) -> Result<Option<Box<dyn Any>>, Box<dyn Error>> {
        if store.is_login() {
            return Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Already logged in",
            )));
        }
        let mut db = connect_db()?;
        let mode = match self.mode {
            HopeMode::Admin => "admin",
            HopeMode::Customer => "customer",
        };
        let query = format!("SELECT id, email FROM {} WHERE email=$1", mode);
        let email = db.query(&query, &[&self.email])?;
        let password = db.query(
            &format!("SELECT id FROM {} WHERE email=$1 AND password=$2", mode),
            &[&self.email, &self.password],
        )?;

        if email.len() == 1 && password.len() == 1 {
            let id: i32 = email[0].get("id");
            let email: &str = email[0].get("email");
            store.login(Login::new(id, email.to_string(), self.mode));
            store.status = LockStatus::LogIn;
            Ok(None)
        } else if email.len() == 1 && password.len() == 0 {
            Err(Box::new(io::Error::new(
                ErrorKind::InvalidData,
                "Wrong password",
            )))
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Login failed: no user",
            )))
        }
    }
}

pub struct LogoutCommand;
impl Command for LogoutCommand {
    fn run(&mut self, store: &mut Hope) -> Result<Option<Box<dyn Any>>, Box<dyn Error>> {
        if !store.is_login() {
            return Err(Box::new(io::Error::new(ErrorKind::Other, "Not logged in")));
        }
        store.logout();
        Ok(None)
    }
}

pub struct RegisterCommand {
    pub mode: HopeMode,
    pub email: String,
    pub password: String,
}
impl Command for RegisterCommand {
    fn run(&mut self, store: &mut Hope) -> Result<Option<Box<dyn Any>>, Box<dyn Error>> {
        panic!("Register command not implemented");
    }
}

pub struct AddSupplierCommand {
    pub admin_id: i32,
    pub address_id: i32,
    pub name: String,
}
impl Command for AddSupplierCommand {
    fn run(&mut self, store: &mut Hope) -> Result<Option<Box<dyn Any>>, Box<dyn Error>> {
        let mut db = connect_db()?;
        let ok_insert = db.execute(
            "INSERT INTO supplier(admin_id, address_id, name) VALUES ($1, $2, $3)",
            &[&self.admin_id, &self.address_id, &self.name],
        )?;
        if ok_insert == 1 {
            Ok(None)
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
    pub quantity: String,
    pub price: String,
}
impl Command for AddProductCommand {
    fn run(&mut self, store: &mut Hope) -> Result<Option<Box<dyn Any>>, Box<dyn Error>> {
        let mut db = connect_db()?;

        let ok_insert = db.execute(
            "INSERT INTO product(supplier_id, name, quantity, price)
             VALUES ($1,$2,$3,$4)",
            &[&self.supplier_id, &self.name, &self.quantity, &self.price],
        )?;
        if ok_insert == 1 {
            Ok(None)
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to create product",
            )))
        }
    }
}
