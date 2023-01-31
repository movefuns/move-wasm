use clap::Parser;

use aptos_wasm::Tool;

use std::process::exit;

fn main() {
    // Run the corresponding tools
    // At this point, we'll want to print and determine whether to exit for an error code
    match Tool::parse().execute() {
        Ok(inner) => println!("{}", inner),
        Err(inner) => {
            println!("{}", inner);
            exit(1);
        }
    }
}