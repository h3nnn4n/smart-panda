extern crate rand;

use rand::Rng;
use std::{thread, time};

mod board;
mod gamestate;
mod piece;

pub fn main() {
    let mut game = gamestate::GameState::new();

    game.demo_random();
}
