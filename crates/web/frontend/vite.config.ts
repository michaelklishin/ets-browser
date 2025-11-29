import { defineConfig } from 'vite'

export default defineConfig({
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:15692',
        changeOrigin: true,
      },
    },
  },
  build: {
    outDir: 'dist',
  },
})
