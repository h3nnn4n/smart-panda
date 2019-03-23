const enumValue = (name) => Object.freeze({
    toString: () => name
});

const AgentState = Object.freeze({
    GAMEOVER: enumValue("AgentState.GAMEOVER"),
    GAMESTART: enumValue("AgentState.GAMESTART"),
    SPAWN: enumValue("AgentState.SPAWN"),
    ROTATE: enumValue("AgentState.ROTATE"),
    MOVE_RIGHT: enumValue("AgentState.MOVE_RIGHT"),
    MOVE_LEFT: enumValue("AgentState.MOVE_LEFT"),
    MOVE_DOWN: enumValue("AgentState.MOVE_DOWN"),
});

var currentState = AgentState.GAMESTART;
var gameOverCounter = 60;
const gameOverFrames = 90;

export function RandomAgent(gamestate) {
    switch (currentState) {
        case AgentState.GAMEOVER:
            if (gameOverCounter > 0) {
                gameOverCounter -= 1;
            } else {
                gameOverCounter = gameOverFrames;
                currentState = AgentState.GAMESTART;
            }
            break;
        case AgentState.GAMESTART:
            gamestate.reset();
            currentState = AgentState.SPAWN;
            gamestate.set_game_over(false);
            break;
        case AgentState.SPAWN:
            if (gamestate.spawn_random_piece()) {
                currentState = AgentState.ROTATE;
            } else {
                gamestate.set_game_over(true);
                currentState = AgentState.GAMEOVER;
            }
            break;
        case AgentState.ROTATE:
            if (Math.random() < 0.33) {
                gamestate.rotate_active_piece_right();
            } else {
                if (Math.random() < 0.5) {
                    currentState = AgentState.MOVE_RIGHT;
                } else {
                    currentState = AgentState.MOVE_LEFT;
                }
            }
            break;
        case AgentState.MOVE_RIGHT:
            if (Math.random() < 0.75) {
                gamestate.move_active_piece_right();
            } else {
                currentState = AgentState.MOVE_DOWN;
            }
            break;
        case AgentState.MOVE_LEFT:
            if (Math.random() < 0.75) {
                gamestate.move_active_piece_left();
            } else {
                currentState = AgentState.MOVE_DOWN;
            }
            break;
        case AgentState.MOVE_DOWN:
            if (!gamestate.move_active_piece_down_and_try_sleep()) {
                currentState = AgentState.SPAWN;
            }
            break;
        default:
    }
}