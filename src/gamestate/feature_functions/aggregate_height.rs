#![allow(dead_code)]
use gamestate::board;
use wasm_bindgen::prelude::*;

use gamestate::feature_functions::get_column_height;
use gamestate::GameState;

pub fn aggregate_height(board: &board::Board) -> u32 {
    let mut total_aggregate_height = 0;

    for i in 0..board.get_width() {
        total_aggregate_height += get_column_height(board, i);
    }

    total_aggregate_height
}

#[wasm_bindgen]
impl GameState {
    pub fn get_aggregate_height(&self) -> u32 {
        aggregate_height(&self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregate_height_zero() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        assert_eq!(0, aggregate_height(&board));
    }

    #[test]
    fn aggregate_height_with_floating_o_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(2);

        assert_eq!(0, aggregate_height(&board));
    }

    #[test]
    fn aggregate_height_with_floating_and_placed_o_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(2);
        while { board.move_active_piece_down_and_try_sleep() } {}

        board.spawn_piece(2);

        assert_eq!(4, aggregate_height(&board));
    }

    #[test]
    fn aggregate_height_with_o_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(2);

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(4, aggregate_height(&board));
    }

    #[test]
    fn aggregate_height_with_multiple_o_pieces() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(2);
        while { board.move_active_piece_down_and_try_sleep() } {}

        board.spawn_piece(2);
        while { board.move_active_piece_down_and_try_sleep() } {}

        board.spawn_piece(2);
        board.move_active_piece_left();
        board.move_active_piece_left();
        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(12, aggregate_height(&board));
    }

    #[test]
    fn aggregate_height_with_i_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(1);

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(4, aggregate_height(&board));
    }

    #[test]
    fn aggregate_height_with_i_piece_rotated() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(1);
        board.rotate_active_piece_right();

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(4, aggregate_height(&board));
    }

    #[test]
    fn aggregate_height_with_z_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(5);

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(5, aggregate_height(&board));
    }
    #[test]

    fn aggregate_height_with_z_piece_rotated() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(5);
        board.rotate_active_piece_right();

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(5, aggregate_height(&board));
    }

    #[test]
    fn aggregate_height_with_l_piece() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(3);

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(6, aggregate_height(&board));
    }
    #[test]

    fn aggregate_height_with_l_piece_rotated() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(3);
        board.rotate_active_piece_right();

        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(4, aggregate_height(&board));
    }
}
