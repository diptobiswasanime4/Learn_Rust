#[derive(Debug)]
struct Player {
    player_name: String,
    pos_x: i32,
    pos_y: i32,
    hp: u16,
    is_alive: bool,
}

impl Player {
    fn health_check(self: &Self) -> String {
        if self.hp == 100 {
            "Full health.".to_string()
        } else {
            "Not full health.".to_string()
        }
    }

    fn health_compare(self: &Self, player: &Player) -> String {
        let mut resp: String;
        if self.hp > player.hp {
            resp = format!("{} has more hp compared to {}", self.player_name, player.player_name);
        } else if self.hp < player.hp {
            resp = format!("{} has more hp compared to {}", player.player_name, self.player_name);
        } else {
            resp = format!("Both players have equal hp");
        }
        resp
    }
}

fn main() {
    let player1 = populate_struct(String::from("Prince Vegeta"), 50);
    let player2 = populate_struct(String::from("John Cena"), 50);

    println!("{:#?}", player1);
    println!("{:#?}", player2);

    println!("{}", player1.health_check());
    println!("{}", player2.health_check());

    println!("{}", player1.health_compare(&player2));
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