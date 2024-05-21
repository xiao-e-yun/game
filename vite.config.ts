import { defineConfig } from 'vite'

import vue from '@vitejs/plugin-vue'
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
  ],
  base: "",
  build: {
    target: 'ESNext',
    assetsDir: "assets"
  },
  resolve:{
    alias:{
      'wasm' : path.resolve(__dirname, './wasm/pkg'),
    },
  },
})
