/* jslint esversion: 6 */

import {
    number_of_features
} from "../feature_functions";

const temperature = 1.0;

const total_samples = 10;

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

export function get_best_feature_weights() {
    return feature_weights;
}

export function get_current_feature_weights() {
    return trial_feature_weights;
}

export const get_best_lines_cleared = () => {
    return best_lines_cleared;
};

export const get_current_mean = () => {
    return mean(trial_scores);
};

export const get_best_mean = () => {
    return mean(scores);
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
    } else {
        const delta_energy = mean(scores) - mean(trial_scores);

        if (Math.exp(-delta_energy / temperature) >= Math.random()) {
            feature_weights = [...trial_feature_weights];
            scores = [...trial_scores];
        }
    }

    trial_scores = [];

    perturbate_trial_feature_weights();
};

const perturbate_trial_feature_weights = () => {
    if (mean(scores) <= 4 || Math.random() < 0.1) {
        trial_feature_weights = random_feature_weights();
    } else {
        if (Math.random() < 0.5) {
            trial_feature_weights = perturbate_from_best_feature_weights();
        } else {
            trial_feature_weights = perturbate_one_variable_from_best_feature_weights();
        }
    }
};

/**
 * Update the samples left for the current feature_function under evaluation
 * @returns {boolean} True if the tick finished all evaluations
 */
const tick_samples = () => {
    trial_samples_left -= 1;

    if (trial_samples_left < 0 ||
        mean(trial_scores) <= 5 ||
        (mean(trial_scores) < mean(scores) / 2 && trial_scores.length > 3)
    ) {
        trial_samples_left = total_samples;

        return true;
    }

    return false;
};

const mean = (values) => {
    if (!values || values.length === 0) {
        return 0;
    }

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

const perturbate_from_best_feature_weights = () => {
    const magnitude = 1;

    for (let index = 0; index < number_of_features(); index++) {
        trial_feature_weights[index] = feature_weights[index] + rand_float(-magnitude, magnitude);
    }

    return trial_feature_weights;
};

const perturbate_one_variable_from_best_feature_weights = () => {
    const magnitude = 1;
    const index = rand_int(0, trial_feature_weights.length - 1);

    trial_feature_weights[index] = feature_weights[index] + rand_float(-magnitude, magnitude);

    return trial_feature_weights;
};

var rand_int = (minimum, maximum) => {
    return Math.floor(Math.random() * (maximum - minimum + 1)) + minimum;
};

const initialize = () => {
    trial_feature_weights = random_feature_weights();
    feature_weights = random_feature_weights();
    scores = [0];
};

initialize();