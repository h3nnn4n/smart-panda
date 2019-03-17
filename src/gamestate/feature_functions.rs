#![allow(dead_code)]

mod aggregate_height;
mod delta_height;
mod number_of_holes;
mod surface_variance;

use gamestate::board;

pub fn get_column_height(board: &board::Board, column: u32) -> u32 {
    for j in 0..board.get_height() {
        let block = board.get_block(column, j);

        if is_static(block) {
            return board.get_height() - j;
        }
    }

    0
}

pub fn is_static(block: u32) -> bool {
    block != 0 && block < 10
}
