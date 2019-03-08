/* jshint esversion: 6 */

import * as Rust from "smart-panda";
import * as Fps from "./js/fps.js";
import * as Draw from "./js/draw.js";
import * as RandomAgent from "./js/random_agent.js";
import * as LearningAgent from "./js/learning_agent.js";
import * as Interface from "./js/user_interface.js";

var gamestate;

const renderLoop = () => {
    Fps.fps.render();

    if (Interface.useRandomAgent()) {
        RandomAgent.RandomAgent(gamestate);
    } else if (Interface.useLearningAgent()) {
        LearningAgent.LearningAgent(gamestate);
    }

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