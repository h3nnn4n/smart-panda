/* jshint esversion: 6 */

import * as Print from "sprintf-js";

const feature_functions = [
    (a) => a.get_aggregate_height(),
    (a) => a.get_surface_variance(),
    (a) => a.get_number_of_holes(),
    (a) => a.get_delta_height()
];

export const number_of_features = () => {
    return feature_functions.length;
};

export function drawFeatures(gamestate) {
    const feature_function_area = document.getElementById("feature-functions");

    var feature_string = "";

    feature_string += Print.sprintf("aggregate_height: %3d\n", gamestate.get_aggregate_height());
    feature_string += Print.sprintf("surface_variance: %3d\n", gamestate.get_surface_variance());
    feature_string += Print.sprintf(" number_of_holes: %3d\n", gamestate.get_number_of_holes());
    feature_string += Print.sprintf("    delta_height: %3d\n", gamestate.get_delta_height());

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