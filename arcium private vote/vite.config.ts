     1|import { defineConfig } from 'vite';
     2|import tailwindcss from '@tailwindcss/vite';
     3|import react from '@vitejs/plugin-react';
     4|import { cloudflare } from '@cloudflare/vite-plugin';
     5|
     6|// https://vite.dev/config/
     7|export default defineConfig({
     8|	plugins: [
     9|		tailwindcss(), // Required for Tailwind v4
    10|		react(),
    11|		cloudflare(), // Cloudflare Workers integration
    12|	],
    13|	server: {
    14|		port: 8000,
    15|		host: true,
    16|	},
    17|});
    18|