use postgres::{Client, NoTls};
use std::{error::Error, io::Write};
use std::{io, io::ErrorKind};

use crate::command::*;
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
    fn run(&self) -> Result<(), Box<dyn Error>> {
        // https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html#deletion
        print!("\u{001b}[2J\u{001b}[H");
        io::stdout().flush()?;
        Ok(())
    }
}

pub struct AddAddressCommand {
    pub street: String,
    pub city: String,
    pub country: String,
    pub telephone: String,
}
impl Command for AddAddressCommand {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let ok_insert = db.execute(
            "INSERT INTO address(street, city, country, telephone)
             VALUES ($1, $2, $3, $4)",
            &[&self.street, &self.city, &self.country, &self.telephone],
        )?;
        if ok_insert == 1 {
            Ok(())
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
    pub city: String,
    pub country: String,
    pub telephone: String,
}
impl CustomerCommand for RegiserCustomerCommand {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let address_command = AddAddressCommand{
            street: self.street.to_owned(),
            city: self.city.to_owned(),
            country: self.country.to_owned(),
            telephone: self.telephone.to_owned(),
        };
        address_command.run()?;
        let address = db.query_one("SELECT id FROM address
                                    WHERE street = $1 AND city = $2 AND country = $3 AND telephone = $4",
                                    &[&self.street, &self.city, &self.country, &self.telephone])?;
        let address_id: i32 = address.get("id");

        let ok_insert = db.execute(
            "INSERT INTO customer(address_id, first_name, last_name, email, password)
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
            let login = LoginCustomerCommand{
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

pub struct LoginCustomerCommand{
    pub email: String,
    pub password: String,
}
impl CustomerCommand for LoginCustomerCommand {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let email = db.query_one("SELECT email FROM customer WHERE email = $1", &[&self.email]);
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
            let last_name: String  = login.get("last_name");
            let email: String      = login.get("email");
            let address_id: i32    = login.get("address_id");
            let street: String     = login.get("street");
            let city: String       = login.get("city");
            let country: String    = login.get("country");
            let telephone: String  = login.get("telephone");
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
impl Command for AddSupplierCommand {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;
        let ok_insert = db.execute(
            "INSERT INTO supplier(admin_id, address_id, name) VALUES ($1, $2, $3)",
            &[&self.admin_id, &self.address_id, &self.name],
        )?;
        if ok_insert == 1 {
            panic!("Not implemented")
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
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut db = connect_db()?;

        let ok_insert = db.execute(
            "INSERT INTO product(supplier_id, name, quantity, price)
             VALUES ($1,$2,$3,$4)",
            &[&self.supplier_id, &self.name, &self.quantity, &self.price],
        )?;
        if ok_insert == 1 {
            panic!("Not implemented")
        } else {
            Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Failed to create product",
            )))
        }
    }
}
