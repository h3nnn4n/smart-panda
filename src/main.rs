mod board;
mod gamestate;
mod piece;

pub fn main() {
    let mut game = gamestate::GameState::new();
    game.set_board_size(10, 18);
    game.spawn_random_piece();
    game.print_board();
}
