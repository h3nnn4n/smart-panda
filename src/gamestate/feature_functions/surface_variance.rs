#![allow(dead_code)]

use gamestate::board;
use wasm_bindgen::prelude::*;

use gamestate::feature_functions::get_column_height;
use gamestate::GameState;

pub fn surface_variance(board: &board::Board) -> u32 {
    let mut total_surface_variance = 0;

    let mut previous_height = get_column_height(board, 0);

    for i in 1..board.get_width() {
        let column_height = get_column_height(board, i);
        total_surface_variance += (previous_height as i32 - column_height as i32).abs() as u32;
        previous_height = column_height;
    }

    total_surface_variance
}

#[wasm_bindgen]
impl GameState {
    pub fn get_surface_variance(&self) -> u32 {
        surface_variance(&self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_variance_zero() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        assert_eq!(0, surface_variance(&board));
    }

    #[test]
    fn surface_variance_with_floating_o_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(2);

        assert_eq!(0, surface_variance(&board));
    }

    #[test]
    fn surface_variance_with_floating_and_placed_o_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(2);
        while { board.move_active_piece_down_and_try_sleep() } {}

        board.spawn_piece(2);

        assert_eq!(4, surface_variance(&board));
    }

    #[test]
    fn surface_variance_with_i_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(1);

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(2, surface_variance(&board));
    }

    #[test]
    fn surface_variance_with_i_piece_rotated() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(1);
        board.rotate_active_piece_right();

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(8, surface_variance(&board));
    }
}
