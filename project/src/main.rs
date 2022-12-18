/**
 * @file   main.rs
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Hope store - We sell hopes and dreams.
 *         Database interface front application.
 * @date   2022-11-23
 *
 * @copyright Copyright (c) 2022
 */
use std::{error::Error, io::Write};

mod command;
mod service;
mod hope;

use crate::command::Command;
use crate::service::*;
use crate::hope::Hope;
use crate::hope::HopeMode;

fn read_input(label: &str) -> Result<String ,Box<dyn Error>> {
    let mut input = String::new();
    print!("{}", label);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn run_command<C: Command>(store: &mut Hope, mut command: C) {
    match command.run(store) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err)
    }
}

fn main() {
    let pepper: &'static str = r#"
 /_/_  _  _    __/__  __
/ //_//_//_' _\ / /_///_'
     /                   "#.trim_start_matches('\n');
    let slanted: &'static str = r#"
    __  __                         __                
   / / / /___  ____  ___     _____/ /_____  ________ 
  / /_/ / __ \/ __ \/ _ \   / ___/ __/ __ \/ ___/ _ \
 / __  / /_/ / /_/ /  __/  (__  ) /_/ /_/ / /  /  __/
/_/ /_/\____/ .___/\___/  /____/\__/\____/_/   \___/ 
           /_/                     Hopes and dreams"#.trim_start_matches('\n');
    let speed: &'static str = r#"
______  __                              _____                   
___  / / /__________________     _________  /__________________ 
__  /_/ /_  __ \__  __ \  _ \    __  ___/  __/  __ \_  ___/  _ \
_  __  / / /_/ /_  /_/ /  __/    _(__  )/ /_ / /_/ /  /   /  __/
/_/ /_/  \____/_  .___/\___/     /____/ \__/ \____//_/    \___/ 
               /_/                 Hopes and dreams"#.trim_start_matches('\n');

    let mut store = Hope::new();
    let login = LoginCommand{
        mode: HopeMode::Admin,
        email: "eric@hopestore.se".to_string(),
        password: "hello".to_string()
    };
    let logout = LogoutCommand{};
    run_command(&mut store, login);
    println!("{}", store.user.to_string());
    run_command(&mut store, logout);
    println!("{}", store.is_login());
}
