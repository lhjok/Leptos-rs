/** @type {import('tailwindcss').Config} */
module.exports = {
	mode: "all",
	content: [
		"./src/**/*.rs",
		"./index.html",
		"./src/**/*.html",
		"./src/**/*.css",
		"./node_modules/tw-elements/dist/js/**/*.js",
	],
	plugins: [
		require("tw-elements/dist/plugin")
	],
	darkMode: 'class',
	variants: {},
	theme: {},
};
