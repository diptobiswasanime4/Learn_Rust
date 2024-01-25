#[derive(Debug)]
struct Player {
    name: String,
    age: u32,
    score: Option<u32>,
    team: Option<String>,
}

fn main() {
    let player1 = Player {
        name: String::from("Ash Ketchum"),
        age: 10,
        score: Some(100),
        team: Some(String::from("Forever Young")),
    };

    let player2 = Player {
        name: String::from("John Cena"),
        age: 40,
        score: None,
        team: None,
    };

    let player1 = populate_player(String::from("Ash Ketchum"), 10, Some(100), Some(String::from("Forever Young")));
    let player2 = populate_player(String::from("John Cena"), 40, None, None);

    println!("{:#?}", player1);
    println!("{:#?}", player2);
}

fn populate_player(name: String, age: u32, score: Option<u32>, team: Option<String>) -> Player {
    Player {
        name,
        age,
        score,
        team,
    }
}


