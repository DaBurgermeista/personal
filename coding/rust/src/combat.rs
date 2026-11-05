// src/combat.rs

use crate::player::Player;
use std::io::{self, Write};

pub enum CombatAction {
    Attack,
    Rest,
    Flee,
}

struct Enemy {
    name: String,
    hp: i32,
    damage: i32,
}

pub fn start_battle(player: &mut Player) {
    let mut goblin = Enemy {
        name: String::from("Cave Goblin"),
        hp: 30,
        damage: 8,
    };

    println!("\n\x1b[31mA wild {} rushes out from the shadows!\x1b[0m", goblin.name);

    while player.health > 0 && goblin.hp > 0 {
        print_combat_status(player, &goblin);

        let action = get_combat_action();

        match action {
            CombatAction::Attack => {
                let player_damage = 10;
                goblin.hp -= player_damage;
                println!("\nYou slice the {} for \x1b[36m{} damage\x1b[0m!", goblin.name, player_damage);
            }
            CombatAction::Rest => {
                let heal_amount = 15;
                player.health += (player.health + heal_amount).min(player.max_health);
                println!("\nYou take a defensive stance and catch your breath, recovering \x1b[32m{} HP\x1b[0m.", heal_amount);
            }
            CombatAction::Flee => {
                println!("\nYou manage to scramble away back to safety!");
                return;
            }
        }

        if goblin.hp > 0 {
            player.take_damage(goblin.damage);
            println!("The {} retaliates, hitting your for \x1b[31m{} damage\x1b[0m!", goblin.name, goblin.damage);

        }
    }

    if player.health <= 0 {
        println!("\n\x1b[31mYou have been defeated in battle... Game Over.\x1b[0m");
        return;
    } else {
        println!("\n\x1b[32mVictory! You have vanquished the {}!\x1b[0m", goblin.name);
        return;
    }
}

fn print_combat_status(player: &Player, enemy: &Enemy) {
    println!("\n-------------------------------");
    println!("{}: \x1b[32m{}/{} HP\x1b[0m", player.name, player.health, player.max_health);
    println!("{}: \x1b[31m{} HP\x1b[0m", enemy.name, enemy.hp);
    println!("-------------------------------");
    println!("[1] Attack");
    println!("[2] Rest / Catch Breath");
    println!("[3] Flee");
}

fn get_combat_action() -> CombatAction {
    loop {
        print!("Choose your action: ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        match input.trim().to_lowercase().as_str() {
            "1" => return CombatAction::Attack,
            "2" => return CombatAction::Rest,
            "3" => return CombatAction::Flee,
            _ => println!("\x1b[31mInvalid action. Try again.\x1b[0m"),
        }
    }
}