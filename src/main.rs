fn main() {
    let game: Game {
        players: 2,
    }
}

struct Position {
    x: i32,
    y: i32,
}

struct Coin {
    player_id: u8,
    pos: Position,
}

impl Coin {
    pub fn new(player_id: u8, x: i32, y: i32) -> Coin {
        Coin {
            player_id,
            pos: Position { x, y },
        }
    }
}

struct Game {
    players: u8,
    field: Field,
}

impl Game {
    pub fn new(players: u8) -> Game {
        if players < 2 { panic!("You can't play this game alone.")}
        else if players > 4 { panic!("You can't play this game with more than 4 players.")}

        Game {
            players,
            field: Field::new()
        }
    }
}

struct Field {
    coins: Vec<Vec<Coin>>
}

impl Field {
    
}
