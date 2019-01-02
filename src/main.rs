#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

// use std::io;
mod game;

pub use self::game::Game;
pub use self::game::PlayerCount;

fn main() {
    let game: Game = Game::new(
        PlayerCount::Two
    );
}