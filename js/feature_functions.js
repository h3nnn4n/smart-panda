/* jshint esversion: 6 */

import * as Print from "sprintf-js";

const feature_functions = [
    (a) => a.get_aggregate_height(),
    (a) => a.get_surface_variance()
];

export function drawFeatures(gamestate) {
    const feature_function_area = document.getElementById("feature-functions");

    var feature_string = "";

    feature_string += Print.sprintf("aggregate_height: %3d\n", gamestate.get_aggregate_height());
    feature_string += Print.sprintf("surface_variance: %3d\n", gamestate.get_surface_variance());

    feature_function_area.textContent = feature_string;
}

export function get_board_score(gamestate, feature_weights) {
    var board_score = 0;

    var features_scores = feature_functions.map(f => f(gamestate));

    for (let index = 0; index < feature_weights.length; index++) {
        const weight = feature_weights[index];
        const score = features_scores[index];
        board_score += weight * score;
    }

    return board_score;
}