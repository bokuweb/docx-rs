const path = require("path");
const { merge } = require("webpack-merge");
const common = require("./webpack.common.js");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = merge(common, {
  plugins: [
    ...common.plugins,
    new HtmlWebpackPlugin({ template: "assets/template.html" })
    // new WasmPackPlugin({
    //   crateDirectory: path.resolve(__dirname, ".")
    // })
  ]
});
