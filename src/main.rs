extern crate rand;

mod board;
mod gamestate;
mod piece;

pub fn main() {
    let mut game = gamestate::GameState::new();

    game.demo_random();
}
