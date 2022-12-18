use std::error::Error;
use crate::hope::Hope;

pub trait Command {
    fn run(&mut self, store: &mut Hope) -> Result<(), Box<dyn Error>>;
}

