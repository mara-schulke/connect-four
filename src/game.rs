pub mod field;
pub use self::field::Field;

pub struct Game {
    field: Field,
    // players: Players,
    curentPlayer: u8
}

impl Game {
    pub fn new( playerCount: u8, col: u8, row: u8 ) -> Self {
        let mut players : [ u8; playerCount ] = []; 

        for 0..playerCount {

        }

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