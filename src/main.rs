extern crate rand;

use rand::Rng;
use std::{thread, time};

mod board;
mod gamestate;
mod piece;

pub fn main() {
    let mut game = gamestate::GameState::new();
    let mut rng = rand::thread_rng();

    game.set_board_size(10, 18);

    loop {
        game.spawn_random_piece();
        let direction = rng.gen_range(0, 2);
        let amount = rng.gen_range(0, 5);
        let rotation = rng.gen_range(0, 4);

        for _ in 0..rotation {
            game.board.rotate_active_piece_right();
            game.update_view();
        }

        for _ in 0..amount {
            let moved = match direction {
                0 => game.board.move_active_piece_left(),
                1 => game.board.move_active_piece_right(),
                _ => unreachable!(),
            };

            game.update_view();

            if !moved {
                break;
            }
        }

        while {
            let moved = game.board.move_active_piece_down();
            game.update_view();
            moved
        } {}
    }
}
