/* jshint esversion: 6 */

const enumValue = (name) => Object.freeze({
    toString: () => name
});

const AgentState = Object.freeze({
    GAMEOVER: enumValue("AgentState.GAMEOVER"),
});

var currentState = AgentState.GAMESTART;

export function LearningAgent(gamestate) {
    // gamestate.get_aggregate_height();
}