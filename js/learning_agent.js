/* jshint esversion: 6 */

import {
    memory
} from "smart-panda/smart_panda_bg";

const enumValue = (name) => Object.freeze({
    toString: () => name
});

const AgentState = Object.freeze({
    GAMEOVER: enumValue("AgentState.GAMEOVER"),
    LEARN: enumValue("AgentState.LEARN"),
    SPAWN: enumValue("AgentState.SPAWN"),
    GAMESTART: enumValue("AgentState.GAMESTART"),
    FIND_AND_PLACE: enumValue("AgentState.FIND_AND_PLACE"),
});

const rand_int = (min, max) => {
    return Math.floor(Math.random() * (max - min + 1)) + min;
};

var currentState = AgentState.GAMESTART;

var feature_weights = [-1, -1];
const feature_functions = [
    (a) => a.get_aggregate_height(),
    (a) => a.get_surface_variance()
];

var todo_rotation;
var todo_lateral_move;

var active_piece;
var board_pointer;
var board_data;

const get_board_score = (gamestate) => {
    var board_score = 0;

    var features_scores = feature_functions.map(f => f(gamestate));

    for (let index = 0; index < feature_weights.length; index++) {
        const weight = feature_weights[index];
        const score = features_scores[index];
        board_score += weight * score;
    }

    return board_score;
};

const get_board_pointer = (gamestate) => {
    const cellsPtr = gamestate.get_board_pointer();
    const width = gamestate.get_width();
    const height = gamestate.get_height();

    return new Uint32Array(memory.buffer, cellsPtr, width * height);
};

const store_board = (gamestate) => {
    board_pointer = get_board_pointer(gamestate);
    board_data = new Uint32Array(board_pointer);
    active_piece = gamestate.get_active_piece();
};

const load_board = (gamestate) => {
    set_board(board_pointer, board_data);
    gamestate.build_active_piece(
        active_piece.get_id(),
        active_piece.get_x(),
        active_piece.get_y(),
        active_piece.get_rotation()
    );
};

const set_board = (board_pointer, board_data) => {
    for (let index = 0; index < board_pointer.length; index++) {
        board_pointer[index] = board_data[index];
    }
};

export function LearningAgent(gamestate) {
    // console.log(currentState.toString());
    switch (currentState) {
        case AgentState.GAMEOVER:
            game_over_state(gamestate);
            break;
        case AgentState.GAMESTART:
            game_start_state(gamestate);
            break;
        case AgentState.SPAWN:
            spawn(gamestate);
            break;
        case AgentState.FIND_AND_PLACE:
            find_and_place(gamestate);
            break;
    }
}

const game_over_state = (gamestate) => {
    currentState = AgentState.GAMESTART;
};

const game_start_state = (gamestate) => {
    gamestate.reset();
    currentState = AgentState.SPAWN;
    gamestate.set_game_over(false);
};

const spawn = (gamestate) => {
    if (gamestate.spawn_random_piece()) {
        currentState = AgentState.FIND_AND_PLACE;
    } else {
        gamestate.set_game_over(true);
        currentState = AgentState.GAMEOVER;
    }
};

const find_and_place = (gamestate) => {
    find_best_place(gamestate);
    rotate(gamestate);
    move(gamestate);
    place(gamestate);
    currentState = AgentState.SPAWN;
};

const find_best_place = (gamestate) => {
    store_board(gamestate);

    var best_rotation;
    var best_position;
    var best_score = -Infinity;

    for (let position = -6; position <= 6; position++) {
        for (let rotation = 0; rotation < 4; rotation++) {

            todo_rotation = rotation;
            todo_lateral_move = position;

            rotate(gamestate);
            move(gamestate);
            place(gamestate);
            var score = get_board_score(gamestate);

            if (score > best_score) {
                best_rotation = rotation;
                best_position = position;
                best_score = score;
            }

            load_board(gamestate);
        }
    }

    todo_rotation = best_rotation;
    todo_lateral_move = best_position;
};

const rotate = (gamestate) => {
    if (todo_rotation > 0) {
        while (todo_rotation > 0) {
            todo_rotation -= 1;

            gamestate.rotate_active_piece_right();
        }
    }
};

const move = (gamestate) => {
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

const place = (gamestate) => {
    while (gamestate.move_active_piece_down_and_try_sleep()) {}
};