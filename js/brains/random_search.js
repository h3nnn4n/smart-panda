/* jslint esversion: 6 */

import {
    number_of_features
} from "../feature_functions";

const total_samples = 5;

var best_lines_cleared = 0;

var scores = [];
var feature_weights = [];

var trial_feature_weights = [];
var trial_scores = [];
var trial_samples_left = total_samples;

export function game_over_tick(gamestate) {
    update_best_lines_cleared(gamestate);

    trial_scores.push(gamestate.get_lines_cleared());
    if (tick_samples()) {
        learn();
    }
}

export function get_feature_weights() {
    return trial_feature_weights;
}

export const get_best_lines_cleared = () => {
    return best_lines_cleared;
};

const update_best_lines_cleared = (gamestate) => {
    best_lines_cleared = Math.max(
        best_lines_cleared,
        gamestate.get_lines_cleared()
    );
};

const learn = () => {
    if (mean(trial_scores) > mean(scores)) {
        feature_weights = [...trial_feature_weights];
        scores = [...trial_scores];
    }

    trial_scores = [];

    if (mean(scores) <= 1) {
        trial_feature_weights = random_feature_weights();
    } else {
        trial_feature_weights = perturbate_best_feature_weights();
    }
};

/**
 * Update the samples left for the current feature_function under evaluation
 * @returns {boolean} True if the tick finished all evaluations
 */
const tick_samples = () => {
    trial_samples_left -= 1;

    if (trial_samples_left < 0) {
        trial_samples_left = total_samples;

        return true;
    }

    return false;
};

const mean = (values) => {
    const sum = values.reduce((previous, current) => current += previous);
    const avg = sum / values.length;

    return avg;
};

const rand_float = (min, max) => {
    return Math.random() * (max - min + 1) + min;
};

const random_feature_weights = () => {
    var weights = [];

    for (let index = 0; index < number_of_features(); index++) {
        weights.push(rand_float(-5, 5));
    }

    return weights;
};

const perturbate_best_feature_weights = () => {
    const magnitude = 1;

    for (let index = 0; index < number_of_features(); index++) {
        trial_feature_weights[index] = feature_weights[index] + rand_float(-magnitude, magnitude);
    }

    return trial_feature_weights;
};

const initialize = () => {
    trial_feature_weights = random_feature_weights();
    feature_weights = random_feature_weights();
    scores = [0];
};

initialize();