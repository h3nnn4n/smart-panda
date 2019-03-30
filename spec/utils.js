export const spawn_and_place_o_piece = (gamestate, todo_lateral_move) => {
    gamestate.spawn_piece(2);
    move(gamestate, todo_lateral_move);
    drop_piece_down(gamestate);
};

export const drop_piece_down = (gamestate) => {
    while (gamestate.move_active_piece_down_and_try_sleep()) {}
};

export const move = (gamestate, todo_lateral_move) => {
    if (todo_lateral_move > 0) {
        while (todo_lateral_move > 0) {
            todo_lateral_move -= 1;

            gamestate.move_active_piece_left();
        }
    } else if (todo_lateral_move < 0) {
        while (todo_lateral_move < 0) {
            todo_lateral_move += 1;

            gamestate.move_active_piece_right();
        }
    }
};