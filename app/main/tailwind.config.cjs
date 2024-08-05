const resolve = require('path').resolve

module.exports = {
	content: [
		resolve(__dirname, 'index.html'),
		resolve(__dirname, 'src/**/*.{vue,ts}'),
		resolve(__dirname, '../common/vue_lib/src/**/*.{vue,ts}')
	],
	plugins: [
		require('@tailwindcss/typography'),
	],
}
