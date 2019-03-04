/*jshint esversion: 6 */

import * as Rust from "smart-panda";

Rust.console_log("Starting");
console.log(Rust);

var gamestate = Rust.get_new_gamestate();
gamestate.set_board_size(10, 10);

console.log(gamestate);
// debugger;