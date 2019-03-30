import {
    expect
} from "chai";
import {
    describe,
    it
} from "mocha";

describe("Rust Wasm Interface", () => {
    describe("Board", () => {
        it('width and height', (done) => {
            (() => import( /* webpackChunkName: "smart_panda_test" */ '../pkg/smart_panda').then(module => {
                let Rust = module;

                let gamestate = Rust.get_new_gamestate();
                gamestate.set_board_size(10, 20);

                expect(gamestate.get_width()).to.equal(10);
                expect(gamestate.get_height()).to.equal(20);

                done();
            }))();
        });
    });
});