/* jshint esversion: 6 */

import * as Rust from "smart-panda";
import * as Fps from "./js/fps.js";
import * as RandomPlayer from "./js/random_player.js";
import * as Draw from "./js/draw.js";
import * as Interface from "./js/user_interface.js";

// Lets use the js call from rust to js again just to make sure
// that everything (or at least this) is working
Rust.console_log("Starting");

var gamestate;

const renderLoop = () => {
    Fps.fps.render();

    if (Interface.useRandomAgent()) {
        RandomPlayer.RandomAgent(gamestate);
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