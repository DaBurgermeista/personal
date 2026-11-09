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
    let mut unit: &str = &args[2];

    let conversion = match unit.to_lowercase().as_str() {
        "f" => {
            unit = "C";
            (temp - 32.0) * (5.0 / 9.0)
        }
        "c" => {
            unit = "F";
            (temp * (9.0/5.0)) + 32.0
        }
        _ => {
            eprintln!("Not a valid Unit type");
            std::process::exit(1)
        }
    };

    
    println!("Temperature: {:.2}°{}", conversion, unit);

}
