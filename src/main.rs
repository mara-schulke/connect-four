fn main() {
    let mut game: Game = Game::new(4);

    loop {
        println!(
            "Player {}, please choose a row to enter your coin!",
            game.current_player
        );

        print_game_field(&game.field);
        // game.current_player = next_player(&game);

        break;
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
struct Coin {
    player_id: u8,
    symbol: PlayerSymbol,
    pos: Position,
}

impl Coin {
    pub fn new(player_id: u8, x: i32, y: i32) -> Coin {
        Coin {
            player_id,
            symbol: PlayerSymbol::from_u8(player_id).unwrap(),
            pos: Position { x, y },
        }
    }
}

#[derive(Debug)]
struct Game {
    players: u8,
    current_player: u8,
    round: u32,
    field: Vec<Vec<Option<Coin>>>,
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
            current_player: 1,
            round: 0,
            // initialize a 8 x 8 field
            field: vec![
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![
                    None,
                    None,
                    Some(Coin::new(2, 3, 3)),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ],
        }
    }
}

fn next_player(g: &Game) -> u8 {
    if g.current_player == g.players {
        return 1;
    }

    g.current_player + 1
}

fn print_game_field(field: &Vec<Vec<Option<Coin>>>) {
    print!("\n");

    for col in field {
        for coin in col {
            print!(
                "[{}]",
                match coin {
                    Some(c) => c.symbol.as_str(),
                    None => " ",
                }
            );
        }

        print!("\n");
    }

    print!("\n");
}
