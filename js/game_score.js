/* jshint esversion: 6 */

import * as Print from "sprintf-js";

export function drawFeatures(gamestate) {
    const score_area = document.getElementById("score");

    var score_string = "";

    score_string += Print.sprintf("        score: %3d\n", gamestate.get_points());
    score_string += Print.sprintf("lines_cleared: %3d\n", gamestate.get_lines_cleared());

    score_area.textContent = score_string;
}