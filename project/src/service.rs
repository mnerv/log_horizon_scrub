use postgres::{Client, NoTls};
use std::{error::Error, io::Write};
use std::{io, io::ErrorKind};

use crate::command::Command;
use crate::hope::*;

pub fn connect_db() -> Result<Client, Box<dyn Error>> {
    let host   = dotenv::var("PG_HOST")?;
    let user   = dotenv::var("PG_USER")?;
    let pwd    = dotenv::var("PG_PASSWORD")?;
    let db     = dotenv::var("PG_DB")?;
    let schema = dotenv::var("PG_SCHEMA")?;

    std::io::stdout().flush()?;
    let mut client = Client::connect(&format!("host={host} user={user} password='{pwd}' dbname={db}"), NoTls)?;
    client.execute(&format!("SET SCHEMA '{}'", schema), &[])?;
    Ok(client)
}

pub struct ClearCommand;
impl Command for ClearCommand {
    fn run(&mut self, _: &mut Hope) -> Result<(), Box<dyn Error>> {
        // https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html#deletion
        print!("\u{001b}[2J\u{001b}[H");
        io::stdout().flush()?;
        Ok(())
    }
}

pub struct LoginCommand {
    pub mode: HopeMode,
    pub email: String,
    pub password: String
}
impl Command for LoginCommand {
    fn run(&mut self, store: &mut Hope) -> Result<(), Box<dyn Error>> {
        if store.is_login() {
            return Err(Box::new(io::Error::new(ErrorKind::Other, "Already logged in")));
        }
        let mut db = connect_db()?;
        let mode = match self.mode {
            HopeMode::Admin => "admin",
            HopeMode::Customer => "customer"
        };
        let query = format!("SELECT id, email FROM {} WHERE email=$1", mode);
        let email = db.query(&query, &[&self.email])?;
        let password = db.query(&format!("SELECT id FROM {} WHERE email=$1 AND password=$2", mode),
                                      &[&self.email, &self.password])?;

        if email.len() == 1 && password.len() == 1 {
            let id: i32 = email[0].get("id");
            let email: &str = email[0].get("email");
            store.login(Login::new(id, email.to_string(), self.mode));
            Ok(())
        } else if email.len() == 1 && password.len() == 0 {
            Err(Box::new(io::Error::new(ErrorKind::InvalidData, "Wrong password")))
        } else {
            Err(Box::new(io::Error::new(ErrorKind::NotFound, "Login failed: no user")))
        }
    }
}

pub struct LogoutCommand;
impl Command for LogoutCommand {
    fn run(&mut self, store: &mut Hope) -> Result<(), Box<dyn Error>> {
        if !store.is_login() {
            return Err(Box::new(io::Error::new(ErrorKind::Other, "Not logged in")));
        }
        store.logout();
        Ok(())
    }
}

pub struct RegisterCommand{
    pub mode: HopeMode,
    pub email: String,
    pub password: String
}
impl Command for RegisterCommand {
    fn run(&mut self, store: &mut Hope) -> Result<(), Box<dyn Error>> {
        panic!("Register command not implemented");
    }
}

