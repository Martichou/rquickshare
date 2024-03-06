const resolve = require('path').resolve
const colors = require('tailwindcss/colors')

module.exports = {
	content: [resolve(__dirname, 'index.html'), resolve(__dirname, 'src/**/*.{vue,ts}')],
	plugins: [
		require("daisyui"),
		require('@tailwindcss/typography'),
	],
	daisyui: {
		themes: [{
			rquickshare: {
				"primary": colors.green[200],
				"secondary": colors.white,
				"base-100": colors.green[50],
				"base-300": colors.green[100]
			},
		}
		],
		logs: false,
	},
}
