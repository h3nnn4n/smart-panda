const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const glob = require("glob");

let entry = "./bootstrap.js";
let outputPath = path.resolve(__dirname, "dist");

if (process.env.TESTBUILD) {
  entry = glob.sync(__dirname + "/spec/**/*.spec.js");
  outputPath = path.resolve(__dirname, "test-dist");
}

module.exports = {
  entry: entry,
  output: {
    path: outputPath,
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
  module: {
    rules: [{
      include: [
        path.resolve(__dirname, "js")
      ],
    }, {
      test: /\.js$/,
      exclude: ["/node_modules/"],
      use: [{
        loader: "babel-loader",
        options: {
          presets: [
            ['@babel/preset-env', {
              targets: "> 0.25%, not dead"
            }]
          ],
        },
      }, ],
    }, ]
  },
  node: {
    fs: 'empty'
  }
};