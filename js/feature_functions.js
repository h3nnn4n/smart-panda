/* jshint esversion: 6 */

import * as Print from "sprintf-js";

export function drawFeatures(gamestate) {
    const feature_function_area = document.getElementById("feature-functions");

    var feature_string = "";

    feature_string += Print.sprintf("aggregate_height: %3d\n", gamestate.get_aggregate_height());
    feature_string += Print.sprintf("surface_variance: %3d\n", gamestate.get_surface_variance());

    feature_function_area.textContent = feature_string;
}