#[derive(Debug)]
struct Player {
    player_name: String,
    pos_x: i32,
    pos_y: i32,
    hp: u16,
    is_alive: bool,
}

fn main() {
    let player1 = populate_struct(String::from("Prince Vegeta"), 50);
    println!("{:#?}", player1);
}

fn populate_struct(player_name: String, hp: u16) -> Player {
    Player {
        player_name,
        pos_x: 0,
        pos_y: 0,
        hp,
        is_alive: true
    }
}