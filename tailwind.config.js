/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './html_generator/src/**/*.{html,rs,css}',
    './browser/src/**/*.{html,rs,css}',
    './shared/src/**/*.{html,rs,css}'
  ],
  theme: {
    extend: {}
  },
  plugins: []
}
