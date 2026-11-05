// src/player.rs

pub struct Player {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub level: u32,
    pub experience: i32,
}

// Implement methods for the Player struct
impl Player {
    // A constructor function to create a new player
    pub fn new(name: String) -> Self {
        Player {
            name,
            health: 35,
            max_health: 35,
            level: 1,
            experience: 0,
        }
    }

    // A method to show player status
    pub fn print_status(&self) {
        println!(
            "\n--- Player Status ---\nName:   {}\nLVL:    {}(XP: {})\nHP: {}/{}\n",
            self.name, self.level, self.experience, self.health, self.max_health
        );
    }

    // Taking damage with a death check. Returns 'true' if alive
    pub fn take_damage(&mut self, amount: i32) -> bool {
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
    pub fn add_experience(&mut self, amount: i32) {
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

