// Placeholder Rust file

use std::io::{self, Write};

struct Player {
    name: String,
    health: i32,
    max_health: i32,
    level: u32,
    experience: i32,
}

// Implement methods for the Player struct
impl Player {
    // A constructor function to create a new player
    fn new(name: String) -> Self {
        Player {
            name,
            health: 35,
            max_health: 35,
            level: 1,
            experience: 0,
        }
    }

    // A method to show player status
    fn print_status(&self) {
        println!(
            "\n--- ⚔️ Player Status ⚔️ ---\nName:   {}\nLVL:    {}(XP: {})\nHP: {}/{}\n",
            self.name, self.level, self.experience, self.health, self.max_health
        );
    }

    // Taking damage
    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        println!("{} took {} damage!", self.name, amount);
    }

    // Adding experience
    fn add_experience(&mut self, amount: i32) {
        self.experience += amount;
        println!("\n--- ✨  Gained {} EXP! ✨  ---", amount);
    }
}

fn main(){

    print!("What is your name adventurer?: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let trimmed_input = user_input.trim().to_string();

    // Create the player using constructor
    let mut player = Player::new(trimmed_input);
    
    // Call the status method
    player.print_status();
    player.take_damage(5);
    player.add_experience(4);
    player.print_status();

}
