// Placeholder Rust file
//

struct Player {
    name: String,
    health: i32,
}

fn main(){
    let player = Player{name: "Borg".to_string(), health: 35};
    println!("Hello {}! Your health is currently: {}", &player.name, &player.health);
}
