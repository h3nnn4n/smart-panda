#![allow(dead_code)]

use gamestate::board;
use wasm_bindgen::prelude::*;

use gamestate::feature_functions::get_column_height;
use gamestate::GameState;

pub fn delta_height(board: &board::Board) -> u32 {
    let mut min_height = get_column_height(board, 0);
    let mut max_height = get_column_height(board, 0);

    for i in 1..board.get_width() {
        let column_height = get_column_height(board, i);

        min_height = std::cmp::min(min_height, column_height);
        max_height = std::cmp::max(max_height, column_height);
    }

    max_height - min_height
}

#[wasm_bindgen]
impl GameState {
    pub fn get_delta_height(&self) -> u32 {
        delta_height(&self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_height_zero() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        assert_eq!(0, delta_height(&board));
    }

    #[test]
    fn delta_height_with_floating_o_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(2);

        assert_eq!(0, delta_height(&board));
    }

    #[test]
    fn delta_height_with_floating_and_placed_o_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(2);
        while { board.move_active_piece_down_and_try_sleep() } {}

        board.spawn_piece(2);

        assert_eq!(2, delta_height(&board));
    }

    #[test]
    fn delta_height_with_i_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(1);

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(1, delta_height(&board));
    }

    #[test]
    fn delta_height_with_i_piece_rotated() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(1);
        board.rotate_active_piece_right();

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(4, delta_height(&board));
    }
}
