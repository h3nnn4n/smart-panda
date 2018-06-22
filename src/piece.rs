pub struct Piece {
    piece_id: u32,
    posx: u32,
    posy: u32,
    active: bool,
    rotation: u32,
    rotation_freedom: u32,
    body: [(u32, u32); 4],
}

impl Piece {
    pub fn new_random() -> Piece {
        Piece::o_piece()
    }

    pub fn get_body(&self) -> [(u32, u32); 4] {
        self.body
    }

    pub fn get_x(&self) -> u32 {
        self.posx
    }

    pub fn get_y(&self) -> u32 {
        self.posy
    }

    pub fn get_id(&self) -> u32 {
        self.piece_id
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    fn i_piece() -> Piece {
        Piece {
            piece_id: 1,
            posx: 0,
            posy: 3,
            active: true,
            rotation: 0,
            rotation_freedom: 2,
            body: [(0, 0), (0, 1), (0, 2), (0, 2)],
        }
    }

    fn o_piece() -> Piece {
        Piece {
            piece_id: 2,
            posx: 5,
            posy: 9,
            active: true,
            rotation: 0,
            rotation_freedom: 1,
            body: [(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }
}
