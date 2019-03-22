#![allow(dead_code)]

use gamestate::board;
use wasm_bindgen::prelude::*;

use gamestate::GameState;

pub fn vertical_roughness(board: &board::Board) -> u32 {
    let mut total_vertical_roughness = 0;

    for x in 0..board.get_width() {
        let mut last_block = board.get_block(x, 0);

        if last_block > 127 {
            last_block = 0;
        }

        for y in 1..board.get_height() {
            let block_id = board.get_block(x, y);

            if block_id > 127 {
                continue;
            }

            if last_block != block_id {
                total_vertical_roughness += 1;
            }

            last_block = block_id;
        }
    }

    total_vertical_roughness
}

#[wasm_bindgen]
impl GameState {
    pub fn get_vertical_roughness(&self) -> u32 {
        vertical_roughness(&self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_roughness_zero() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        assert_eq!(0, vertical_roughness(&board));
    }

    #[test]
    fn vertical_roughness_dont_count_active_pieces() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(2);

        assert_eq!(0, vertical_roughness(&board));
    }

    #[test]
    fn three_vertical_roughness() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(7);
        board.rotate_active_piece_right();
        board.rotate_active_piece_right();
        while { board.move_active_piece_down_and_try_sleep() } {}

        println!("{}", board);

        assert_eq!(3, vertical_roughness(&board));
    }

    #[test]
    fn five_vertical_roughness() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(7);
        while { board.move_active_piece_down_and_try_sleep() } {}

        println!("{}", board);

        assert_eq!(5, vertical_roughness(&board));
    }

    #[test]
    fn nine_vertical_roughness() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        for _ in 0..2 {
            board.spawn_piece(7);
            while { board.move_active_piece_down_and_try_sleep() } {}
        }

        println!("{}", board);

        assert_eq!(9, vertical_roughness(&board));
    }
}
