const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin');
const { override, addWebpackPlugin, addPostcssPlugins } = require('customize-cra')

module.exports = override (
    addWebpackPlugin(new MonacoWebpackPlugin({
        languages: ['cpp']
    })),
    addPostcssPlugins([require('tailwindcss')])
)