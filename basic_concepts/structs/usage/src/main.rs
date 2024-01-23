#[derive(Debug)]
struct Player {
    player_name: String,
    pos_x: i32,
    pos_y: i32,
    hp: u16,
    is_alive: bool,
}

fn main() {
    let mut player1 = Player {
        player_name: String::from("John Cena"),
        pos_x: 0,
        pos_y: 0,
        hp: 100,
        is_alive: true,
    };

    let player2 = Player {
        player_name: String::from("Prince Vegeta"),
        ..player1
    };
    println!("Name: {}", player1.player_name);
    println!("HP: {}", player1.hp);
    println!("Name: {}", player2.player_name);
    println!("HP: {}", player2.hp);
}
