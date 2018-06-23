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
    game.print_board();
    game.step();
}
