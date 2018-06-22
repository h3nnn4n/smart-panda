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
            print!("{:4} ", j + 1);
            for i in 0..self.width {
                let c = match self.get_piece(i, j) {
                    0 => ' ',
                    1...126 => 'O',
                    _ => 'X',
                };

                print!("{} ", c);
            }

            println!();
        }
    }

    fn update_board(&mut self) {
        self.clear_board();

        for i in 0..self.pieces.len() {
            for j in 0..4 {
                let (x_, y_) = self.pieces[i].get_body()[j];
                let (x, y) = (x_ + self.pieces[i].get_x(), y_ + self.pieces[i].get_y());
                let piece_id = self.pieces[i].get_id();
                let is_active = self.pieces[i].is_active();
                self.place_piece(x, y, piece_id, is_active);
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
