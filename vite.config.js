import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  test: {
    // Test-specific configurations for Vitest
    globals: true,
    environment: 'jsdom',
    // Include your test files here
    include: ['**/test/**/*.test.js', '**/test/**/*.spec.js'],
  }
}));
