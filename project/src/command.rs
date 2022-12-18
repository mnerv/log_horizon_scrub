use crate::hope::Hope;
use std::any::Any;
use std::error::Error;

pub trait Command {
    fn run(&mut self, store: &mut Hope) -> Result<Option<Box<dyn Any>>, Box<dyn Error>>;
}
