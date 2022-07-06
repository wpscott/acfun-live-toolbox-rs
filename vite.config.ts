import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

import path from "path";
import Components from "unplugin-vue-components/vite";
import AutoImport from "unplugin-auto-import/vite";
import { ElementPlusResolver } from "unplugin-vue-components/resolvers";
import Markdown from "vite-plugin-md";
import Inspect from "vite-plugin-inspect";
import eslint from "@rollup/plugin-eslint";

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      "@/": `${path.resolve(__dirname, "./src")}/`,
      "@front/": `${path.resolve(__dirname, "./src")}/`,
    },
  },
  plugins: [
    {
      ...eslint({ fix: true, include: "src/**/*.+(vue|js|ts)" }),
      // enforce: "pre",
    },
    vue({
      include: [/\.vue$/, /\.md$/],
    }),

    // https://github.com/antfu/unplugin-auto-import
    AutoImport({
      imports: ["vue", "vue-router"],
      resolvers: [ElementPlusResolver()],
      dts: true,
    }),

    // Auto Register Vue Components https://github.com/antfu/unplugin-vue-components
    Components({
      resolvers: [ElementPlusResolver()],
      extensions: ["vue", "md"],
      include: [/\.vue$/, /\.vue\?vue/, /\.md$/],
      dts: true, // auto-generated component type definitions
    }),

    // Enable Markdown Support https://github.com/antfu/vite-plugin-md
    Markdown({
      markdownItSetup(md) {
        // https://prismjs.com/
        // md.use(Prism)
      },
    }),

    // https://github.com/antfu/vite-plugin-inspect
    Inspect({
      // change this to enable inspect for debugging
      enabled: true,
    }),
  ],
});
