#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

// use std::io;
mod game;
mod field;

use crate::game::Game as Game;
use crate::field::Field as Field;

fn main() {
    let game = Game::new();

    game.turn();
}