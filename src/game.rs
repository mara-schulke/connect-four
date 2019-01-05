pub mod field;
pub use self::field::Field;

pub struct Game {
    field: Field,
    pub players: Vec<u8>,
    curentPlayer: u8
}

impl Game {
    pub fn new( playerCount: u8, col: u8, row: u8 ) -> Self {
        let mut Players = vec![]; 

        for i in 0..playerCount {
            Players.push( i );
        }

        Game {
            field: Field::new(col, row),
            players: Players,
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