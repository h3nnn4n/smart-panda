#![allow(dead_code)]
use rand::Rng;

use wasm_bindgen::prelude::*;

mod board;
mod piece;

#[wasm_bindgen]
pub struct GameState {
    points: u32,
    lines_cleared: u32,
    board: board::Board,
}

#[wasm_bindgen]
impl GameState {
    pub fn new() -> GameState {
        let bb = board::Board::new();

        GameState {
            points: 0,
            lines_cleared: 0,
            board: bb,
        }
    }

    pub fn get_points(&mut self) -> u32 {
        self.points
    }

    pub fn get_lines_cleared(&mut self) -> u32 {
        self.lines_cleared
    }

    pub fn set_board_size(&mut self, width: u32, height: u32) {
        self.board.set_board_size(width, height);
    }

    pub fn reset(&mut self) {
        self.points = 0;
        self.board.reset();
    }

    pub fn spawn_piece(&mut self, id: u32) {
        self.board.spawn_piece(id);
    }

    pub fn spawn_random_piece(&mut self) -> bool {
        self.board.spawn_random_piece()
    }

    pub fn board_to_string(&self) -> String {
        self.board.to_string()
    }

    pub fn has_active_piece(&mut self) -> bool {
        self.board.has_active_piece()
    }

    pub fn step(&mut self) {
        self.board.step();
    }

    pub fn board_move_active_piece_down_and_try_sleep(&mut self) -> bool {
        self.board.move_active_piece_down_and_try_sleep()
    }

    pub fn clear_lines(&mut self) -> u32 {
        let cleared_lines = self.board.clear_lines();
        self.lines_cleared += cleared_lines;
        self.points += cleared_lines * 4;
        cleared_lines
    }

    pub fn demo_random(&mut self) {
        let mut rng = rand::thread_rng();

        self.set_board_size(10, 18);

        loop {
            if !self.board.spawn_random_piece() {
                println!("Game over");
                break;
            }

            let direction = rng.gen_range(0, 2);
            let amount = rng.gen_range(0, 5);
            let rotation = rng.gen_range(0, 4);

            for _ in 0..rotation {
                self.board.rotate_active_piece_right();
            }

            for _ in 0..amount {
                let moved = match direction {
                    0 => self.board.move_active_piece_left(),
                    1 => self.board.move_active_piece_right(),
                    _ => unreachable!(),
                };

                if !moved {
                    break;
                }
            }

            while {
                let moved = self.board.move_active_piece_down_and_try_sleep();
                moved
            } {}
        }
    }
}
