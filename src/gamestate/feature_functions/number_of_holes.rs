#![allow(dead_code)]

use gamestate::board;
use wasm_bindgen::prelude::*;

use gamestate::GameState;

pub fn number_of_holes(board: &board::Board) -> u32 {
    let mut total_number_of_holes = 0;

    for x in 1..board.get_width() {
        let mut found_some = false;

        for y in 0..board.get_height() {
            let block_id = board.get_block(x, y);

            if block_id > 0 && block_id < 100 {
                found_some = true;
            }

            if found_some && block_id == 0 {
                total_number_of_holes += 1;
            }
        }
    }

    total_number_of_holes
}

#[wasm_bindgen]
impl GameState {
    pub fn get_number_of_holes(&self) -> u32 {
        number_of_holes(&self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_number_of_holes() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        assert_eq!(0, number_of_holes(&board));
    }
}
