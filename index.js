/*jshint esversion: 6 */

import * as Rust from "smart-panda";
import * as RandomPlayer from "./js/random_player.js";

const pre = document.getElementById("game-of-life-canvas");

Rust.console_log("Starting");

var gamestate = Rust.get_new_gamestate();
gamestate.set_board_size(10, 20);

const draw = () => {
    if (RandomPlayer.isGameOver()) {
        pre.textContent = 'Game Over';
    } else {
        pre.textContent = gamestate.board_to_string();
    }
};

RandomPlayer.startGame();

const renderLoop = () => {
    RandomPlayer.RandomAgent(gamestate);

    draw();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);