#![allow(dead_code)]

extern crate rand;

use board;
use rand::Rng;
use std::{thread, time};

pub struct GameState {
    points: u32,
    lines_cleared: u32,
    pub board: board::Board,
}

impl GameState {
    pub fn new() -> GameState {
        let bb = board::Board::new();

        GameState {
            points: 0,
            lines_cleared: 0,
            board: bb,
        }
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

    pub fn spawn_random_piece(&mut self) {
        self.board.spawn_random_piece();
    }

    pub fn print_board(&self) {
        self.board.print_board()
    }

    pub fn has_active_piece(&mut self) -> bool {
        self.board.has_active_piece()
    }

    pub fn step(&mut self) {
        self.board.step();
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
                self.update_view();
            }

            for _ in 0..amount {
                let moved = match direction {
                    0 => self.board.move_active_piece_left(),
                    1 => self.board.move_active_piece_right(),
                    _ => unreachable!(),
                };

                self.update_view();

                if !moved {
                    break;
                }
            }

            while {
                let moved = self.board.move_active_piece_down_and_try_sleep();
                self.update_view();
                moved
            } {}
        }
    }

    pub fn update_view(&self) {
        print!("{}[2J", 27 as char);
        self.print_board();
        thread::sleep(time::Duration::from_millis(80));
        //thread::sleep(time::Duration::from_millis(160));
    }
}
