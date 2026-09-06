// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
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
