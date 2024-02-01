pub trait SpecialMove {
    fn sp_move (&self) -> String;
}

struct Pokemon {
    pub id: u32,
    pub name: String,
    pub special_move: String,
    pub types: Vec<String>
}

impl SpecialMove for Pokemon {
    fn sp_move (&self) -> String {
        format!("id: {}, name: {}, sp_move: {}", self.id, self.name, self.special_move)
    }
}

struct Digimon {
    pub name: String,
    pub special_move: String
}


impl SpecialMove for Digimon {
    fn sp_move (&self) -> String {
        format!("name: {}, sp_move: {}", self.name, self.special_move)
    }
}

fn main() {
    let charizard = Pokemon {
        id: 6,
        name: String::from("Charizard"),
        special_move: String::from("Flamethrower"),
        types: vec![String::from("Fire"), String::from("Flying")],
    };

    let black_war_greymon = Digimon {
        name: String::from("Black War Greymon"),
        special_move: String::from("Terra Destroyer"),
    };

    println!("{}", charizard.sp_move());
    println!("{}", black_war_greymon.sp_move());
}
