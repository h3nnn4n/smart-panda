/* jshint esversion: 6 */

import * as Print from "sprintf-js";

export function drawFeatures(gamestate) {
    const feature_function_area = document.getElementById("feature-functions");

    feature_function_area.textContent = Print.sprintf("aggregate_height: %3d", gamestate.get_aggregate_height());
}