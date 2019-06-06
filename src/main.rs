fn main() {
    let game: Game = Game::new(2);
}

enum PlayerSymbol {
    First,
    Second,
    Third,
    Fourth,
}

impl PlayerSymbol {
    pub fn from_u8(s: u8) -> Option<PlayerSymbol> {
        match s {
            1 => Some(PlayerSymbol::First),
            2 => Some(PlayerSymbol::Second),
            3 => Some(PlayerSymbol::Third),
            4 => Some(PlayerSymbol::Fourth),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PlayerSymbol::First => "x",
            PlayerSymbol::Second => "o",
            PlayerSymbol::Third => "+",
            PlayerSymbol::Fourth => "*",
        }
    }
}

struct Position {
    pub x: i32,
    pub y: i32,
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
        if players < 2 {
            panic!("You can't play this game alone.")
        } else if players > 4 {
            panic!("You can't play this game with more than 4 players.")
        }

        Game {
            players,
            field: Field::new(),
        }
    }
}

struct Field {
    coins: Vec<Vec<Option<Coin>>>,
}

impl Field {
    pub fn new() -> Field {
        // initialize a 8 x 8 field
        Field {
            coins: vec![
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
            ],
        }
    }
}
