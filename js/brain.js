import * as MonteCarlo from "./brains/monte_carlo.js";

const brain = MonteCarlo;

export const game_over_tick = (gamestate) => {
    brain.game_over_tick(gamestate);
};

export const get_best_feature_weights = () => {
    return brain.get_best_feature_weights();
};

export const get_current_feature_weights = () => {
    return brain.get_current_feature_weights();
};

export const get_current_mean = () => {
    return brain.get_current_mean();
};

export const get_best_mean = () => {
    return brain.get_best_mean();
};

export const get_best_lines_cleared = () => {
    return brain.get_best_lines_cleared();
};