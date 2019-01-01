#![allow(dead_code)]

// use std::io;
mod game;

fn main() {
    let game = game::Game::new();

    game.turn();
}