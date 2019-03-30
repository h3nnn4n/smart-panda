import * as Utils from './utils';
import {
    expect
} from "chai";
import {
    describe,
    it
} from "mocha";

describe("Rust Wasm Interface", () => {
    before(async function () {
        this.Rust = await import('../pkg/smart_panda');
    });

    describe("Board", () => {
        it('width and height is set', function () {
            let gamestate = this.Rust.get_new_gamestate();
            gamestate.set_board_size(10, 20);

            expect(gamestate.get_width()).to.equal(10);
            expect(gamestate.get_height()).to.equal(20);
        });

        it('can clear lines', function () {
            let gamestate = this.Rust.get_new_gamestate();
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

    describe("Active piece", () => {
        describe("#has_active_piece", () => {
            it('yes', function () {
                let gamestate = this.Rust.get_new_gamestate();
                gamestate.set_board_size(10, 20);
                gamestate.spawn_piece(2);

                expect(gamestate.has_active_piece()).to.equal(true);
            });

            it('no', function () {
                let gamestate = this.Rust.get_new_gamestate();
                gamestate.set_board_size(10, 20);

                expect(gamestate.has_active_piece()).to.equal(false);
            });
        });
    });
});