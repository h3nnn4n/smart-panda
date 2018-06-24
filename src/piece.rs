const SPAWN_POS_X: u32 = 5;
const SPAWN_POS_Y: u32 = 5;

pub struct Piece {
    piece_id: u32,
    posx: u32,
    posy: u32,
    active: bool,
    rotation: u32,
    rotation_freedom: u32,
    body: [(i32, i32); 4],
}

impl Piece {
    pub fn new_random() -> Piece {
        Piece::o_piece()
    }

    pub fn get_body(&self) -> [(i32, i32); 4] {
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

    pub fn hard_drop(&mut self) {
        ()
    }

    pub fn move_left(&mut self) {
        self.posx -= 1;
    }

    pub fn move_right(&mut self) {
        self.posx += 1;
    }

    pub fn move_down(&mut self) {
        self.posy += 1;
    }

    pub fn sleep(&mut self) {
        self.active = false
    }

    pub fn spawn_piece(id: u32) -> Piece {
        match id {
            1 => Piece::i_piece(),
            2 => Piece::o_piece(),
            3 => Piece::l_piece(),
            4 => Piece::j_piece(),
            5 => Piece::z_piece(),
            6 => Piece::s_piece(),
            7 => Piece::t_piece(),
            _ => Piece::o_piece(),
        }
    }

    fn i_piece() -> Piece {
        Piece {
            piece_id: 1,
            posx: SPAWN_POS_X,
            posy: SPAWN_POS_Y,
            active: true,
            rotation: 0,
            rotation_freedom: 2,
            body: [(0, 0), (0, 1), (0, 2), (0, 3)],
        }
    }

    fn o_piece() -> Piece {
        Piece {
            piece_id: 2,
            posx: SPAWN_POS_X,
            posy: SPAWN_POS_Y,
            active: true,
            rotation: 0,
            rotation_freedom: 1,
            body: [(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn l_piece() -> Piece {
        Piece {
            piece_id: 3,
            posx: SPAWN_POS_X,
            posy: SPAWN_POS_Y,
            active: true,
            rotation: 0,
            rotation_freedom: 4,
            body: [(0, 0), (0, 1), (0, 2), (1, 2)],
        }
    }

    fn j_piece() -> Piece {
        Piece {
            piece_id: 4,
            posx: SPAWN_POS_X,
            posy: SPAWN_POS_Y,
            active: true,
            rotation: 0,
            rotation_freedom: 4,
            body: [(0, 0), (0, 1), (0, 2), (-1, 2)],
        }
    }

    fn z_piece() -> Piece {
        Piece {
            piece_id: 5,
            posx: SPAWN_POS_X,
            posy: SPAWN_POS_Y,
            active: true,
            rotation: 0,
            rotation_freedom: 4,
            body: [(0, 0), (1, 0), (0, -1), (-1, -1)],
        }
    }

    fn s_piece() -> Piece {
        Piece {
            piece_id: 6,
            posx: SPAWN_POS_X,
            posy: SPAWN_POS_Y,
            active: true,
            rotation: 0,
            rotation_freedom: 4,
            body: [(0, 0), (-1, 0), (0, -1), (1, -1)],
        }
    }

    fn t_piece() -> Piece {
        Piece {
            piece_id: 7,
            posx: SPAWN_POS_X,
            posy: SPAWN_POS_Y,
            active: true,
            rotation: 0,
            rotation_freedom: 4,
            body: [(0, 0), (1, 0), (0, -1), (-1, 0)],
        }
    }
}
