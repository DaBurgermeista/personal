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
            "\n--- Player Status ---\nName:   {}\nLVL:    {}(XP: {})\nHP: {}/{}\n",
            self.name, self.level, self.experience, self.health, self.max_health
        );
    }

    // Taking damage with a death check
    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        println!("{} took {} damage!", self.name, amount);

        if self.health <= 0 {
            self.health = 0;
            println!("Oh no! {} has fallen in battle...", self.name);
            println!("--- GAME OVER ---");
        }
    }

    // Adding experience and handling level ups
    fn add_experience(&mut self, amount: i32) {
        self.experience += amount;
        println!("\n--- Gained {} EXP! ---", amount);

        // Define how much EXP is needed level up
        let xp_to_level = 10;

        if self.experience >= xp_to_level {
            // Deduct EXP used for the level up
            self.experience -= xp_to_level;

            // Level up
            self.level += 1;

            // Increase Max HP as a level bonus
            self.max_health += 10;

            // Fully heal the player on level up
            self.health = self.max_health;

            println!("\n LEVEL UP! You are now Level {}", self.level);
            println!("Max HP increased! Current HP fully restored to {}.", self.max_health);
        }
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
    
    println!("You stand at a dusty crossroads. What do you do?");
    println!("[1] Go North (towards the mountains)");
    println!("[2] Go South (towards the village)");
    println!("[3] Rest by the road");
    println!("Choose an option (1-3): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => println!("\nYou head North. The wind starts to howl."),
        "2" => println!("\nYou head South. You can smell cooking fires in the distance."),
        "3" => println!("\nYou sit down and catch your breath. You feel refreshed."),
        _   => println!("\nInvalid choice! You stand there, looking confused.")
    }
    
}
