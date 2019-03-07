/* jshint esversion: 6 */

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
const feature_functions = [(a) => {
    a.get_aggregate_height();
}, (a) => {
    a.get_surface_variance();
}];

var todo_rotation;
var todo_lateral_move;

const get_board_score = (gamestate) => {
    var board_score = 0;

    features_scores = feature_functions.map(f => f(gamestate));

    for (let index = 0; index < feature_weights.length; index++) {
        const weight = feature_weights[index];
        const score = features_scores[index];
        board_score += weight * score;
    }

    return board_score;
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
    find_best_plate(gamestate);
    rotate(gamestate);
    move(gamestate);
    place(gamestate);
};

const find_best_plate = (gamestate) => {
    todo_rotation = rand_int(-3, 3);
    todo_lateral_move = rand_int(-5, 5);
};

const rotate = (gamestate) => {
    //
};

const move = (gamestate) => {
    //
};

const place = (gamestate) => {
    while (gamestate.move_active_piece_down_and_try_sleep()) {}
    currentState = AgentState.SPAWN;
};