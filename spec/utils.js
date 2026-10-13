import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import * as Board from "../js/board.js";
import * as wasm from "../pkg/smart_panda.js";

let rust_initialized = false;

export const initRust = () => {
    if (!rust_initialized) {
        const wasm_bytes = readFileSync(resolve(process.cwd(), "pkg/smart_panda_bg.wasm"));

        const wasm_exports = wasm.initSync({ module: wasm_bytes });

        Board.setWasmModule(wasm_exports);

        rust_initialized = true;
    }

    return wasm;
};

export const spawn_and_place_o_piece = (gamestate, todo_lateral_move) => {
    gamestate.spawn_piece(2);
    move(gamestate, todo_lateral_move);
    drop_piece_down(gamestate);
};

export const drop_piece_down = (gamestate) => {
    while (gamestate.move_active_piece_down_and_try_sleep()) {}
};

export const move = (gamestate, todo_lateral_move) => {
    if (todo_lateral_move > 0) {
        while (todo_lateral_move > 0) {
            todo_lateral_move -= 1;

            gamestate.move_active_piece_left();
        }
    } else if (todo_lateral_move < 0) {
        while (todo_lateral_move < 0) {
            todo_lateral_move += 1;

            gamestate.move_active_piece_right();
        }
    }
};
