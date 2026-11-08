import { defineConfig } from "vite";

export default defineConfig({
  plugins: [],
  server: {
    port: 3000,
    open: true,
  },
  build: {
    target: "es2022",
    outDir: "dist",
    sourcemap: true,
  },
});
