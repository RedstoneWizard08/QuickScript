//@ts-check

"use strict";

const path = require("path");

/**@type {import('webpack').Configuration}*/
const config = {
    target: "webworker",
    entry: "./src/extension.ts",

    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "extension.js",
        libraryTarget: "commonjs2",
        devtoolModuleFilenameTemplate: "../[resource-path]",
    },

    devtool: "hidden-source-map",

    externals: {
        vscode: "commonjs vscode",
    },

    resolve: {
        mainFields: ["browser", "module", "main"],
        extensions: [".ts", ".js"],
        alias: {},
    },

    module: {
        rules: [
            {
                test: /\.ts$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "ts-loader",
                    },
                ],
            },
        ],
    },
};

module.exports = config;
