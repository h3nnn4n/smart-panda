/* jslint esversion: 6 */

import * as RandomSearch from "./brains/random_search.js";

const brain = RandomSearch;

export function game_over_tick(gamestate) {
    brain.game_over_tick(gamestate);
}

export function get_feature_weights() {
    return brain.get_feature_weights();
}