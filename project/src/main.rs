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
mod command;
mod hope;
mod service;
mod tui;

use crate::tui::tui_main;

fn main() {
    tui_main();
    println!("Goodbye cruel world...");
}
