use std::io;

fn main() {
    let mut game: Game = Game::new(4);

    loop {
        clear_screen();
        println!(
            "Player {}, please choose a row to enter your coin!",
            game.current_player
        );

        game.print_field();

        // Handle input
        println!("Enter column to insert coin:");
        let mut column: usize;
        loop {
            // make this a cleaner exit (or try again)
            column = match input().parse() {
                Ok(val) => val,
                Err(_) => {
                    exit_with_message("Failed to parse input into a number!");
                    0
                }
            };

            if column > 8 {
                println!("Please choose a column from 1 to 8:");
                continue;
            } else if game.is_col_full(column) {
                println!("Please choose a column thats not full:");
                continue;
            }

            break;
        }

        game.insert_coin(column);

        game.current_player = game.next_player();

        if game.over {
            let winner_id = game.get_winner().unwrap();
            println!("Player {:?} won the game!", winner_id);

            break;
        }
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
struct Coin {
    player_id: u8,
    symbol: PlayerSymbol,
}

impl Coin {
    pub fn new(player_id: u8) -> Coin {
        Coin {
            player_id,
            symbol: PlayerSymbol::from_u8(player_id).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Game {
    players: u8,
    current_player: u8,
    round: u32,
    over: bool,
    field: Vec<Vec<Option<Coin>>>,
}

impl Game {
    pub fn new(players: u8) -> Game {
        if players < 2 {
            exit_with_message("You can't play this game alone.");
        } else if players > 4 {
            exit_with_message("You can't play this game with more than 4 players.");
        }

        Game {
            players,
            current_player: 1,
            round: 0,
            over: false,
            // initialize a 8 x 8 field
            field: vec![
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

    // improve performance through looping backwards -> less steps!
    fn insert_coin(&mut self, x: usize) {
        let mut y: usize = 0;

        for row in &self.field {
            if let Some(coin) = &row[x] {
                println!("{:?}", coin);
                break;
                // exit_with_message("found coin");
            }

            y += 1;
        }

        y -= 1;

        self.field[y][x] = Some(Coin::new(self.current_player));

        println!("{}", y);
    }

    fn is_col_full(&self, x: usize) -> bool {
        let mut counter: usize = 0;

        for row in &self.field {
            if let Some(coin) = &row[x] {
                counter += 1;
            }
        }

        if counter == self.field.len() {
            return true;
        }

        false
    }

    fn next_player(&self) -> u8 {
        if self.current_player == self.players {
            return 1;
        }

        self.current_player + 1
    }

    fn print_field(&self) {
        print!("\n");

        // number display
        for index in 1..self.field.len() + 1 {
            print!(" {} ", index);
        }

        print!("\n");

        for row in &self.field {
            for coin in row {
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

    fn get_winner(&self) -> Option<u8> {
        Some(self.current_player)
    }
}

fn input() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => exit_with_message(""),
    }

    // split \n of
    input.split_off(input.len() - 1);

    input
}

fn exit_with_message(msg: &str) {
    println!("{}", msg);
    std::process::exit(0)
}

fn clear_screen() {
    let output = std::process::Command::new("clear")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
