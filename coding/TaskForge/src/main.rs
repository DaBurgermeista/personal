// src/main.rs

use std::env;

fn main() {
    
    // Collect args
    let args: Vec<String> = env::args().collect();

    // Check for enough arguments
    if args.len() < 3 {
        eprintln!("Usage: {} <name> <count>", args[0]);
        std::process::exit(1);
    }

    let name = &args[1];
    // Convert string to integer
    let count: u32 = &args[2].parse().expect("Count must be a number");

    for _ in 0..count {
        println!("Hello, {}", name)
    }

}
