import * as Utils from './utils';
import {
    expect
} from "chai";
import {
    describe,
    it
} from "mocha";

describe("Rust Wasm Interface", () => {
    describe("Board", () => {
        it('width and height is set (async)', async function () {
            const Rust = await import('../pkg/smart_panda');

            let gamestate = Rust.get_new_gamestate();
            gamestate.set_board_size(10, 20);

            expect(gamestate.get_width()).to.equal(10);
            expect(gamestate.get_height()).to.equal(20);
        });

        it('can clear lines', async function () {
            const Rust = await import('../pkg/smart_panda');

            let gamestate = Rust.get_new_gamestate();
            gamestate.set_board_size(10, 20);

            expect(gamestate.clear_lines()).to.equal(0);

            Utils.spawn_and_place_o_piece(gamestate, -4);
            Utils.spawn_and_place_o_piece(gamestate, -2);
            Utils.spawn_and_place_o_piece(gamestate, 0);
            Utils.spawn_and_place_o_piece(gamestate, 2);
            Utils.spawn_and_place_o_piece(gamestate, 4);

            expect(gamestate.clear_lines()).to.equal(2);
        });
    });
});