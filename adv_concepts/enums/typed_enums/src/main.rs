struct Sprite {
    name: String,
    health: u64
}

enum Characters {
    Hero(Sprite),
    Enemy(Sprite),
    Boss(Sprite)
}

fn main() {
    let hero_sprite = Sprite {
        name: String::from("Jason"),
        health: 100
    };

    let boss_sprite = Sprite {
        name: String::from("Hydra"),
        health: 500
    };

    let hero_character = Characters::Hero(hero_sprite);
    let boss_character = Characters::Boss(boss_sprite);

    gameplay_description(hero_character);
    gameplay_description(boss_character);
}

fn gameplay_description (character: Characters) {
    match character {
        Characters::Hero(sprite) => println!("{} has arrived.", sprite.name),
        Characters::Enemy(sprite) => println!("It's a no name Enemy."),
        Characters::Boss(sprite) => println!("Beware of {}!", sprite.name),
    }
}
