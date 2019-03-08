#![allow(dead_code)]
use gamestate::piece;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,
    board: Vec<u32>,
    active_piece: Option<piece::Piece>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for j in 0..self.height {
            for i in 0..self.width {
                let symbol = if self.get_block(i, j) == 0 {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        Board {
            width: 0,
            height: 0,
            board: Vec::new(),
            active_piece: None,
        }
    }

    pub fn set_board_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.board = Vec::new();

        for _ in 0..(width * height) {
            self.board.push(0);
        }

        assert_eq!(self.board.len() as u32, width * height);
    }

    pub fn reset(&mut self) {
        let width = self.width;
        let height = self.height;

        self.board.clear();
        self.active_piece = None;
        self.set_board_size(width, height);
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_board_pointer(&self) -> *const u32 {
        self.board.as_ptr()
    }

    pub fn set_board(&mut self, board: &mut [u32]) {
        let width = self.width;
        let height = self.height;

        for i in 0..width {
            for j in 0..height {
                let index = (i * height + j) as usize;
                let id: u32 = board[index];

                self.place_block(i, j, id, false);
            }
        }
    }

    pub fn spawn_random_piece(&mut self) -> bool {
        let new_piece = piece::Piece::new_random();
        self.active_piece = Some(new_piece);
        if self.can_active_piece_be_spawned() {
            self.update();
            true
        } else {
            false
        }
    }

    pub fn spawn_and_place_piece(&mut self, id: u32, x: u32, y: u32) {
        let new_piece = piece::Piece::spawn_and_place_piece(id, x, y);
        self.active_piece = Some(new_piece);
        self.sleep_active_piece();
    }

    pub fn spawn_piece(&mut self, id: u32) {
        let new_piece = piece::Piece::spawn_piece(id);
        self.active_piece = Some(new_piece);
        self.update();
    }

    pub fn step(&mut self) {
        match self.active_piece {
            None => (),
            Some(_) => {
                if self.can_active_piece_move_down() {
                    self.move_active_piece_down();
                } else {
                    self.sleep_active_piece();
                }
            }
        }
    }

    pub fn has_active_piece(&self) -> bool {
        self.active_piece.is_some()
    }

    pub fn sleep_active_piece(&mut self) {
        if self.has_active_piece() {
            self.place_active_piece_and_sleep();
            self.active_piece = None;
        }
    }

    fn can_active_piece_rotate_left(&self) -> bool {
        if let None = self.active_piece {
            return false;
        }

        true // FIXME
    }

    fn can_active_piece_rotate_right(&self) -> bool {
        if let None = self.active_piece {
            return false;
        }

        true // FIXME
    }

    pub fn rotate_active_piece_left(&mut self) -> bool {
        if !self.can_active_piece_rotate_left() {
            return false;
        }

        if let Some(ref mut active_piece) = self.active_piece {
            active_piece.rotate_left();
        }

        self.update();

        true
    }

    pub fn rotate_active_piece_right(&mut self) -> bool {
        if !self.can_active_piece_rotate_right() {
            return false;
        }

        if let Some(ref mut active_piece) = self.active_piece {
            active_piece.rotate_right();
        }

        self.update();

        true
    }

    pub fn move_active_piece_right(&mut self) -> bool {
        if !self.can_active_piece_move_right() {
            return false;
        }

        if let Some(ref mut active_piece) = self.active_piece {
            active_piece.move_right();
        }

        self.update();

        true
    }

    pub fn move_active_piece_left(&mut self) -> bool {
        if !self.can_active_piece_move_left() {
            return false;
        }

        if let Some(ref mut active_piece) = self.active_piece {
            active_piece.move_left();
        }

        self.update();

        true
    }

    pub fn move_active_piece_down(&mut self) -> bool {
        if !self.can_active_piece_move_down() {
            return false;
        }

        if let Some(ref mut active_piece) = self.active_piece {
            active_piece.move_down();
        }

        self.update();

        true
    }

    pub fn move_active_piece_down_and_try_sleep(&mut self) -> bool {
        if !self.can_active_piece_move_down() {
            if self.has_active_piece() {
                self.sleep_active_piece()
            }
            return false;
        }

        if let Some(ref mut active_piece) = self.active_piece {
            active_piece.move_down();
        }

        self.update();

        true
    }

    pub fn clearable_lines(&self) -> u32 {
        let mut clearable = 0_u32;
        for i in 0..self.height {
            let mut can_clear = true;
            for j in 0..self.width {
                if self.get_block(j, i) == 0 {
                    can_clear = false;
                    break;
                }
            }

            if can_clear {
                clearable += 1;
            }
        }

        clearable
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut cleared = 0_u32;
        for i in 0..self.height - 0 {
            let mut can_clear = true;
            for j in 0..self.width {
                if self.get_block(j, i) == 0 {
                    can_clear = false;
                    break;
                }
            }

            if can_clear {
                for j in i..self.height {
                    cleared += 1;
                    for x in 0..self.width {
                        let w = self.get_block(x, j - 1);
                        self.place_block(x, j, w, false);
                        self.place_block(x, j - 1, 0, false);
                    }
                }
            }
        }

        cleared
    }

    pub fn place_o_piece(&mut self, x: u32, y: u32) -> bool {
        self.spawn_and_place_piece(2, x, y);

        true
    }

    fn can_active_piece_be_spawned(&self) -> bool {
        if let None = self.active_piece {
            return false;
        }

        if let Some(ref active_piece) = self.active_piece {
            for j in 0..4 {
                let body = active_piece.get_body()[j];
                let (x_, y_) = (body[0], body[1]);
                let (x, y) = (
                    x_ + active_piece.get_x() as i32,
                    y_ + active_piece.get_y() as i32,
                );

                let piece_piece = self.get_block(x as u32, y as u32);

                if piece_piece > 0 {
                    return false;
                }
            }
        }

        true
    }

    fn can_active_piece_move_right(&self) -> bool {
        if let None = self.active_piece {
            return false;
        }

        let x_limit = self.width;

        if let Some(ref active_piece) = self.active_piece {
            for j in 0..4 {
                let body = active_piece.get_body()[j];
                let (x_, y_) = (body[0], body[1]);
                let (x, y) = (
                    x_ + active_piece.get_x() as i32,
                    y_ + active_piece.get_y() as i32,
                );

                if y < 0 {
                    continue;
                }

                if x + 1 >= x_limit as i32 {
                    return false;
                }

                let piece_piece = self.get_block((x + 1) as u32, y as u32);
                if piece_piece > 0 && piece_piece < 127 {
                    return false;
                }
            }
        }

        true
    }

    fn can_active_piece_move_left(&self) -> bool {
        if let None = self.active_piece {
            return false;
        }

        let x_limit = 0;

        if let Some(ref active_piece) = self.active_piece {
            for j in 0..4 {
                let body = active_piece.get_body()[j];
                let (x_, y_) = (body[0], body[1]);
                let (x, y) = (
                    x_ + active_piece.get_x() as i32,
                    y_ + active_piece.get_y() as i32,
                );

                if y < 0 {
                    continue;
                }

                if x <= x_limit as i32 {
                    return false;
                }

                let piece_piece = self.get_block((x - 1) as u32, y as u32);
                if piece_piece > 0 && piece_piece < 127 {
                    return false;
                }
            }
        }

        true
    }

    fn can_active_piece_move_down(&self) -> bool {
        if let None = self.active_piece {
            return false;
        }

        let y_limit = self.height;

        if let Some(ref active_piece) = self.active_piece {
            for j in 0..4 {
                let body = active_piece.get_body()[j];
                let (x_, y_) = (body[0], body[1]);
                let (x, y) = (
                    x_ + active_piece.get_x() as i32,
                    y_ + active_piece.get_y() as i32,
                );

                if y < 0 {
                    continue;
                }

                if y + 1 >= y_limit as i32 {
                    return false;
                }

                let piece_piece = self.get_block(x as u32, (y + 1) as u32);
                if piece_piece > 0 && piece_piece < 127 {
                    return false;
                }
            }
        }

        true
    }

    fn remove_active_piece(&mut self) -> u32 {
        match self.active_piece {
            Some(_) => {
                let mut blocks_removed = 0;
                for x in 0..self.width {
                    for y in 0..self.height {
                        if self.get_block(x, y) > 126 {
                            self.remove_block(x, y);
                            blocks_removed += 1;
                        }
                    }
                }

                //assert_eq!(4, blocks_removed);

                blocks_removed
            }
            None => 0,
        }
    }

    fn place_active_piece_and_sleep(&mut self) -> bool {
        if let None = self.active_piece {
            return false;
        }

        let mut moves: [(i32, i32); 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
        let mut piece_id = 0;

        if let Some(ref active_piece) = self.active_piece {
            piece_id = active_piece.get_id();

            for j in 0..4 {
                let body = active_piece.get_body()[j];
                let (x_, y_) = (body[0], body[1]);
                let (x, y) = (
                    x_ + active_piece.get_x() as i32,
                    y_ + active_piece.get_y() as i32,
                );

                moves[j] = (x, y);
            }
        }

        for i in 0..4 {
            let (x, y) = moves[i];
            self.place_block(x as u32, y as u32, piece_id, false);
        }

        true
    }

    fn place_active_piece(&mut self) -> bool {
        if let None = self.active_piece {
            return false;
        }

        let mut moves: [(i32, i32); 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
        let mut piece_id = 0;

        if let Some(ref active_piece) = self.active_piece {
            piece_id = active_piece.get_id();

            for j in 0..4 {
                let body = active_piece.get_body()[j];
                let (x_, y_) = (body[0], body[1]);
                let (x, y) = (
                    x_ + active_piece.get_x() as i32,
                    y_ + active_piece.get_y() as i32,
                );

                moves[j] = (x, y);
            }
        }

        for i in 0..4 {
            let (x, y) = moves[i];
            self.place_block(x as u32, y as u32, piece_id, true);
        }

        true
    }

    fn update_active_piece(&mut self) {
        self.remove_active_piece();
        self.place_active_piece();
    }

    fn update(&mut self) {
        self.update_active_piece();
    }

    fn clear_board(&mut self) {
        assert_eq!(self.board.len() as u32, self.width * self.height);

        for i in 0..self.width {
            for j in 0..self.height {
                self.place_block(i, j, 0, false);
            }
        }
    }

    fn place_block(&mut self, x: u32, y: u32, id: u32, active: bool) {
        if y > self.height {
            return;
        }

        let index = (x * self.height + y) as usize;

        self.board[index] = id;

        if active {
            self.board[index] += 127;
        }
    }

    fn remove_block(&mut self, x: u32, y: u32) {
        let index = (x * self.height + y) as usize;

        self.board[index] = 0;
    }

    pub fn get_block(&self, x: u32, y: u32) -> u32 {
        let index = (x * self.height + y) as usize;

        self.board[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn count_active_blocks(board: &Board) -> u32 {
        let mut count = 0;
        for x in 0..10 {
            for y in 0..18 {
                if board.get_block(x, y) > 0 {
                    count += 1;
                }
            }
        }

        count
    }

    #[test]
    fn new() {
        let board = Board::new();

        assert_eq!(0, board.width);
        assert_eq!(0, board.height);
        assert_eq!(0, board.board.len());
        assert!(board.active_piece.is_none());
    }

    #[test]
    fn set_board_size() {
        let mut board = Board::new();

        assert_eq!(0, board.width);
        assert_eq!(0, board.height);

        board.set_board_size(10, 18);

        assert_eq!(10, board.width);
        assert_eq!(18, board.height);

        assert_eq!(180, board.board.len());
        assert!(board.active_piece.is_none());
    }

    #[test]
    fn place_block() {
        let mut board = Board::new();

        board.set_board_size(10, 18);
        assert_eq!(0, count_active_blocks(&board));

        board.place_block(1, 1, 1, false);
        assert_eq!(1, count_active_blocks(&board));

        board.place_block(2, 1, 1, false);
        assert_eq!(2, count_active_blocks(&board));

        board.place_block(2, 2, 1, false);
        assert_eq!(3, count_active_blocks(&board));

        board.place_block(1, 2, 1, false);
        assert_eq!(4, count_active_blocks(&board));
    }

    #[test]
    fn get_block() {
        let mut board = Board::new();

        board.set_board_size(10, 18);

        board.place_o_piece(6, 6);

        assert!(board.get_block(6, 6) > 0);
        assert!(board.get_block(6, 5) > 0);
        assert!(board.get_block(5, 6) > 0);
        assert!(board.get_block(5, 5) > 0);

        assert!(board.get_block(6, 7) == 0);
        assert!(board.get_block(7, 6) == 0);
        assert!(board.get_block(7, 7) == 0);
    }

    #[test]
    fn remove_block() {
        let mut board = Board::new();

        board.set_board_size(10, 18);

        board.place_o_piece(1, 17);
        board.place_o_piece(3, 17);

        assert_eq!(8, count_active_blocks(&board));

        board.remove_block(1, 17);
        assert_eq!(7, count_active_blocks(&board));

        board.remove_block(0, 17);
        assert_eq!(6, count_active_blocks(&board));

        board.remove_block(1, 16);
        assert_eq!(5, count_active_blocks(&board));

        board.remove_block(0, 16);
        assert_eq!(4, count_active_blocks(&board));
    }

    #[test]
    fn spawn_and_place_piece() {
        let mut board = Board::new();

        board.set_board_size(10, 18);
        assert_eq!(0, count_active_blocks(&board));

        board.place_o_piece(1, 17);
        assert_eq!(4, count_active_blocks(&board));

        board.place_o_piece(3, 17);
        assert_eq!(8, count_active_blocks(&board));

        board.place_o_piece(5, 17);
        assert_eq!(12, count_active_blocks(&board));
    }

    #[test]
    fn clear_board() {
        let mut board = Board::new();

        board.set_board_size(10, 18);

        board.place_o_piece(1, 17);
        board.place_o_piece(3, 17);
        board.place_o_piece(5, 17);
        board.place_o_piece(7, 17);
        board.place_o_piece(9, 17);

        assert_eq!(20, count_active_blocks(&board));

        board.clear_board();

        assert_eq!(0, count_active_blocks(&board));
        assert_eq!(10, board.width);
        assert_eq!(18, board.height);
    }

    #[test]
    fn has_active_piece() {
        let mut board = Board::new();

        board.set_board_size(10, 18);

        assert!(board.has_active_piece() == false);
        board.spawn_random_piece();
        assert!(board.has_active_piece());

        while { board.move_active_piece_down() } {}

        board.sleep_active_piece();
        assert!(board.has_active_piece() == false);
    }

    #[test]
    fn reset() {
        let mut board = Board::new();

        board.set_board_size(10, 18);

        board.place_o_piece(1, 17);
        board.place_o_piece(3, 17);
        board.spawn_random_piece();
        assert_eq!(12, count_active_blocks(&board));
        assert!(board.active_piece.is_some());

        board.reset();
        assert!(board.active_piece.is_none());
        assert_eq!(10 * 18, board.board.len());
        assert_eq!(18, board.height);
        assert_eq!(10, board.width);
    }

    #[test]
    fn move_active_piece_down_and_try_sleep() {
        let mut board = Board::new();

        board.set_board_size(10, 18);

        for i in 0..4 {
            board.spawn_random_piece();

            while { board.move_active_piece_down_and_try_sleep() } {}

            assert!(!board.has_active_piece());

            assert_eq!((i + 1) * 4, count_active_blocks(&board));
        }

        let mut found_some_piece = false;

        for x in 0..10 {
            if board.get_block(x, 17) > 0 {
                found_some_piece = true;
                break;
            }
        }

        assert!(found_some_piece);
    }

    #[test]
    fn can_all_pieces_be_spawned() {
        for i in 1..8 {
            let mut board = Board::new();
            board.set_board_size(10, 18);

            board.spawn_piece(i);
        }
    }

    #[test]
    fn clearable_lines() {
        let mut board = Board::new();
        board.set_board_size(10, 18);

        assert_eq!(0, board.clearable_lines());

        board.place_o_piece(1, 17);
        board.place_o_piece(3, 17);
        board.place_o_piece(5, 17);
        board.place_o_piece(7, 17);
        assert_eq!(0, board.clearable_lines());
        board.place_o_piece(9, 17);

        assert_eq!(2, board.clearable_lines());

        board.place_o_piece(1, 15);
        board.place_o_piece(3, 15);
        board.place_o_piece(5, 15);
        board.place_o_piece(7, 15);
        assert_eq!(2, board.clearable_lines());
        board.place_o_piece(9, 15);

        assert_eq!(4, board.clearable_lines());
    }

    #[test]
    fn clear_lines() {
        let mut board = Board::new();
        board.set_board_size(10, 18);

        assert_eq!(0, board.clear_lines());

        board.place_o_piece(1, 17);
        board.place_o_piece(3, 17);
        board.place_o_piece(5, 17);
        board.place_o_piece(7, 17);
        board.place_o_piece(9, 17);

        board.place_o_piece(1, 15);
        board.place_o_piece(3, 15);
        board.place_o_piece(5, 15);
        board.place_o_piece(7, 15);
        board.place_o_piece(9, 15);

        assert_eq!(4, board.clearable_lines());
        assert_eq!(4, board.clear_lines());
        assert_eq!(0, board.clearable_lines());

        board.update();

        assert_eq!(0, board.clearable_lines());

        board.place_o_piece(1, 17);
        board.place_o_piece(3, 17);
        board.place_o_piece(5, 17);
        board.place_o_piece(7, 17);
        board.place_o_piece(9, 17);

        assert_eq!(2, board.clearable_lines());
        assert_eq!(2, board.clear_lines());
        assert_eq!(0, board.clearable_lines());

        board.update();

        assert_eq!(0, board.clearable_lines());
    }
}
