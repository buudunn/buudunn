const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const webpack = require('webpack')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
	entry: {
		index: './index.js',
	  },
	  output: {
		filename: '[name].js',
		path: path.resolve(__dirname, 'dist')
	  },
	plugins: [
		new HtmlWebpackPlugin({
			template: 'index.html'
		}),
		new WasmPackPlugin({
			crateDirectory: path.resolve(__dirname, '.')
		}),
		// Have this example work in Edge which doesn't ship `TextEncoder` or
		// `TextDecoder` at this time.
		new webpack.ProvidePlugin({
			TextDecoder: ['text-encoding', 'TextDecoder'],
			TextEncoder: ['text-encoding', 'TextEncoder']
		})
	],
	mode: 'development',
	experiments: {
		asyncWebAssembly: true
	},
	devServer: {
		port: 1337,
		allowedHosts: 'all',
		liveReload: true
	}
}