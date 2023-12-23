const path = require('path')

/** @typedef {import('tailwindcss/types/config').ResolvableTo<import('tailwindcss/types/config').KeyValuePair>} ThemeExtension*/

/**@type {ThemeExtension} */
const spacing = {}
/**@type {ThemeExtension} */
const fontSize = {}

// We were running into some funny business with relative paths which is why we use
// __dirname here.
const anyHtmlRustOrCssFileInTheWorkspace = path.resolve(
  __dirname,
  '..',
  '..',
  '**',
  '*.{html,rs,css}'
)

console.log(`Using ${anyHtmlRustOrCssFileInTheWorkspace} as the content path.`)

for (let x = 0; x <= 180; x++) {
  const key = `grid-${x}`
  const value = `calc(${x}*100vw/180)`

  spacing[key] = value
  fontSize[key] = value
}

/** @type {import('tailwindcss').Config}*/
const config = {
  content: [anyHtmlRustOrCssFileInTheWorkspace],

  theme: {
    extend: {
      fontFamily: {
        fugi: ['Fugi'],
        'aurora-grotesk': ['Aurora Grotesk'],
        clearface: ['Clearface']
      },
      borderRadius: {
        tooth: 'calc(10*100vw/180)'
      },
      colors: {
        slate: '#283036', // background
        dark: '#1B2430', // Dark background overlay
        neutral: '#E6E6E6',
        petal: {
          lavender: '#E189E2',
          salmon: '#E189AB',
          orange: '#E1A889',
          yellow: '#E2D989',
          blue: '#8997E1',
          purple: '#BB89E2' // called barney purple in figma
        }
      },
      fontSize,
      spacing
    }
  },

  plugins: []
}

module.exports = config
