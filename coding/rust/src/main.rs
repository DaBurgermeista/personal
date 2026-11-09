// src/main.rs
use std::io::{self, Write};
use anstream::println;
use anstyle::{AnsiColor, Color, Style};
use inquire::{Select, Text, Confirm};
// use rand::Rng;

mod world;
mod combat;
mod enemy;
mod items;
mod player;

use player::Player;
use world::Location;

pub const PALADIN_GOLD: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow)));
pub const DAMAGE_RED: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Red)));
pub const HEAL_GREEN: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));
pub const SYSTEM_GRAY: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightBlack)));

enum GameState {
    Exploring,
    Combat,
    Inventory,
    Exit,
}

fn main(){
    println!("{SYSTEM_GRAY}--- Starting Terminal RPG Engine ---{SYSTEM_GRAY:#}\n");

    // Create the player using constructor
    let player_name = Text::new("What is your hero's name?")
        .with_default("Thorin")
        .prompt()
        .unwrap_or_else(|_| "Hero".to_string());

    let mut player = Player::new(player_name);
    let mut state = GameState::Exploring;
    let mut current_location = Location::Tavern;

    println!("\nWelcome, {PALADIN_GOLD}{}{PALADIN_GOLD:#}!", player.name);

    let class_options = vec![
        "Warrior",
        "Rogue",
        "Mage",
    ];
    loop {
        let ans = Select::new("What is your class?", class_options.clone()).prompt();

        match ans {
            Ok(choice) => {
                println!("Your journey begins in a dimly lit tavern {}...\n", choice);
                break;
            },
            Err(_) => println!("There was an error, please try again."),
        }
    }
    
    loop {
        println!("{SYSTEM_GRAY}[HP: {HEAL_GREEN}{}/{}{HEAL_GREEN:#}{SYSTEM_GRAY}] | LVL: {}{SYSTEM_GRAY:#}", player.health, player.max_health, player.level);
        match state {
            GameState::Exploring => {
                state = world::explore_location(&mut player, &mut current_location);
            }
            GameState::Exit => {
                let ans = Confirm::new("Are you sure you want to quit?")
                    .with_default(false).prompt();

                match ans {
                    Ok(true) => {
                        println!("\nThank you for playing!");
                        break;
                    },
                    Ok(false) => state = GameState::Exploring,
                    Err(_) => state = GameState::Exploring,
                }
                
            }
            GameState::Combat => {
                if combat::start_battle(&mut player) {
                    state = GameState::Exploring;
                } else {
                    println!("You died...");
                    state = GameState::Exit;
                }
                
            } 
            GameState::Inventory => {
                player.show_inventory();
                state = GameState::Exploring;
            }
        }
    }

    println!("{SYSTEM_GRAY}Game closed successfully.{SYSTEM_GRAY:#}");
    
}

fn run_main_menu() -> GameState {
    let options = vec![
        "Go Exploring",
        "Open Inventory",
        "Quit Game",
    ];

    let ans = Select::new("What would you like to do?", options).prompt();

    match ans {
        Ok("Go Exploring") => GameState::Exploring,
        Ok("Open Inventory") => GameState::Inventory,
        Ok("Quit Game") => GameState::Exit,
        _ => {
            println!("{SYSTEM_GRAY}Invalid option. Please type 1 or 2.{SYSTEM_GRAY:#}");
            GameState::Exploring
        }
    }
}

// User input helper
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");

    input.trim().to_string()
}