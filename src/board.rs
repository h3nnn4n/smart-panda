use piece;

pub struct Board {
    width: u32,
    height: u32,
    board: Vec<u32>,
    pieces: Vec<piece::Piece>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            width: 0,
            height: 0,
            board: Vec::new(),
            pieces: Vec::new(),
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
        self.board.clear();
        self.pieces.clear();
    }

    pub fn spawn_random_piece(&mut self) {
        let new_piece = piece::Piece::new_random();
        self.pieces.push(new_piece);
        self.update();
    }

    pub fn print_board(&self) {
        print!("      +");
        for i in 0..self.width {
            print!("-");
        }
        println!("+ ");

        for j in 0..self.height {
            print!("{:4}  |", j + 1);
            for i in 0..self.width {
                let c = match self.get_block(i, j) {
                    0 => ' ',
                    1...126 => 'O',
                    _ => 'X',
                };

                print!("{}", c);
            }

            println!("|");
        }

        print!("      +");
        for i in 0..self.width {
            print!("-");
        }
        println!("+ ");
    }

    pub fn spawn_and_place_piece(&mut self, id: u32, x: u32, y: u32) {
        let new_piece = piece::Piece::spawn_and_place_piece(id, x, y);
        self.pieces.push(new_piece);
        self.sleep_active_piece();
    }

    pub fn spawn_piece(&mut self, id: u32) {
        let new_piece = piece::Piece::spawn_piece(id);
        self.pieces.push(new_piece);
        self.update();
    }

    pub fn step(&mut self) {
        let active = self.get_active_piece_index();

        match active {
            None => (),
            Some(i) => {
                if self.can_active_piece_move_down(i) {
                    self.move_active_piece_down();
                } else {
                    self.sleep_active_piece();
                    self.update();
                }
            }
        }
    }

    pub fn has_active_piece(&mut self) -> bool {
        let active = self.get_active_piece_index();

        match active {
            Some(_) => true,
            None => false,
        }
    }

    fn sleep_active_piece(&mut self) {
        if self.has_active_piece() {
            self.place_active_piece_and_sleep();
            self.pieces.pop();
        }
    }

    fn can_active_piece_rotate_left(&self, i: usize) -> bool {
        true // FIXME
    }

    fn can_active_piece_rotate_right(&self, i: usize) -> bool {
        true // FIXME
    }

    pub fn rotate_active_piece_left(&mut self) -> bool {
        let active = self.get_active_piece_index();

        match active {
            None => return false,
            Some(i) => {
                if self.can_active_piece_rotate_left(i) {
                    self.pieces[i].rotate_left();
                    self.update();
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn rotate_active_piece_right(&mut self) -> bool {
        let active = self.get_active_piece_index();

        match active {
            None => return false,
            Some(i) => {
                if self.can_active_piece_rotate_right(i) {
                    self.pieces[i].rotate_right();
                    self.update();
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn move_active_piece_right(&mut self) -> bool {
        let active = self.get_active_piece_index();

        match active {
            None => return false,
            Some(i) => {
                if self.can_active_piece_move_right(i) {
                    self.pieces[i].move_right();
                    self.update();
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn move_active_piece_left(&mut self) -> bool {
        let active = self.get_active_piece_index();

        match active {
            None => return false,
            Some(i) => {
                if self.can_active_piece_move_left(i) {
                    self.pieces[i].move_left();
                    self.update();
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn move_active_piece_down(&mut self) -> bool {
        let active = self.get_active_piece_index();

        match active {
            None => return false,
            Some(i) => {
                if self.can_active_piece_move_down(i) {
                    self.pieces[i].move_down();
                    self.update();
                    true
                } else {
                    self.sleep_active_piece();
                    false
                }
            }
        }
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

    fn can_active_piece_move_right(&self, i: usize) -> bool {
        let x_limit = self.width;

        for j in 0..4 {
            let (x_, y_) = self.pieces[i].get_body()[j];
            let (x, y) = (
                x_ + self.pieces[i].get_x() as i32,
                y_ + self.pieces[i].get_y() as i32,
            );

            if x + 1 >= x_limit as i32 {
                return false;
            }

            let piece_id = self.pieces[i].get_id();
            let is_active = self.pieces[i].is_active();

            let piece_piece = self.get_block((x + 1) as u32, x as u32);
            if piece_piece > 0 && piece_piece < 127 {
                return false;
            }
        }

        true
    }

    fn can_active_piece_move_left(&self, i: usize) -> bool {
        let x_limit = 0;

        for j in 0..4 {
            let (x_, y_) = self.pieces[i].get_body()[j];
            let (x, y) = (
                x_ + self.pieces[i].get_x() as i32,
                y_ + self.pieces[i].get_y() as i32,
            );

            if x <= x_limit as i32 {
                return false;
            }

            let piece_id = self.pieces[i].get_id();
            let is_active = self.pieces[i].is_active();

            let piece_piece = self.get_block((x - 1) as u32, x as u32);
            if piece_piece > 0 && piece_piece < 127 {
                return false;
            }
        }

        true
    }

    fn can_active_piece_move_down(&self, i: usize) -> bool {
        let y_limit = self.height;

        for j in 0..4 {
            let (x_, y_) = self.pieces[i].get_body()[j];
            let (x, y) = (
                x_ + self.pieces[i].get_x() as i32,
                y_ + self.pieces[i].get_y() as i32,
            );

            if y + 1 >= y_limit as i32 {
                return false;
            }

            let piece_id = self.pieces[i].get_id();
            let is_active = self.pieces[i].is_active();

            let piece_piece = self.get_block(x as u32, (y + 1) as u32);
            if piece_piece > 0 && piece_piece < 127 {
                return false;
            }
        }

        true
    }

    fn get_active_piece_index(&self) -> Option<usize> {
        if self.pieces.len() == 0 {
            return None;
        }

        for i in 0..self.pieces.len() {
            if self.pieces[i].is_active() {
                return Some(i);
            }
        }

        return None;
    }

    fn remove_active_piece(&mut self) -> u32 {
        match self.get_active_piece_index() {
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
        match self.get_active_piece_index() {
            Some(i) => {
                for j in 0..4 {
                    let (x_, y_) = self.pieces[i].get_body()[j];
                    let (x, y) = (
                        x_ + self.pieces[i].get_x() as i32,
                        y_ + self.pieces[i].get_y() as i32,
                    );
                    let piece_id = self.pieces[i].get_id();
                    self.place_block(x as u32, y as u32, piece_id, false);
                }

                true
            }
            None => false,
        }
    }

    fn place_active_piece(&mut self) -> bool {
        match self.get_active_piece_index() {
            Some(i) => {
                for j in 0..4 {
                    let (x_, y_) = self.pieces[i].get_body()[j];
                    let (x, y) = (
                        x_ + self.pieces[i].get_x() as i32,
                        y_ + self.pieces[i].get_y() as i32,
                    );
                    let piece_id = self.pieces[i].get_id();
                    let is_active = self.pieces[i].is_active();
                    self.place_block(x as u32, y as u32, piece_id, is_active);
                }

                true
            }
            None => false,
        }
    }

    fn update_active_piece(&mut self) {
        self.remove_active_piece();
        self.place_active_piece();
    }

    fn update(&mut self) {
        self.update_active_piece();
    }

    fn clear_board(&mut self) {
        unreachable!(); // FIXME

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

    fn get_block(&self, x: u32, y: u32) -> u32 {
        let index = (x * self.height + y) as usize;

        self.board[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_and_place_piece() {
        fn count_active_pieces(board: &Board) -> u32 {
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

        let mut board = Board::new();

        board.set_board_size(10, 18);
        assert_eq!(0, count_active_pieces(&board));

        board.place_o_piece(1, 17);
        assert_eq!(4, count_active_pieces(&board));

        board.place_o_piece(3, 17);
        assert_eq!(8, count_active_pieces(&board));

        board.place_o_piece(5, 17);
        assert_eq!(12, count_active_pieces(&board));
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

        board.print_board();
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
