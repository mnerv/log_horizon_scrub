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
mod admin_service;
mod command;
mod common_service;
mod customer_service;
mod db;
mod hope;
mod mock;
mod tui;

use std::env;
use std::fs;

use crate::tui::tui_main;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut is_mock: bool = false;
    let mut init_file: String = String::new();
    let mut is_help: bool = false;

    for mut i in 0..args.len() {
        let arg = &args[i];
        if arg == "--mock" {
            is_mock = true;
        }
        if arg == "--init" {
            i += 1;
            init_file = args[i].to_owned();
        }
        if arg == "--help" {
            is_help = true;
        }
    }

    if is_help {
        println!("hopestore [options]");
        println!("usage:");
        println!("--init [file] - Regenerate SQL table");
        println!("--mock        - Generate mock data");
        return;
    }

    if !init_file.is_empty() {
        let contents =
            fs::read_to_string(init_file).expect("Should have been able to read the file");
        db::init_db(contents);
    }

    if is_mock {
        let mock_res = mock::mock_data();
        if let Err(err) = mock_res {
            eprintln!("{}", err);
            return;
        }
    }

    tui_main();
    println!("Goodbye cruel world...");
}
