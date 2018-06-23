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
        self.update_board();
    }

    pub fn print_board(&self) {
        for j in 0..self.height {
            print!("{:4}  ", j + 1);
            for i in 0..self.width {
                let c = match self.get_piece(i, j) {
                    0 => ' ',
                    1...126 => 'O',
                    _ => 'X',
                };

                print!("{}", c);
            }

            println!();
        }
    }

    pub fn spawn_piece(&mut self, id: u32) {
        let new_piece = piece::Piece::spawn_piece(id);
        self.pieces.push(new_piece);
        self.update_board();
    }

    pub fn step(&mut self) {
        let active = self.get_active_piece_index();

        match active {
            None => (),
            Some(i) => {
                if self.can_active_piece_move_down(i) {
                    self.move_active_piece(i)
                } else {
                    self.sleep_active_piece(i);
                    self.update_board();
                }
            }
        }
    }

    pub fn has_active_piece(&mut self) -> bool {
        let active = self.get_active_piece_index();

        match active {
            None => true,
            Some(_) => false,
        }
    }

    fn sleep_active_piece(&mut self, i: usize) {
        self.pieces[i].sleep();
    }

    fn move_active_piece(&mut self, i: usize) {
        self.pieces[i].move_down();
        self.update_board();
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

            let piece_piece = self.get_piece(x as u32, (y + 1) as u32);
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

    fn update_board(&mut self) {
        self.clear_board();

        for i in 0..self.pieces.len() {
            for j in 0..4 {
                let (x_, y_) = self.pieces[i].get_body()[j];
                let (x, y) = (
                    x_ + self.pieces[i].get_x() as i32,
                    y_ + self.pieces[i].get_y() as i32,
                );
                let piece_id = self.pieces[i].get_id();
                let is_active = self.pieces[i].is_active();
                self.place_piece(x as u32, y as u32, piece_id, is_active);
            }
        }
    }

    fn clear_board(&mut self) {
        assert_eq!(self.board.len() as u32, self.width * self.height);

        for i in 0..self.width {
            for j in 0..self.height {
                self.place_piece(i, j, 0, false);
            }
        }
    }

    fn place_piece(&mut self, x: u32, y: u32, id: u32, active: bool) {
        let index = (x * self.height + y) as usize;

        self.board[index] = id;

        if active {
            self.board[index] += 127;
        }
    }

    fn get_piece(&self, x: u32, y: u32) -> u32 {
        let index = (x * self.height + y) as usize;

        self.board[index]
    }
}
