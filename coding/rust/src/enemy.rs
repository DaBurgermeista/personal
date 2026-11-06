// src/enemy.rs
use std::time::{SystemTime, UNIX_EPOCH};
use crate::items::Weapon;

pub struct Enemy {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub damage: i32,
    pub exp: i32,
}

impl Enemy {
    // Base constructor
    pub fn new(name: &str, health: i32,  damage: i32, exp: i32) -> Self {
        Enemy {
            name: name.to_string(),
            health,
            max_health: health,
            damage,
            exp,
        }
    }

    // --- Preset Enemy Types ---
    pub fn enemy() -> Self {
        Self::new("Cave Goblin", 30, 8, 5)
    }

    pub fn skeleton() -> Self {
        Self::new("Rattling Skeleton", 45, 12, 8)
    }

    pub fn dragon() -> Self {
        Self::new("Shadow Dragon", 100, 20, 25)
    }

    // --- Random Spanwer ---
    pub fn spawn_random() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;

        let roll = seed % 100;

        if roll < 50 {
            Self::enemy()      // 50% Chance
        } else if roll < 85 {
        Self::skeleton()        // 35% Chance
        } else {
            Self::dragon()      // 15% Chance
        }
    }
}