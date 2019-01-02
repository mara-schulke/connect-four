pub mod field;
pub use self::field::Field;

pub struct Game {
    field: Field,
    // players: Players,
    curentPlayer: u8
}

impl Game {
    pub fn new() -> Self {

        return Game {
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