const { override, addPostcssPlugins } = require('customize-cra')

module.exports = override (
    addPostcssPlugins([require('tailwindcss')]),
    addPostcssPlugins([require('autoprefixer')])
)