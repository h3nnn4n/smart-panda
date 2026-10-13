import { defineConfig } from "vite";

export default defineConfig({
    base: "./",
    build: {
        outDir: "dist",
        emptyOutDir: false,
    },
    test: {
        environment: "jsdom",
        globals: true,
        include: ["spec/**/*.spec.js"],
    },
});
