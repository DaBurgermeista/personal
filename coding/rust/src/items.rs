// src/items.rs

#[derive(Clone, Copy, PartialEq)] // Added PartialEq to allow checking if weapon == Weapon::Fists
pub enum Weapon {
    Fists,
    RustyDagger,
    IronSword,
    Greatsword,
}

impl Weapon {
    pub fn name(&self) -> &str {
        match self {
            Weapon::Fists => "Fists",
            Weapon::RustyDagger => "Rusty Dagger",
            Weapon::IronSword => "Iron Sword",
            Weapon::Greatsword => "Steel Greatsword",
        }
    }

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
    ElixirOfStrength,
}

impl Consumable {
    pub fn name(&self) -> &str {
        match self {
            Consumable::HealthPotion => "Minor Health Potion",
            Consumable::ElixirOfStrength => "Elixir of Strength",
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
