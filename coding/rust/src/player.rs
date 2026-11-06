// src/player.rs

#[derive(Clone, Copy)]
pub enum Weapon {
    Fists,
    RustyDagger,
    IronSword,
    Greatsword,
}

impl Weapon {
    // Get the display name of the weapon
    pub fn name(&self) -> &str {
        match self {
            Weapon::Fists => "Fists",
            Weapon::RustyDagger => "Rusty Dagger",
            Weapon::IronSword => "Iron Sword",
            Weapon::Greatsword => "Steel Greatsword",
        }
    }

    // Base damage values for weapons
    pub fn base_damage(&self) -> i32 {
        match self {
            Weapon::Fists => 4,
            Weapon::RustyDagger => 8,
            Weapon::IronSword => 15,
            Weapon::Greatsword => 28,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Consumable {
    HealthPotion,
    ElixerOfStrength,
}

impl Consumable {
    pub fn name(&self) -> &str {
        match self {
            Consumable::HealthPotion => "Minor Health Potion",
            Consumable::ElixerOfStrength => "Elixer of Strength",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Item {
    WeaponItem(Weapon),
    ConsumableItem(Consumable),
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::WeaponItem(w) => w.name(),
            Item::ConsumableItem(c) => c.name(),
        }
    }
}

pub struct Player {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub level: u32,
    pub experience: i32,
    pub strength: i32,
    pub weapon: Weapon,
    pub inventory: Vec<Weapon>,
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
            strength: 10,
            weapon: Weapon::RustyDagger,
            inventory: vec![
                Item::WeaponItem(Weapon::IronSword),
                Item::ConsumableItem(Consumable::HealthPotion),
                Item::ConsumableItem(Consumable::ElixerOfStrength),
            ],
        }
    }

    // A method to show player status
    pub fn print_status(&self) {
        println!(
            "\n--- Player Status ---\nName:   {}\nLVL:    {}(XP: {})\nHP: \x1b[31m{}/{}\x1b[0m\nWeapon: {}\n",
            self.name, self.level, self.experience, self.health, self.max_health,self.weapon.name()
        );
    }

    pub fn show_inventory(&mut self) {
        if self.inventory.is_empty() {
            println!("\nYour inventory bag is empty.");
            return;
        }

        println!("\n--- Inventory Bag ---");
        for (index, item) in self.inventory.iter().enumerate() {
            println!("[{}] {}", index + 1, item.name());
        }
        println!("[0] Back to menu");
        let choice = crate::get_user_input("Choose an item to use/equip: ");

        if let Ok(num) = choice.trim().parse::<usize>() {
            if num == 0 {
                return;
            }
            if num <= self.inventory.len() {
                let selected_index = num - 1;
                let chosen_item = self.inventory[selected_index];

                match chosen_item {
                    Item::WeaponItem(new_weapon) => {
                        let old_equipped = self.weapon;
                        self.weapon = new_weapon;
                        self.inventory[selected_index] = Item::WeaponItem(old_equipped);
                        println!("\nYou unequipped {} and equipped the \x1b[35m{}\x1b[0m", old_equipped.name(), self.weapon.name());
                    }
                    Item::ConsumableItem(potion) => {
                        match potion {
                            Consumable::HealthPotion => {
                                let heal_amount = 20;
                                self.health = (self.health + heal_amount).min(self.max_health);
                                println!("\nYou drink the {} and recover \x1b[32m{} HP\x1b[0m ({}/{})", 
                                    potion.name(), heal_amount, self.health, self.max_health
                                );
                            }
                            Consumable::ElixerOfStrength => {
                                self.strength += 2;
                                println!("\nYou drink the {}! Your Strength has permenantly increased to {}!",
                                    potion.name(), self.strength
                                );
                            }
                        }
                        self.inventory.remove(selected_index);
                    }
                }

                
            }
        }
    }

    // Calculate scaling physical attack power
    pub fn get_attack_power(&self) -> i32 {
        let weapon_dmg = self.weapon.base_damage();
        // Strength acts as a percentage multiplier
        let multiplier = 1.0 + (self.strength as f32 / 100.0);

        (weapon_dmg as f32 * multiplier) as i32
    }

    // Taking damage with a death check. Returns 'true' if alive
    pub fn take_damage(&mut self, amount: i32) -> bool {
        self.health -= amount;
        if self.health <= 0 {
            self.health = 0;
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
            self.strength += 3;

            // Increase Max HP as a level bonus
            self.max_health += 10;

            // Fully heal the player on level up
            self.health = self.max_health;

            println!("\nLEVEL UP! You are now Level {}", self.level);
            println!("Max HP increased! Current HP fully restored to {}.", self.max_health);
        }
    }
}

