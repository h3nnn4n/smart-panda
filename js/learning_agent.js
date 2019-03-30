import * as Board from "./board.js";
import * as Brain from "./brain.js";
import {
    suffled_array
} from './utils';
import {
    get_board_score
} from "./feature_functions.js";
import {
    AgentState
} from "./agent_state.js";

var currentState = AgentState.GAMESTART;

var todo_rotation;
var todo_lateral_move;

export function LearningAgent(gamestate) {
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
    Brain.game_over_tick(gamestate);
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
    Board.store_board(gamestate);

    var best_rotation;
    var best_position;
    var best_score = -Infinity;
    var shuffled_positions = suffled_array(6, 13);

    for (let i = 0; i < shuffled_positions.length; i++) {
        const position = shuffled_positions[i];

        for (let rotation = 0; rotation < 4; rotation++) {

            todo_rotation = rotation;
            todo_lateral_move = position;

            rotate(gamestate);
            move(gamestate);
            place(gamestate);
            var score = get_board_score(
                gamestate,
                Brain.get_current_feature_weights()
            );

            if (score > best_score) {
                best_rotation = rotation;
                best_position = position;
                best_score = score;
            }

            Board.load_board(gamestate);
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