#![allow(dead_code)]

use gamestate::board;
use wasm_bindgen::prelude::*;

use gamestate::GameState;

// Measures how many cells are hidden bellow a hole.
// It works cumulatively. e.g. a hole at height 3 covers 2 cells,
// A hole at height 6 covers 5 cells. If there is a hole at height
// 3 and 6 then the hole at 3 covers 2 cells and the one at 6
// covers 4 cells (there is a hole there also). This amounts to
// a total of 4 + 2 for that column

pub fn covered_cells(board: &board::Board) -> u32 {
    let mut covered_cells_count = 0;

    for x in 1..board.get_width() {
        let mut found_some = false;

        for y in 0..board.get_height() {
            let block_id = board.get_block(x, y);

            if block_id > 0 && block_id < 100 {
                found_some = true;
            }

            if found_some && block_id == 0 {
                covered_cells_count += number_of_blocks_bellow(board, x, y);
            }
        }
    }

    covered_cells_count
}

fn number_of_blocks_bellow(board: &board::Board, column: u32, start_y: u32) -> u32 {
    let mut covered_cells = 0;

    for y in (start_y + 1)..board.get_height() {
        let block_id = board.get_block(column, y);

        if block_id > 0 && block_id < 100 {
            covered_cells += 1;
        }
    }

    covered_cells
}

#[wasm_bindgen]
impl GameState {
    pub fn get_covered_cells(&self) -> u32 {
        covered_cells(&self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covered_cells_zero() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        assert_eq!(0, covered_cells(&board));
    }

    #[test]
    fn covered_cells_dont_count_active_pieces() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(2);

        assert_eq!(0, covered_cells(&board));
    }

    #[test]
    fn zero_covered_cells() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        board.spawn_piece(7);
        board.rotate_active_piece_right();
        board.rotate_active_piece_right();
        while { board.move_active_piece_down_and_try_sleep() } {}

        assert_eq!(0, covered_cells(&board));
    }

    #[test]
    fn two_covered_cells() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        for _ in 0..2 {
            board.spawn_piece(7);
            while { board.move_active_piece_down_and_try_sleep() } {}
        }

        println!("{}", board);

        assert_eq!(2, covered_cells(&board));
    }

    #[test]
    fn six_covered_cells() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        for _ in 0..3 {
            board.spawn_piece(7);
            while { board.move_active_piece_down_and_try_sleep() } {}
        }

        println!("{}", board);

        assert_eq!(6, covered_cells(&board));
    }
    #[test]
    fn twelve_covered_cells() {
        let mut board = board::Board::new();
        board.set_board_size(10, 18);

        for _ in 0..4 {
            board.spawn_piece(7);
            while { board.move_active_piece_down_and_try_sleep() } {}
        }

        println!("{}", board);

        assert_eq!(12, covered_cells(&board));
    }
}
