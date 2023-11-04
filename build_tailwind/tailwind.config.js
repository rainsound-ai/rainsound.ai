const path = require('path')

// If you change this, also update the rerun-if commands in serverless_functions/build.rs.
// We were running into some funny business with relative paths which is why we use
// __dirname here.
const anyHtmlRustOrCssFileInTheWorkspace = path.resolve(
  __dirname,
  '..',
  '**',
  '*.{html,rs,css}'
)

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [anyHtmlRustOrCssFileInTheWorkspace],
  theme: {
    extend: {}
  },
  plugins: []
}
