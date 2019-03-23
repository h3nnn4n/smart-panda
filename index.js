/* jshint esversion: 6 */

import * as Rust from "./pkg/smart_panda";
import * as Draw from "./js/draw.js";
import * as RandomAgent from "./js/random_agent.js";
import * as LearningAgent from "./js/learning_agent.js";
import * as Interface from "./js/user_interface.js";

var gamestate;

const renderLoop = () => {
    if (Interface.useRandomAgent()) {
        RandomAgent.RandomAgent(gamestate);
    } else if (Interface.useLearningAgent()) {
        LearningAgent.LearningAgent(gamestate);
    }

    gamestate.clear_lines();

    Draw.draw();

    requestAnimationFrame(renderLoop);
};

const init = () => {
    Interface.initInterface();
    gamestate = Rust.get_new_gamestate();
    gamestate.set_board_size(10, 20);

    Draw.initCanvas(gamestate);

    requestAnimationFrame(renderLoop);
};


init();