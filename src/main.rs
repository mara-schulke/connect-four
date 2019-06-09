use std::fmt;
use std::io;

type PlayerId = u8;

fn main() {
    utils::clear_screen();
    utils::print_logo();

    // Input variable for the player count.
    println!("Please choose a player amount from 2 to 4:");
    let mut player_count: u8;
    // Input loop -> loop until a valid input was provided.
    loop {
        player_count = match utils::input().parse() {
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
        utils::clear_screen();

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
            column = match utils::input().parse() {
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
            utils::clear_screen();
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

// The Player Symbol enum holds all available player symbols
// and maps the symbols to numbers and vice versa.
#[derive(Debug)]
pub enum PlayerSymbol {
    One,
    Two,
    Three,
    Four,
}

impl PlayerSymbol {
    // This maps a number to a PlayerSymbol if one is given
    pub fn from_player_id(s: PlayerId) -> Option<PlayerSymbol> {
        match s {
            1 => Some(PlayerSymbol::One),
            2 => Some(PlayerSymbol::Two),
            3 => Some(PlayerSymbol::Three),
            4 => Some(PlayerSymbol::Four),
            _ => None,
        }
    }

    // This function is used to map the symbols to thier string representatives.
    pub fn as_str(&self) -> &'static str {
        match self {
            PlayerSymbol::One => "x",
            PlayerSymbol::Two => "o",
            PlayerSymbol::Three => "+",
            PlayerSymbol::Four => "*",
        }
    }
}

// The Coin Struct is used to keep track of the player moves.
// It holds the player id of the player it belongs to and its symbol.
#[derive(Debug)]
pub struct Coin {
    player_id: u8,
    symbol: PlayerSymbol,
}

impl Coin {
    // Create a new coin from a player id.
    pub fn new(player_id: PlayerId) -> Coin {
        Coin {
            player_id,
            symbol: PlayerSymbol::from_player_id(player_id).unwrap(),
        }
    }
}

// Implement fmt::Display to convienently print out coins.
impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This uses the string representative of the symbol.
        write!(f, "{}", self.symbol.as_str(),)
    }
}

// The Game Struct holds all the game state.
#[derive(Debug)]
pub struct Game {
    // The Player Amount
    players: u8,
    // The Current Player
    current_player: u8,
    // The Round Number
    round: u32,
    // If the Game is Over or Not
    over: bool,
    // The Game Field of Coins
    field: Vec<Vec<Option<Coin>>>,
}

impl Game {
    // Create a new Game Struct from a player count.
    pub fn new(players: u8) -> Game {
        // Create Boundaries to ensure enough symbols are available.
        if players < 2 {
            utils::exit_with_message("You can't play this game alone.");
        } else if players > 4 {
            utils::exit_with_message("You can't play this game with more than 4 players.");
        }

        Game {
            players,
            current_player: 1,
            round: 1,
            over: false,
            // Initialize a 8 x 8 field
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

    // Insert a coin into the game field, into a specific column
    fn insert_coin(&mut self, x: usize) {
        // Y Pos of the Field to insert the coin in.
        let mut y: usize = 0;

        // TODO: Improve performance through looping backwards -> less steps!
        // Loop trough all rows in the Field.
        for row in &self.field {
            // If a coin is in the current field -> break the loop.
            if let Some(_) = &row[x] {
                break;
            }

            // Otherwise just increase the Y Pos.
            y += 1;
        }

        // Decrease it by 1 to get the field above the first coin.
        y -= 1;

        // Create a new Coin in this field.
        self.field[y][x] = Some(Coin::new(self.current_player));
    }

    // Checks if a player, which id gets passed to the function, has won.
    fn check_if_player_won(&self, player_id: PlayerId) -> bool {
        // This Function is devided into 4 Sections:
        // 1. Check for a row streak
        // 2. Check for a column streak
        // 3. Check for a diagonal streak from top left to bottom right
        // 4. Check for a diagonal streak from bottom left to top right

        // Check if the current player got a row streak
        {
            // Loop through all rows
            for y in 0..self.field.len() {
                // Each row has a highest streak -> debug purposes
                let mut highest_streak = 0;

                // Loop trough the first 5 coins in each row.
                // The last 3 are unimportant since they cant start a streak,
                // thats long enough to win the game.
                for x in 0..(self.field[y].len() / 2 + 1) {
                    let mut streak = 0;

                    // Go through this and the next 3 coins to check thier identity.
                    for i in 0..4 {
                        // If the coin belongs to this player and doesnt overflow the vector, increment the streak.
                        if x + i < self.field[y].len()
                            && utils::coin_has_player_id(&self.field[y][x + i], player_id)
                        {
                            streak += 1;
                        } else {
                            break;
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

        // Check if the current player got a column streak
        {
            // Loop through the first 5 rows
            for y in 0..(self.field.len() / 2 + 1) {
                // Each column has a highest streak -> debug purposes
                let mut highest_streak = 0;

                // Loop trough the first 5 coins in each row.
                // The last 3 are unimportant since they cant start a streak,
                // thats long enough to win the game.
                for x in 0..self.field[y].len() {
                    let mut streak = 0;

                    // Go through this and the next 3 coins to check thier identity.
                    for i in 0..4 {
                        // If the coin belongs to this player and doesnt overflow the vector, increment the streak.
                        if y + i < self.field.len()
                            && utils::coin_has_player_id(&self.field[y + i][x], player_id)
                        {
                            streak += 1;
                        } else {
                            break;
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

        // Check if the current player got a diagonal streak from top left to bottom right
        {
            // Loop through the first 5 rows
            for y in 0..(self.field.len() / 2 + 1) {
                // Each column has a highest streak -> debug purposes
                let mut highest_streak = 0;

                // Loop trough the first 5 coins in each row.
                // The last 3 are unimportant since they cant start a streak,
                // thats long enough to win the game.
                for x in 0..(self.field[y].len() / 2 + 1) {
                    let mut streak = 0;

                    // Go through this and the next 3 coins to check thier identity.
                    for i in 0..4 {
                        // If the coin belongs to this player and doesnt overflow the vector, increment the streak.
                        if x + i < self.field[x].len()
                            && y + i < self.field.len()
                            && utils::coin_has_player_id(&self.field[y + i][x + i], player_id)
                        {
                            streak += 1;
                        } else {
                            break;
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

        // Check if the current player got a diagonal streak from bottom left to top right
        {
            // // Loop through the first 5 rows
            for y in (self.field.len() / 2 - 1)..self.field.len() {
                // Each column has a highest streak -> debug purposes
                let mut highest_streak = 0;

                // Loop trough the first 5 coins in each row.
                // The last 3 are unimportant since they cant start a streak,
                // thats long enough to win the game.
                for x in 0..(self.field[y].len() / 2 + 1) {
                    let mut streak = 0;

                    // Go through this and the next 3 coins to check thier identity.
                    for i in 0..4 {
                        // If the coin belongs to this player and doesnt overflow the vector, increment the streak.
                        if x + i < self.field[y].len()
                            && utils::coin_has_player_id(&self.field[y - i][x + i], player_id)
                        {
                            streak += 1;
                        } else {
                            break;
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

        // If none of the blocks of above returned true - the current player didn't won.
        false
    }

    // This is used to check if a given column is completly full.
    fn is_col_full(&self, x: usize) -> bool {
        // Amount of coins in this column
        let mut counter: usize = 0;

        // For every row in the field
        for row in &self.field {
            // Check if theres a coin at a given x coordinate.
            if let Some(_coin) = &row[x] {
                // If so, count this coin.
                counter += 1;
            }
        }

        // If the amount if coins equals the length of a column, return true.
        if counter == self.field.len() {
            return true;
        }

        // Otherwise return false, since there less than 8 coins in this column.
        false
    }

    // This returns the next player.
    fn next_player(&self) -> PlayerId {
        // If the current player is the last player, return the first player.
        if self.current_player == self.players {
            return 1;
        }

        // Otherwise just go on and count up.
        self.current_player + 1
    }

    // Just a utility which adds 1 to the current round.
    fn next_round(&self) -> u32 {
        self.round + 1
    }

    // Prints the field to the terminal.
    fn print_field(&self) {
        // Space the field from the rest of the output.
        print!("\n");

        // Display a number aboce each column
        for index in 1..self.field.len() + 1 {
            print!(" {} ", index);
        }

        print!("\n");

        for row in &self.field {
            // For each coin in the field
            for coin in row {
                // If a coin exists, output a [ ] with the player symbol inside
                if let Some(c) = coin {
                    print!("[{}]", c);
                }
                // Else just print [ ]
                else {
                    print!("[ ]")
                }
            }

            print!("\n");
        }

        // Space the field from the rest of the output.
        print!("\n");
    }
}

mod utils {
    use super::*;

    // Easily get User Input.
    pub fn input() -> String {
        let mut input = String::new();

        // Read the next line from stdin into 'input'
        match io::stdin().read_line(&mut input) {
            // Everything is fine, so just go on
            Ok(_) => {}
            // TODO: Use an input loop.
            // Error -> process exits.
            Err(_) => exit_with_message("Crashed while reading from stdin."),
        }

        // This line splits the '\n' of and allows us to parse the input into types.
        input.split_off(input.len() - 1);

        // If the input equals the command :q, exit the process with 0;
        if input == ":q" {
            clear_screen();
            std::process::exit(0);
        }

        // Otherwise just return the optimized input
        input
    }

    // Utility to print a message and exit the process
    pub fn exit_with_message(msg: &str) {
        println!("{}", msg);
        std::process::exit(0)
    }

    // Clears the terminal output
    pub fn clear_screen() {
        // Store the output of 'clear' in this variable.
        let output = std::process::Command::new("clear").output();

        // If 'clear' gave any output -> print it
        if let Ok(output) = output {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        // Otherwise print some new line characters.
        else {
            println!("\n\n\n\n");
        }
    }

    // Prints the 'figlet connect four' output.
    pub fn print_logo() {
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

    // Checks if a coin exits and if it belongs to the given player_id.
    pub fn coin_has_player_id(coin: &Option<Coin>, player_id: u8) -> bool {
        // If the Coin is some
        if let Some(c) = coin {
            // And the coin belongs to the player -> return true;
            if c.player_id == player_id {
                return true;
            }
        }

        // Otherwise false.
        false
    }
}
