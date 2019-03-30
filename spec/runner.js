import "mocha";

let context = require.context('./', true, /\.spec\.js/);
context.keys().forEach(context);
module.exports = context;