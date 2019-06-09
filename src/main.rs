use std::io;

fn main() {
    clear_screen();
    print_logo();

    // Input variable for the player count.
    println!("Please choose a player amount from 2 to 4:");
    let mut player_count: u8;
    // Input loop -> loop until a valid input was provided.
    loop {
        player_count = match input().parse() {
            // If the input is okay, assign it to player_count.
            Ok(val) => val,
            // If the input isn't okay, skip the current iteration and try it again.
            Err(_) => {
                println!("Please enter a number, nothing else:");
                continue;
            }
        };

        // If the player count is less than 2 or bigger than 4,
        // skip the current iteration and try it again.
        if player_count < 2 || player_count > 4 {
            println!("Please choose a player amount from 2 to 4:");
            continue;
        }

        // If the input was okay and no other conditions skiped the current iteration,
        // break the input loop and use the given value.
        break;
    }

    // Create the Game State with the given player count.
    let mut game: Game = Game::new(player_count);

    // Main Game Loop ( each round is one iteration ).
    loop {
        clear_screen();

        // Display the "UI"
        println!("Player: {} - Round: {}", game.current_player, game.round);
        game.print_field();

        // Stop the game if the field is full.
        if game.round > (8 * 8) {
            println!("Game draw! No fields left..");
            break;
        }

        println!("Please choose a column to enter your coin!");
        // The Column to insert the coin.
        let mut column: usize;
        // Input loop -> loop until a valid input was provided.
        loop {
            // If the input is okay, assign it to column.
            column = match input().parse() {
                Ok(val) => val,
                Err(_) => {
                    // If the input isn't okay, skip the current iteration and try it again.
                    println!("Please enter a number, nothing else:");
                    continue;
                }
            };

            // If the column is less than 1 or bigger than 8,
            // skip the current iteration and try it again.
            if column < 1 || column > 8 {
                println!("Please choose a column from 1 to 8:");
                continue;
            }
            // Else decrease the column by 1, to map the user input to the real index.
            // eg: column 1 ( user input ) -> column 0 ( what this code uses ).
            else {
                column -= 1;
            }

            // If the given column if full, skip this iteration and try it again.
            if game.is_col_full(column) {
                println!("Please choose a column thats not full:");
                continue;
            }

            // If the input was okay and no other conditions skiped the current iteration,
            // break the input loop and use the given value.
            break;
        }

        // At this point, we know the column the user wants to insert his coin in is valid,
        // so we can just insert it without error handling.
        game.insert_coin(column);

        // Check the Game State after the coin was inserted, since now the current player could've won.
        if game.check_if_player_won(game.current_player) {
            clear_screen();
            println!("Player {:?} won the game!", game.current_player);
            game.print_field();
            game.over = true;
            break;
        }

        // Next Round Logic:
        // Set the current player to the next player
        // and increase the round number
        game.current_player = game.next_player();
        game.round = game.next_round();
    }
}

#[derive(Debug)]
enum PlayerSymbol {
    One,
    Two,
    Three,
    Four,
}

impl PlayerSymbol {
    pub fn from_u8(s: u8) -> Option<PlayerSymbol> {
        match s {
            1 => Some(PlayerSymbol::One),
            2 => Some(PlayerSymbol::Two),
            3 => Some(PlayerSymbol::Three),
            4 => Some(PlayerSymbol::Four),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PlayerSymbol::One => "x",
            PlayerSymbol::Two => "o",
            PlayerSymbol::Three => "+",
            PlayerSymbol::Four => "*",
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
            round: 1,
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
            if let Some(_) = &row[x] {
                break;
            }

            y += 1;
        }

        y -= 1;

        self.field[y][x] = Some(Coin::new(self.current_player));
    }

    fn check_if_player_won(&self, player_id: u8) -> bool {
        // Check for a row streak
        {
            // Loop through all rows
            'outer: for y in 0..self.field.len() {
                // each row has a highest streak -> debug purposes
                #[allow(unused_variables)]
                let mut highest_streak = 0;

                // Loop trough the first 4 coins in this row.
                // The last 4 are unimportant since they cant start a streak,
                // thats long enough to win the game.
                for x in 0..self.field[y].len() / 4 {
                    let mut streak = 0;

                    // Go through this and the next 3 coins to check thier identity.
                    for i in 0..4 {
                        // If the coin belongs to this player, increment the streak.
                        if coin_has_player_id(&self.field[y][x + i], player_id) {
                            streak += 1;
                        }
                    }

                    // Reassign the highest streak if the current one is bigger.
                    if streak > highest_streak {
                        highest_streak = streak;
                    }

                    // If the current streak is bigger or equal to 4,
                    // return true to indicate that the current player won.
                    if streak > 3 {
                        return true;
                    }
                }
            }
        }

        // let mut has_col_streak = false;
        // {
        //     let mut streak = 0;

        //     for row in &self.field {}

        //     if streak >= 4 {
        //         has_col_streak = true;
        //     }
        // }

        // let mut has_diagonal_left_to_right_streak = false;
        // {
        //     let mut streak = 0;

        //     for row in &self.field {}

        //     if streak >= 4 {
        //         has_diagonal_left_to_right_streak = true;
        //     }
        // }

        // let mut has_diagonal_right_to_left_streak = false;
        // {
        //     let mut streak = 0;

        //     for row in &self.field {}

        //     if streak >= 4 {
        //         has_diagonal_right_to_left_streak = true;
        //     }
        // }

        false
    }

    fn is_col_full(&self, x: usize) -> bool {
        let mut counter: usize = 0;

        for row in &self.field {
            if let Some(_coin) = &row[x] {
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

    fn next_round(&self) -> u32 {
        self.round + 1
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
}

fn input() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => exit_with_message(""),
    }

    // split \n of
    input.split_off(input.len() - 1);

    if input == ":q" {
        clear_screen();
        std::process::exit(0);
    }

    input
}

fn exit_with_message(msg: &str) {
    println!("{}", msg);
    std::process::exit(0)
}

fn clear_screen() {
    // let output = std::process::Command::new("clear")
    //     .output()
    //     .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    // print!("{}", String::from_utf8_lossy(&output.stdout));
}

fn print_logo() {
    println!(
        "
                                 _      __    
  ___ ___  _ __  _ __   ___  ___| |_   / _| ___  _   _ _ __ 
 / __/ _ \\| '_ \\| '_ \\ / _ \\/ __| __| | |_ / _ \\| | | | '__|
| (_| (_) | | | | | | |  __/ (__| |_  |  _| (_) | |_| | |   
 \\___\\___/|_| |_|_| |_|\\___|\\___|\\__| |_|  \\___/ \\__,_|_|   
                                                            
    "
    );
}

fn coin_has_player_id(coin: &Option<Coin>, player_id: u8) -> bool {
    if let Some(c) = coin {
        if c.player_id == player_id {
            return true;
        }
    }

    false
}
