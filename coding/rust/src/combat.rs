// src/combat.rs

use crate::enemy::Enemy;
use crate::player::Player;
use crate::{SYSTEM_GRAY, DAMAGE_RED, HEAL_GREEN, PALADIN_GOLD};
use inquire::{Select, Text};
use anstyle::{AnsiColor, Color, Style};
use rand::Rng;


pub enum CombatAction {
    Attack,
    Rest,
    Flee,
}

pub fn start_battle(player: &mut Player) -> bool {
    let mut enemy = Enemy::spawn_random();
    let mut rng = rand::thread_rng();

    println!("\n{DAMAGE_RED}A wild {PALADIN_GOLD}{}{PALADIN_GOLD:#} {DAMAGE_RED}rushes out from the shadows!\x1b[0m", enemy.name);

    while player.health > 0 && enemy.health > 0 {
        print_combat_status(player, &enemy);

        let action = get_combat_action();

        match action {
            CombatAction::Attack => {
                let player_damage = player.get_attack_power();
                enemy.health -= player_damage;
                println!("\nYou slice the {} for {DAMAGE_RED}{} damage{DAMAGE_RED:#}!", enemy.name, player_damage);
            }
            CombatAction::Rest => {
                let heal_amount = 15;
                player.health = (player.health + heal_amount).min(player.max_health);
                println!("\nYou take a defensive stance and catch your breath, recovering {HEAL_GREEN}{} HP{HEAL_GREEN:#}.", heal_amount);
            }
            CombatAction::Flee => {
                println!("\nYou manage to scramble away back to safety!");
                return true
            }
        }

        if enemy.health > 0 {
            if player.take_damage(enemy.damage) {
                println!("The {} retaliates, hitting your for {DAMAGE_RED}{} damage{DAMAGE_RED:#}!", enemy.name, enemy.damage);
            } else {
                println!("\n{DAMAGE_RED}You have been defeated in battle... Game Over.{DAMAGE_RED:#}");
                return false
            }

        }
    }

    println!("\n\x1b[32mVictory! You have vanquished the {}!\x1b[0m", enemy.name);
    player.add_experience(enemy.exp);
    let gold_gain = rng.gen_range(0..=(enemy.level * 10));
    return true
}

fn print_combat_status(player: &Player, enemy: &Enemy) {
    println!("\n-------------------------------");
    println!("{}: {HEAL_GREEN}{}/{} HP{HEAL_GREEN:#}", player.name, player.health, player.max_health);
    println!("{}: {DAMAGE_RED}{}/{} HP{DAMAGE_RED:#}", enemy.name, enemy.health, enemy.max_health);
    println!("-------------------------------");
    println!("[1] Attack");
    println!("[2] Rest / Catch Breath");
    println!("[3] Flee");
}

fn get_combat_action() -> CombatAction {
    let options = vec![
        "Attack",
        "Rest",
        "Flee",
    ];

    loop {
        let ans = Select::new("What would you like to do?", options.clone()).prompt();

        match ans {
            Ok("Attack") => return CombatAction::Attack,
            Ok("Rest")   => return CombatAction::Rest,
            Ok("Flee")   => return CombatAction::Flee,
            _            => println!("{DAMAGE_RED}You must choose an action to continue.{DAMAGE_RED:#}"),
        }
    }
    
}