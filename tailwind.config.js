const path = require('path')

const serverlessFunctions = path.join(
  __dirname,
  'serverless_functions',
  'src',
  '**',
  '*.{html,rs,css}'
)

const browser = path.join(__dirname, 'browser', 'src', '**', '*.{html,rs,css}')

const shared = path.join(__dirname, 'shared', 'src', '**', '*.{html,rs,css}')

console.debug('serverless functions glob', serverlessFunctions)

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    // We pass in absolute paths using command line arguments
    // in tailwind.rs because we were having some pathing issues
    // using this file.
    // './serverless_functions/src/**/*.{html,rs,css}',
    serverlessFunctions,
    // './browser/src/**/*.{html,rs,css}',
    browser,
    // './shared/src/**/*.{html,rs,css}'
    shared
  ],
  theme: {
    extend: {}
  },
  plugins: []
}
