/* jshint esversion: 6 */

import * as Print from "sprintf-js";
import * as Brain from "./brain.js";

export function drawScore(gamestate) {
    const score_area = document.getElementById("score");

    var score_string = "";

    // score_string += Print.sprintf("        score: %3d\n", gamestate.get_points());
    score_string += Print.sprintf("lines_cleared: %3d\n", gamestate.get_lines_cleared());
    score_string += Print.sprintf("         best: %3d\n", Brain.get_best_lines_cleared());

    score_string += get_weight_string();

    score_area.textContent = score_string;
}

const get_weight_string = () => {
    var weights = Brain.get_feature_weights();
    var weight_string = "\n";

    weights.forEach(weight => {
        weight_string += Print.sprintf("%6.2f ", weight);
    });

    weight_string += '\n';

    return weight_string;
};