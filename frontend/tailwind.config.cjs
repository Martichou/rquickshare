const resolve = require('path').resolve

module.exports = {
  content: [resolve(__dirname, 'index.html'), resolve(__dirname, 'src/**/*.{vue,ts}')],
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
