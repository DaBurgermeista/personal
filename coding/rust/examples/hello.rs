// Placeholder Rust file
//

use std::io::{self, Write};

struct Player {
    name: String,
    health: i32,
}

fn main(){
    let mut player = Player{name: "Borg".to_string(), health: 35};

    print!("What is your name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let trimmed_input = user_input.trim();
    player.name = trimmed_input.to_string();
    println!("Hello {}! Your health is currently: {}", &player.name, &player.health);
}
