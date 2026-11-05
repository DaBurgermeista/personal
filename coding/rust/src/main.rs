use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

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

    // Taking damage with a death check. Returns 'true' if alive
    fn take_damage(&mut self, amount: i32) -> bool {
        self.health -= amount;
        println!("{} took {} damage!", self.name, amount);

        if self.health <= 0 {
            self.health = 0;
            println!("Oh no! {} has fallen in battle...", self.name);
            println!("--- GAME OVER ---");
            false
        } else {
            true
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
    
    player.print_status();
    loop {
        // Action Loop
        println!("What would you like to do?");
        println!("[1] Train (Gain EXP)");
        println!("[2] Venture into the Cave (Take Damage)");
        println!("[3] See Player Status");
        println!("[4] Quit");
        println!("Choose an option (1-4): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

    
        match choice.trim() {
            "1" => {
                if player.level >= 5 {
                    println!("\nYou can't learn anymore at this time!");
                } else {
                    println!("\nYou practice your sword swings!");
                    player.add_experience(5);
                }
            }
            "2" => {
                let seed = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u32;

                let roll = seed % 100;

                if roll < 40 {
                    //40% Chance
                    println!("You found a safe resting spot in the cave.");
                    player.health = (player.health + 10).min(player.max_health);
                } else if roll < 80 {
                    //40% Chance
                    let damage = (seed % 6) + 5;
                    let exp_gained = 3;

                    println!("\nA wild Goblin attacks!");

                    if player.take_damage(damage as i32) {
                        println!("You strike back and defeat the goblin!");
                        player.add_experience(exp_gained);
                    } else {
                        break;
                    }
                } else {
                    //20% Chance
                    let damage = (seed % 11) + 15;
                    let exp_gained = 8;

                    println!("\nOh no! A Cave Dragon emerges!");

                    if player.take_damage(damage as i32) {
                        println!("Against all odds, you defeat the Cave Dragon!");
                        player.add_experience(exp_gained);
                    } else {
                        break;
                    }
                }

            }
            "3" => {
                player.print_status();
            }
            "4" | "q" | "quit" => {
                println!("\nFarewell, {}! Thanks for playing", player.name);
                break;
            }
            _   => {
                println!("\nInvalid choice! You stand there, looking confused.");
            }
        }
    }

    println!("Game closed successfully.");
    
}
