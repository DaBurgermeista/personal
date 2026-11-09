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

    let temp: f64 = args[1].parse::<f64>().expect("Temp must be a number");
    // Convert string to integer
    let mut unit = &args[2];
    let mut conversion = 0.0;

    match unit.to_lowercase() {
        "f" => {
            let conversion = (temp - 32.0) * (5.0 / 9.0);
            unit = "C";
        }
        "c" => {
            let conversion = (temp * (9.0/5.0)) + 32.0;
            unit = "F";
        }
        _ => println!("Not a valid Unit type"),
    }

    
    println!("{} {}", conversion, unit);

}
