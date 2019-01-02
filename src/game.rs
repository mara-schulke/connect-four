pub mod field;
pub use self::field::Field;

fn matchDecimalEnum <T> ( enum ) -> <T> {
    match enum {
        enum::Two => { 2 },
        enum::Three => { 3 }, 
        enum::Four => { 4 }
    };
}

pub enum PlayerCount {
    Two,
    Three,
    Four
}

pub struct Game {
    field: Field,
    // players: Players,
    curentPlayer: u8
}

impl Game {
    pub fn new( playerCount: PlayerCount, col: u8, row: u8 ) -> Self {



        Game {
            field: Field::new(),
            curentPlayer: 0
        }
    }

    pub fn start(&self) {
        println!("ie");
    }

    pub fn turn(&self) {
        println!("turn");
    }
}