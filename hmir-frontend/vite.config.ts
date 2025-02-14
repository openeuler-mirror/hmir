/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 09:38:36
 * @LastEditors: Z&N
 * @LastEditTime: 2025-02-07 17:27:48
 * @FilePath: /hmir-frontend/vite.config.ts
 * @Description: 
 */
import { defineConfig } from "vite";
import vueJsx from '@vitejs/plugin-vue-jsx';
import vue from "@vitejs/plugin-vue";
import path from 'path';
import { createSvgIconsPlugin } from 'vite-plugin-svg-icons';
import eslintPlugin from 'vite-plugin-eslint'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(),
  vueJsx(),
  eslintPlugin({
    include: ['src/**/*.js', 'src/**/*.vue', 'src/*.js', 'src/*.vue']
  }),
  createSvgIconsPlugin({
    // 指定需要缓存的图标文件夹
    iconDirs: [path.resolve(process.cwd(), 'src/assets/icons')],
    // 指定symbolId格式
    symbolId: 'icon-[dir]-[name]',
  })],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1421,
    strictPort: true,
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_", 'TAURI_ENV_*'],
  build: {
    // Tauri supports es2021
    //target: ["es2021", "chrome100", "safari13"],
    // Tauri 在 Windows 上使用 Chromium，在 macOS 和 Linux 上使用 WebKit
    target:
      process.env.TAURI_ENV_PLATFORM == 'windows'
        ? 'chrome105'
        : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  resolve: {
    alias: {
      "@": path.resolve("./src") // 相对路径别名配置，使用 @ 代替 src
    }
  }
});
