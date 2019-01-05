#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

// use std::io;
mod game;

pub use self::game::Game;

fn main() {
    let game: Game = Game::new(
        2,  // PLAYERCOUNT
        10, // COLS
        10, // ROWS
    );
}