import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
      'balm-ui-plus': 'balm-ui/dist/balm-ui-plus.esm.js',
      'balm-ui-css': 'balm-ui/dist/balm-ui.css'
    }
  },
  server: {
    host: true,
    port: 5173
  },
  preview: {
    host: true,
    port: 5173
  } 
})
