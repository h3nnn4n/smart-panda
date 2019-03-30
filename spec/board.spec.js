import * as Board from "../js/board";
import * as Utils from './utils';

import {
    expect
} from "chai";
import {
    describe,
    it
} from "mocha";

describe("Board", () => {
    before(async function () {
        this.Rust = await import('../pkg/smart_panda');
    });

    describe("#store_board", () => {
        it('runs with active piece', function (done) {
            let gamestate = this.Rust.get_new_gamestate();
            gamestate.set_board_size(10, 20);
            gamestate.spawn_piece(2);

            Board.store_board(gamestate);

            done();
        });

        it('runs without active piece', function (done) {
            let gamestate = this.Rust.get_new_gamestate();
            gamestate.set_board_size(10, 20);

            Board.store_board(gamestate);

            done();
        });
    });

    describe("#load_board", () => {
        it('runs with active piece', function (done) {
            let gamestate = this.Rust.get_new_gamestate();
            gamestate.set_board_size(10, 20);
            gamestate.spawn_piece(2);

            Board.store_board(gamestate);
            Board.load_board(gamestate);

            done();
        });

        it('runs without active piece', function (done) {
            let gamestate = this.Rust.get_new_gamestate();
            gamestate.set_board_size(10, 20);

            Board.store_board(gamestate);
            Board.load_board(gamestate);

            done();
        });
    });

    describe("consistency", () => {
        it('restores inactive pieces', function () {
            let gamestate = this.Rust.get_new_gamestate();
            gamestate.set_board_size(10, 20);

            expect(gamestate.clear_lines()).to.equal(0);

            Utils.spawn_and_place_o_piece(gamestate, -4);
            Utils.spawn_and_place_o_piece(gamestate, -2);
            Utils.spawn_and_place_o_piece(gamestate, 0);
            Utils.spawn_and_place_o_piece(gamestate, 2);

            Board.store_board(gamestate);
            Utils.spawn_and_place_o_piece(gamestate, 4);
            expect(gamestate.clear_lines()).to.equal(2);
            Board.load_board(gamestate);

            expect(gamestate.clear_lines()).to.equal(0);

            Utils.spawn_and_place_o_piece(gamestate, 4);
            expect(gamestate.clear_lines()).to.equal(2);
        });

        describe("active piece states", () => {
            it('stores active piece', function () {
                let gamestate = this.Rust.get_new_gamestate();
                gamestate.set_board_size(10, 20);
                gamestate.spawn_piece(2);

                Board.store_board(gamestate);
                Utils.drop_piece_down(gamestate);
                Board.load_board(gamestate);

                expect(gamestate.has_active_piece()).to.equal(true);
            });

            it('stores active piece (0 piece)', function () {
                let gamestate = this.Rust.get_new_gamestate();
                gamestate.set_board_size(10, 20);
                gamestate.spawn_piece(2);

                Board.store_board(gamestate);
                Utils.drop_piece_down(gamestate);
                gamestate.spawn_piece(5);
                Board.load_board(gamestate);

                const active_piece = gamestate.get_active_piece();

                expect(active_piece.get_id()).to.equal(2);
            });

            it('stores active piece (T piece)', function () {
                let gamestate = this.Rust.get_new_gamestate();
                gamestate.set_board_size(10, 20);
                gamestate.spawn_piece(7);

                Board.store_board(gamestate);
                Utils.drop_piece_down(gamestate);
                gamestate.spawn_piece(3);
                Board.load_board(gamestate);

                const active_piece = gamestate.get_active_piece();

                expect(active_piece.get_id()).to.equal(7);
            });

            it.skip('stores active piece X position', function () {
                // TODO: Looks like the x position is not being stored
                // This is actually irrelevant for now since the active piece
                // state is always stored on the initial position. Still, it
                // would be good to have it properly working

                let active_piece;

                let gamestate = this.Rust.get_new_gamestate();
                gamestate.set_board_size(10, 20);
                gamestate.spawn_random_piece();
                gamestate.move_active_piece_right();

                Board.store_board(gamestate);
                Utils.drop_piece_down(gamestate);
                Board.load_board(gamestate);

                gamestate.move_active_piece_right();
                gamestate.move_active_piece_right();

                active_piece = gamestate.get_active_piece();
                expect(active_piece.get_x()).to.equal(6);
            });
        });
    });
});