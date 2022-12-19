use std::error::Error;

use crate::hope::{Customer, Admin};

pub trait CustomerCommand {
    fn run(&self, customer: &mut Customer) -> Result<(), Box<dyn Error>>;
}

pub trait AdminCommand {
    fn run(&self, admin: &mut Admin) -> Result<(), Box<dyn Error>>;
}

pub trait Command {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

