// src/main.rs
use std::io::{self, Write};

mod combat;
mod enemy;
mod player;
use player::Player;

enum GameState {
    MainMenu,
    Exploring,
    Combat,
    Resting,
    Exit,
}

fn main(){

    // Create the player using constructor
    let mut player = Player::new(get_user_input("What is your name adventurer?: "));
    let mut state = GameState::MainMenu;
    
    loop {
        match state {
            GameState::MainMenu => {
                player.print_status();
                state = run_main_menu();
            }
            GameState::Exploring => {
                println!("\n\x1b[32m[Exploring] You step into the dark woods...\x1b[0m");
                state = GameState::Combat;
            }
            GameState::Exit => {
                println!("\nThank you for playing!");
                break;
            }
            GameState::Combat => {
                if combat::start_battle(&mut player) {
                    state = GameState::MainMenu;
                } else {
                    println!("You died...");
                    state = GameState::Exit;
                }
                
            } 
            GameState::Resting => todo!()
        }
    }

    println!("Game closed successfully.");
    
}

fn run_main_menu() -> GameState {
    println!("\n--- Actions ---");
    println!("[1] Go Exploring");
    println!("[2] Quit Game");

    let choice = get_user_input("\nSelect an option: ");

    match choice.as_str() {
        "1" | "start" | "start new game" => GameState::Exploring,
        "2" | "quit" | "q" | "exit" => GameState::Exit,
        _ => {
            println!("\x1b[31mInvalid option. Please type 1 or 2.\x1b[0m");
            GameState::MainMenu
        }
    }
}

// User input helper
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");

    input.trim().to_string()
}