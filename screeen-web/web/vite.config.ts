import { defineConfig } from 'vite';
import { join } from 'path';
import Components from 'vite-plugin-components';
import Icons, { ViteIconsResolver } from 'vite-plugin-icons';
import Vue from '@vitejs/plugin-vue';
import WindiCSS from 'vite-plugin-windicss';
import Pages from 'vite-plugin-pages';

const extensions: Array<string> = ['vue', 'js', 'jsx', 'ts', 'tsx'];

export const config = defineConfig({
	plugins: [
		Components({
			extensions,
			customComponentResolvers: [
				ViteIconsResolver({
					componentPrefix: 'i',
					enabledCollections: [
						'feather',
						'heroicons-outline',
					],
				}),
			],
		}),
		Icons(),
		Pages({
			extensions,
		}),
		Vue(),
		WindiCSS({
			config: {
				darkMode: 'class',
				plugins: [
					require('windicss/plugin/aspect-ratio'),
					require('windicss/plugin/forms'),
					require('windicss/plugin/typography'),
				],
				theme: {
					extend: {
						colors: {
							gray: {
								100: '#eaeaeb',
								200: '#cacbcd',
								300: '#a7a9ac',
								400: '#696c71',
								500: '#282d34',
								600: '#24292f',
								700: '#181b20',
								800: '#121518',
								900: '#0c0e10',
							},
						},
					},
				},
			}
		}),
	],
	resolve: {
		alias: {
			'~': join(__dirname, './src/'),
		}
	}
});

export default config;
