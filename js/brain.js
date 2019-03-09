/* jslint esversion: 6 */

import * as RandomSearch from "./brains/random_search.js";

const brain = RandomSearch;

export const game_over_tick = (gamestate) => {
    brain.game_over_tick(gamestate);
};

export const get_feature_weights = () => {
    return brain.get_feature_weights();
};

export const get_best_lines_cleared = () => {
    return brain.get_best_lines_cleared();
};