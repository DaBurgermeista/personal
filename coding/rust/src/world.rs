// stc/world.rs

use anstream::println;
use inquire::Select;
use crate::player::Player;
use crate::{PALADIN_GOLD, SYSTEM_GRAY, HEAL_GREEN, GameState};

#[derive(Clone, Copy, PartialEq)]
pub enum Location {
    Tavern,
    TownSquare,
    Blacksmith,
    DarkWoods,
    DeepCave,
}

impl Location {
    pub fn name(&self) -> &str {
        match self {
            Location::Tavern => "The Rusty Anchor Tavern",
            Location::TownSquare => "Oakhaven Town Square",
            Location::Blacksmith => "Garrick's Forge",
            Location::DarkWoods => "The Whispering Woods",
            Location::DeepCave => "The Shadowed Caven",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Location::Tavern => "A warm fire crackles in the hearth. The smell of roasted meat and ale fills the air.",
            Location::TownSquare => "A bustling cobblestone square. Roads lead to the tavern, the forge, and the wild woods to the north.",
            Location::Blacksmith => "The rhythmic clanging of hammer on anvil echoes here. Weapons line the stone walls.",
            Location::DarkWoods => "Tall, twisted trees block out the sun. You hear rustling in the undergrowth.",
            Location::DeepCave => "Damp, pitch-black tunnels stretch out before you. Danger lurks in every shadow.",
        }
    }
}

pub fn explore_location(player: &mut Player, current_loc: &mut Location) -> GameState {
    println!("\n{PALADIN_GOLD}=== {} ==={PALADIN_GOLD:#}", current_loc.name());
    println!("{SYSTEM_GRAY}{}{SYSTEM_GRAY:#}\n", current_loc.description());

    match current_loc {
        Location::Tavern => {
            let options = vec![
                "Rest by the fire (Heal to Full)",
                "Go Outside -> Town Square",
                "Open Inventory",
                "Check Status",
                "Quit Game",
            ];

            match Select::new("What do you do?", options).prompt() {
                Ok("Rest by the fire (Heal to Full)") => {
                    player.health = player.max_health;
                    println!("{HEAL_GREEN}You rest by the warm hearth. Your health is fully restored.{HEAL_GREEN:#}");
                    GameState::Exploring
                }
                Ok("Go Outside -> Town Square") => {
                    *current_loc = Location::TownSquare;
                    GameState::Exploring
                }
                Ok("Open Inventory") => GameState::Inventory,
                Ok("Check Status") => {
                    player.print_status();
                    GameState::Exploring
                }
                Ok("Quit Game") => GameState::Exit,
                _ => GameState::Exploring
            }
        }

        Location::TownSquare => {
            let options = vec![
                "Enter The Rusty Anchor Tavern",
                "Visit Garrick's Forge",
                "Head North -> The Whispering Woods",
                "Open Inventory",
                "Check Status",
                "Quit Game",
            ];

            match Select::new("Where do you go?", options).prompt() {
                Ok("Enter The Rusty Anchor Tavern") => {
                    *current_loc = Location::Tavern;
                    GameState::Exploring
                }
                Ok("Visit Garrick's Forge") => {
                    *current_loc = Location::Blacksmith;
                    GameState::Exploring
                }
                Ok("Head North -> The Whispering Woods") => {
                    *current_loc = Location::DarkWoods;
                    GameState::Exploring
                }
                Ok("Open Inventory") => GameState::Inventory,
                Ok("Check Status") => {
                    player.print_status();
                    GameState::Exploring
                }
                Ok("Quit Game") => GameState::Exit,
                _ => GameState::Exploring,
            }
        }

        Location::Blacksmith => {
            let options = vec![
                "Talk to Garrick (Train STR)",
                "Go Outside -> Town Square",
                "Open Inventory",
            ];

            match Select::new("What do you do?", options).prompt() {
                Ok("Talk to Garrick (Train STR)") => {
                    if player.level >= 5 {
                        println!("\nGarrick says: 'You're already stronger than me! Go fight some monsters!'");
                    } else {
                        println!("\nYou help Garrick hammer iron for an hour!");
                        player.add_experience(5);
                    }
                    GameState::Exploring
                }
                Ok("Go Outside -> Town Square") => {
                    *current_loc = Location::TownSquare;
                    GameState::Exploring
                }
                Ok("Open Inventory") => GameState::Inventory,
                _ => GameState::Exploring,
            }
        }

        Location::DarkWoods => {
            let options = vec![
                "Hunt for Monsters (Combat)",
                "Venture Deeper -> The Shadowed Cavern",
                "Return South -> Town Square",
                "Open Inventory",
            ];

            match Select::new("What do you do?", options).prompt() {
                Ok("Hunt for Monsters (Combat)") => GameState::Combat,
                Ok("Venture Deeper -> The Shadowed Cavern") => {
                    *current_loc = Location::DeepCave;
                    GameState::Exploring
                }
                Ok("Return South -> Town Square") => {
                    *current_loc = Location::TownSquare;
                    GameState::Exploring
                }
                Ok("Open Inventory") => GameState::Inventory,
                _ => GameState::Exploring,
            }
        }

        Location::DeepCave => {
            let options = vec![
                "Search the Darkness (Combat)",
                "Retreat -> The Whispering Woods",
                "Open Inventory",
            ];

            match Select::new("What do you do?", options).prompt() {
                Ok("Search the Darkness (Combat)") => GameState::Combat,
                Ok("Retreat -> The Whispering Woods") => {
                    *current_loc = Location::DarkWoods;
                    GameState::Exploring
                }
                Ok("Open Inventory") => GameState::Inventory,
                _ => GameState::Exploring,
            }
        }
    }
}