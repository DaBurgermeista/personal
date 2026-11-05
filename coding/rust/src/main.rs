use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

mod player;

fn main(){

    print!("What is your name adventurer?: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let trimmed_input = user_input.trim().to_string();

    // Create the player using constructor
    let mut player = player.new(trimmed_input);
    
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
