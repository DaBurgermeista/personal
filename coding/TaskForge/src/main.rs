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
    let unit: &str = &args[2];

    let (converted_temp, target_unit) = match unit.to_lowercase().as_str() {
        "f" | "fahrenheit" => {
            ((temp - 32.0) * (5.0 / 9.0), "C")
        }
        "c" | "celcius" => {
            ((temp * (9.0/5.0)) + 32.0, "F")
        }
        _ => {
            eprintln!("Not a valid Unit type");
            std::process::exit(1)
        }
    };
  
    println!("Temperature: {:.2}°{}", converted_temp, target_unit);

}
