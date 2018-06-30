use board;
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
        cleared_lines
    }

    pub fn update_view(&self) {
        print!("{}[2J", 27 as char);
        self.print_board();
        thread::sleep(time::Duration::from_millis(80));
        //thread::sleep(time::Duration::from_millis(160));
    }
}
