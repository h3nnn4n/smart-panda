#![allow(dead_code)]
use wasm_bindgen::prelude::*;

mod board;
mod feature_functions;
mod piece;

#[wasm_bindgen]
pub struct GameState {
    points: u32,
    lines_cleared: u32,
    game_over: bool,
    board: board::Board,
}

#[wasm_bindgen]
impl GameState {
    pub fn new() -> GameState {
        let bb = board::Board::new();

        GameState {
            points: 0,
            lines_cleared: 0,
            game_over: false,
            board: bb,
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn set_game_over(&mut self, game_over: bool) {
        self.game_over = game_over;
    }

    pub fn get_points(&mut self) -> u32 {
        self.points
    }

    pub fn get_lines_cleared(&mut self) -> u32 {
        self.lines_cleared
    }

    pub fn get_width(&self) -> u32 {
        self.board.get_width()
    }

    pub fn get_height(&self) -> u32 {
        self.board.get_height()
    }

    pub fn get_board_pointer(&self) -> *const u32 {
        self.board.get_board_pointer()
    }

    pub fn set_board(&mut self, board: &mut [u32]) {
        self.board.set_board(board);
    }

    pub fn get_active_piece(&self) -> piece::Piece {
        self.board.get_active_piece()
    }

    pub fn build_active_piece(&mut self, id: u32, x: u32, y: u32, rotation: u32) {
        self.board.build_active_piece(id, x, y, rotation);
    }

    pub fn set_board_size(&mut self, width: u32, height: u32) {
        self.board.set_board_size(width, height);
    }

    pub fn reset(&mut self) {
        self.points = 0;
        self.lines_cleared = 0;
        self.board.reset();
    }

    pub fn spawn_piece(&mut self, id: u32) -> bool {
        self.board.spawn_piece(id)
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

    pub fn move_active_piece_down_and_try_sleep(&mut self) -> bool {
        self.board.move_active_piece_down_and_try_sleep()
    }

    pub fn move_active_piece_left(&mut self) -> bool {
        self.board.move_active_piece_left()
    }

    pub fn move_active_piece_right(&mut self) -> bool {
        self.board.move_active_piece_right()
    }

    pub fn rotate_active_piece_left(&mut self) -> bool {
        self.board.rotate_active_piece_left()
    }

    pub fn rotate_active_piece_right(&mut self) -> bool {
        self.board.rotate_active_piece_right()
    }

    pub fn clear_lines(&mut self) -> u32 {
        let cleared_lines = self.board.clear_lines();
        self.lines_cleared += cleared_lines;
        self.points += cleared_lines * 4;
        cleared_lines
    }
}
