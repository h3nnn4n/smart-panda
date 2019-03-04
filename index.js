/*jshint esversion: 6 */

import * as Rust from "smart-panda";

const pre = document.getElementById("game-of-life-canvas");

Rust.console_log("Starting");

var gamestate = Rust.get_new_gamestate();
gamestate.set_board_size(10, 20);

const draw = () => {
    pre.textContent = gamestate.board_to_string();
}


const renderLoop = () => {
    if (gamestate.board_move_active_piece_down_and_try_sleep()) {
        draw();
    } else {
        if (gamestate.spawn_random_piece()) {
            draw();
        } else {
            pre.textContent = "Game Over";
        }
    }

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop)