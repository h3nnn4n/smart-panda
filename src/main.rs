use std::{thread, time};

mod board;
mod gamestate;
mod piece;

pub fn main() {
    let mut game = gamestate::GameState::new();
    game.set_board_size(10, 18);
    //game.spawn_random_piece();
    game.spawn_piece(1);
    //game.spawn_piece(2);
    //game.spawn_piece(3);
    //game.spawn_piece(4);
    //game.spawn_piece(5);
    //game.spawn_piece(6);
    //game.spawn_piece(7);
    //game.print_board();

    for _ in 0..40 {
        print!("{}[2J", 27 as char);
        game.print_board();
        thread::sleep(time::Duration::from_millis(120));

        game.step();

        if game.has_active_piece() {
            game.spawn_piece(2);
        }
    }
}
