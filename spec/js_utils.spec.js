import * as Utils from '../js/utils';

import {
    expect,
    assert
} from "chai";
import {
    describe,
    it
} from "mocha";


describe("Utils", () => {
    describe("#mean", () => {
        it("mean of undefined is zero", () => {
            expect(Utils.mean()).to.equal(0);
        });

        it("mean of [] is zero", () => {
            expect(Utils.mean([])).to.equal(0);
        });

        it("mean of [5] is five", () => {
            expect(Utils.mean([5])).to.equal(5);
        });

        it("mean of [1, 5] is 3", () => {
            expect(Utils.mean([1, 5])).to.equal(3);
        });
    });

    describe("#rand_float", () => {
        it("between range", () => {
            for (let i = 0; i < 100; i++) {
                const low = Math.random();
                const high = Math.random() + 1;

                expect(low).to.below(high);

                const value = Utils.rand_float(low, high);

                expect(value).to.above(low);
                expect(value).to.below(high);
            }
        });
    });

    describe("#rand_int", () => {
        it("between range", () => {
            for (let i = 0; i < 100; i++) {
                const low = Math.floor(Math.random() * 3);
                const high = Math.ceil(Math.random() * 3 + 4);

                expect(low).to.below(high);

                const value = Utils.rand_int(low, high);

                expect(value).to.above(low - 1);
                expect(value).to.below(high + 1);
            }
        });
    });

    describe("#shuffle_array", () => {
        it("has the correct length (10)", () => {
            let array = Array.from({
                length: 10
            }).map((_, index) => (index));

            Utils.shuffle_array(array);

            for (let i = 0; i < 10; i++) {
                expect(array).to.include(i);
            }

            expect(array).to.have.lengthOf(10);
        });

        it("has the correct length (5)", () => {
            let array = Array.from({
                length: 5
            }).map((_, index) => (index));

            Utils.shuffle_array(array);

            expect(array).to.have.lengthOf(5);
        });

        it("has the correct values (10)", () => {
            let array = Array.from({
                length: 10
            }).map((_, index) => (index));

            Utils.shuffle_array(array);
        });

        it("has the correct values (5)", () => {
            let array = Array.from({
                length: 5
            }).map((_, index) => (index));

            Utils.shuffle_array(array);

            for (let i = 0; i < 5; i++) {
                expect(array).to.include(i);
            }
        });

        it("has the shuffled values (10)", () => {
            let array = Array.from({
                length: 10
            }).map((_, index) => (index));

            const shuffled_array = Utils.shuffle_array(array.slice(0));

            assert.notSameOrderedMembers(array, shuffled_array);
        });

        it("has the shuffled values (5)", () => {
            let array = Array.from({
                length: 5
            }).map((_, index) => (index));

            const shuffled_array = Utils.shuffle_array(array.slice(0));

            assert.notSameOrderedMembers(array, shuffled_array);
        });
    });
});