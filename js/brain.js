/* jslint esversion: 6 */

import * as RandomSearch from "./brains/random_search.js";

const brain = RandomSearch;

export function tick(gamestate) {
    brain.tick();
}

export function get_feature_weights() {
    return brain.get_feature_weights();
}