import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  plugins: [solidPlugin()],
  server: {
    port: 4002,
    proxy: {
      "/api": "http://localhost:4001",
    },
  },
  build: {
    target: "esnext",
  },
});
