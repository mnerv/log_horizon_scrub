/**
 * @file   command.rs
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @brief  Comand traits
 * @date   2022-12-20
 *
 * @copyright Copyright (c) 2022
 */
use std::error::Error;

use crate::hope::{Admin, Customer};

pub trait CustomerCommand<T> {
    fn run(&self, customer: &mut Customer) -> Result<T, Box<dyn Error>>;
}

pub trait AdminCommand<T> {
    fn run(&self, admin: &mut Admin) -> Result<T, Box<dyn Error>>;
}

pub trait Command<T> {
    fn run(&self) -> Result<T, Box<dyn Error>>;
}
