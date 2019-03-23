import * as Brain from "../js/brain";
import * as FeatureFunctions from "../js/feature_functions";
import {
    expect
} from "chai";
import {
    describe,
    it
} from "mocha";

describe("Brain", () => {
    it("best_feature_weights have the currect number of elements", () => {
        const best_feature_weights = Brain.get_best_feature_weights();

        expect(best_feature_weights.length).to.equal(FeatureFunctions.number_of_features());
    });

    it("current_feature_weights have the currect number of elements", () => {
        const current_feature_weights = Brain.get_current_feature_weights();

        expect(current_feature_weights.length).to.equal(FeatureFunctions.number_of_features());
    });

    it("best_lines_cleared is zero", () => {
        const best_lines_cleared = Brain.get_best_lines_cleared();

        expect(best_lines_cleared).to.equal(0);
    });

    it("current_mean is zero", () => {
        const current_mean = Brain.get_current_mean();

        expect(current_mean).to.equal(0);
    });

    it("best_mean is zero", () => {
        const best_mean = Brain.get_best_mean();

        expect(best_mean).to.equal(0);
    });
});