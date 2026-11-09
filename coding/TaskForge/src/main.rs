// src/main.rs

use std::env;

fn main() {
    
    // Collect args
    let args: Vec<String> = env::args().collect();

    // Check for enough arguments
    if args.len() < 3 {
        eprintln!("Usage: {} <temp> <unit>", args[0]);
        std::process::exit(1);
    }

    let temp: i32 = args[1].parse().expect("Temp must be a number");
    // Convert string to integer
    let count = &args[2];

    let conversion = (temp - 32) * (5 / 9);
    println!("{} C", conversion);

}
