// src/player.rs

use inquire::Select;
use comfy_table::{Table, Cell, Color as TableColor, Attribute};
use crate::items::{Item, Weapon, Consumable};

pub struct Player {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub level: u32,
    pub experience: i32,
    pub gold: i32,
    pub strength: i32,
    pub weapon: Weapon,
    pub inventory: Vec<Item>,
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
            gold: 0,
            strength: 10,
            weapon: Weapon::Fists,
            inventory: vec![
                Item::WeaponItem(Weapon::IronSword),
                Item::ConsumableItem(Consumable::HealthPotion),
                Item::ConsumableItem(Consumable::ElixirOfStrength),
            ],
        }
    }

    // A method to show player status
    pub fn print_status(&self) {
        println!(
            "\n--- Player Status ---\nName:   {}\nLVL:    {}(XP: {})\nHP: \x1b[31m{}/{}\x1b[0m\nWeapon: {}\n",
            self.name, self.level, self.experience, self.health, self.max_health,self.weapon.name()
        );
        let mut table = Table::new();

        table.load_preset(comfy_table::presets::UTF8_ROUND_CORNERS);

        table.set_header(vec![
            Cell::new("Stat").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ]);

        table.add_row(vec![
            Cell::new("Name"),
            Cell::new(&self.name).fg(TableColor::Yellow).add_attribute(Attribute::Bold),
        ]);
        table.add_row(vec![
            Cell::new("Level"),
            Cell::new(&self.level.to_string()).fg(TableColor::Cyan),
        ]);
        table.add_row(vec![
            Cell::new("Experience"),
            Cell::new(&format!("{} XP", self.experience)),
        ]);
        table.add_row(vec![
            Cell::new("Health"),
            Cell::new(&format!("{}/{} HP", self.health, self.max_health)).fg(TableColor::Green),
        ]);
        table.add_row(vec![
            Cell::new("Strength (STR)"),
            Cell::new(&self.strength.to_string()).fg(TableColor::Red),
        ]);
        table.add_row(vec![
            Cell::new("Equipped Weapon"),
            Cell::new(self.weapon.name()).fg(TableColor::Magenta),
        ]);

    }

    pub fn show_inventory(&mut self) {
        if self.inventory.is_empty() {
            println!("\nYour inventory bag is empty.");
            return;
        }

        // 1. Build the menu options list dynamically from your actual items
        let mut options: Vec<String> = self
            .inventory
            .iter()
            .map(|item| item.name().to_string())
            .collect();

        // 2. Add an exit/back option at the bottom
        options.push("↩ Back to Menu".to_string());

        // 3. Prompt the user with inquire::Select
        let prompt_result = Select::new("--- Inventory Bag ---", options).raw_prompt();

        match prompt_result {
            Ok(choice) => {
                let selected_index = choice.index;

                // If they selected the last option ("Back to Menu"), simply exit
                if selected_index == self.inventory.len() {
                    return;
                }

                let chosen_item = self.inventory[selected_index];

                match chosen_item {
                    Item::WeaponItem(new_weapon) => {
                        let old_equipped = self.weapon;
                        self.weapon = new_weapon;

                        // Don't put Fists into your backpack!
                        if old_equipped == Weapon::Fists {
                            self.inventory.remove(selected_index);
                        } else {
                            self.inventory[selected_index] = Item::WeaponItem(old_equipped);
                        }

                        println!(
                            "\nYou unequipped {} and equipped the \x1b[35m{}\x1b[0m!",
                            old_equipped.name(),
                            self.weapon.name()
                        );
                    }

                    Item::ConsumableItem(potion) => {
                        match potion {
                            Consumable::HealthPotion => {
                                let heal_amount = 20;
                                self.health = (self.health + heal_amount).min(self.max_health);
                                println!(
                                    "\nYou drink the {} and recover \x1b[32m{} HP\x1b[0m! ({}/{})",
                                    potion.name(),
                                    heal_amount,
                                    self.health,
                                    self.max_health
                                );
                            }
                            Consumable::ElixirOfStrength => {
                                self.strength += 2;
                                println!(
                                    "\nYou drink the {}! Your Strength has permanently increased to {}!",
                                    potion.name(),
                                    self.strength
                                );
                            }
                        }
                        // Remove consumed item from the bag
                        self.inventory.remove(selected_index);
                    }
                }
            }
            Err(_) => {
                // If they press Escape or Ctrl+C, safely return to game
                println!("\nClosed inventory.");
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

    pub fn add_gold(&mut self, amount: i32) {
        self.gold += amount;
    }
}

