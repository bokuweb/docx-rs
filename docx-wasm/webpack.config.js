const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./docx.ts",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js"
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/
      }
    ]
  },
  resolve: {
    extensions: [".ts", ".js", ".wasm"]
  },
  plugins: [
    new HtmlWebpackPlugin({ template: "assets/template.html" }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, ".")
    })
    // Have this example work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    // new webpack.ProvidePlugin({
    //   TextDecoder: ["text-encoding", "TextDecoder"],
    //   TextEncoder: ["text-encoding", "TextEncoder"]
    // })
  ],
  mode: "development"
};
