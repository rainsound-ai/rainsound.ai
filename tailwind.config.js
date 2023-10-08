const path = require('path')

// If you change this, also update the rerun-if commands in serverless_functions/build.rs.
const anyHtmlRustOrCssFileInThisFolder = path.join(
  __dirname,
  '**',
  '*.{html,rs,css}'
)

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [anyHtmlRustOrCssFileInThisFolder],
  theme: {
    extend: {}
  },
  plugins: []
}
