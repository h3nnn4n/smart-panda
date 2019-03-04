#![allow(dead_code)]

extern crate rand;

use self::rand::Rng;

const SPAWN_POS_X: u32 = 5;
const SPAWN_POS_Y: u32 = 1;

pub struct Piece {
    piece_id: u32,
    posx: u32,
    posy: u32,
    active: bool,
    rotation: u32,
    rotation_freedom: u32,
    body: [[i32; 2]; 4],
    rotation_matrix: [[[i32; 2]; 4]; 4],
}

impl Piece {
    pub fn new_random() -> Piece {
        let id = rand::thread_rng().gen_range(1, 8);
        Piece::spawn_piece(id)
    }

    pub fn get_body(&self) -> [[i32; 2]; 4] {
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

    pub fn rotate_right(&mut self) {
        self.rotation = (self.rotation + 1) % self.rotation_freedom;

        self.update_piece_body()
    }

    pub fn rotate_left(&mut self) {
        self.rotation -= 1;

        self.update_piece_body()
    }

    pub fn move_down(&mut self) {
        self.posy += 1;
    }

    pub fn sleep(&mut self) {
        self.active = false
    }

    pub fn spawn_piece(id: u32) -> Piece {
        Piece::spawn_and_place_piece(id, SPAWN_POS_X, SPAWN_POS_Y)
    }

    pub fn spawn_and_place_piece(id: u32, x: u32, y: u32) -> Piece {
        match id {
            1 => Piece::i_piece(x, y),
            2 => Piece::o_piece(x, y),
            3 => Piece::l_piece(x, y),
            4 => Piece::j_piece(x, y),
            5 => Piece::z_piece(x, y),
            6 => Piece::s_piece(x, y),
            7 => Piece::t_piece(x, y),
            _ => unreachable!(),
        }
    }

    fn update_piece_body(&mut self) {
        self.body = self.rotation_matrix[self.rotation as usize];
    }

    fn i_piece(x: u32, y: u32) -> Piece {
        Piece {
            piece_id: 1,
            posx: x,
            posy: y,
            active: true,
            rotation: 0,
            rotation_freedom: 2,
            body: [[-2, 0], [-1, 0], [0, 0], [1, 0]],
            rotation_matrix: [
                [[-2, 0], [-1, 0], [0, 0], [1, 0]],
                [[0, -2], [0, -1], [0, 0], [0, 1]],
                [[-2, 0], [-1, 0], [0, 0], [1, 0]],
                [[0, -2], [0, -1], [0, 0], [0, 1]],
            ],
        }
    }

    fn o_piece(x: u32, y: u32) -> Piece {
        Piece {
            piece_id: 2,
            posx: x,
            posy: y,
            active: true,
            rotation: 0,
            rotation_freedom: 1,
            body: [[-1, -1], [0, -1], [-1, 0], [0, 0]],
            rotation_matrix: [
                [[-1, -1], [0, -1], [-1, 0], [0, 0]],
                [[-1, -1], [0, -1], [-1, 0], [0, 0]],
                [[-1, -1], [0, -1], [-1, 0], [0, 0]],
                [[-1, -1], [0, -1], [-1, 0], [0, 0]],
            ],
        }
    }

    fn l_piece(x: u32, y: u32) -> Piece {
        Piece {
            piece_id: 3,
            posx: x,
            posy: y,
            active: true,
            rotation: 0,
            rotation_freedom: 4,
            body: [[-1, 0], [1, 0], [1, 1], [0, 0]],
            rotation_matrix: [
                [[-1, 0], [1, 0], [1, 1], [0, 0]],
                [[-1, 1], [0, 1], [0, -1], [0, 0]],
                [[-1, 0], [1, 0], [-1, -1], [0, 0]],
                [[1, -1], [0, 1], [0, -1], [0, 0]],
            ],
        }
    }

    fn j_piece(x: u32, y: u32) -> Piece {
        Piece {
            piece_id: 4,
            posx: x,
            posy: y,
            active: true,
            rotation: 0,
            rotation_freedom: 4,
            body: [[-1, 0], [1, 0], [-1, 1], [0, 0]],
            rotation_matrix: [
                [[-1, 0], [1, 0], [-1, 1], [0, 0]],
                [[-1, -1], [0, 1], [0, -1], [0, 0]],
                [[-1, 0], [1, 0], [1, -1], [0, 0]],
                [[1, 1], [0, 1], [0, -1], [0, 0]],
            ],
        }
    }

    fn z_piece(x: u32, y: u32) -> Piece {
        Piece {
            piece_id: 5,
            posx: x,
            posy: y,
            active: true,
            rotation: 0,
            rotation_freedom: 2,
            body: [[-1, 0], [0, 0], [0, 1], [1, 1]],
            rotation_matrix: [
                [[-1, 0], [0, 0], [0, 1], [1, 1]],
                [[-1, 1], [-1, 0], [0, 0], [0, -1]],
                [[-1, 0], [0, 0], [0, 1], [1, 1]],
                [[-1, 1], [-1, 0], [0, 0], [0, -1]],
            ],
        }
    }

    fn s_piece(x: u32, y: u32) -> Piece {
        Piece {
            piece_id: 6,
            posx: x,
            posy: y,
            active: true,
            rotation: 0,
            rotation_freedom: 2,
            body: [[-1, 1], [0, 1], [0, 0], [1, 0]],
            rotation_matrix: [
                [[-1, 1], [0, 1], [0, 0], [1, 0]],
                [[-1, -1], [-1, 0], [0, 0], [0, 1]],
                [[-1, 1], [0, 1], [0, 0], [1, 0]],
                [[-1, -1], [-1, 0], [0, 0], [0, 1]],
            ],
        }
    }

    fn t_piece(x: u32, y: u32) -> Piece {
        Piece {
            piece_id: 7,
            posx: x,
            posy: y,
            active: true,
            rotation: 0,
            rotation_freedom: 4,
            body: [[-1, -1], [0, -1], [1, -1], [0, 0]],
            rotation_matrix: [
                [[-1, 0], [0, 0], [1, 0], [0, 1]],
                [[0, -1], [0, 0], [1, 0], [0, 1]],
                [[0, -1], [-1, 0], [0, 0], [1, 0]],
                [[0, -1], [-1, 0], [0, 0], [0, 1]],
            ],
        }
    }
}
