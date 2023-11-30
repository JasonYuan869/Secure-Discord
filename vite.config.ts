import { defineConfig } from 'vite'
import { crx } from '@crxjs/vite-plugin'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import path from 'path'
import sveltePreprocess from 'svelte-preprocess'

// @ts-ignore
import manifest from './src/manifest'

export default defineConfig(({ mode }) => {
  const production = mode === 'production'

  return {
    build: {
      emptyOutDir: true,
      outDir: 'build',
      rollupOptions: {
        output: {
          chunkFileNames: 'assets/chunk-[hash].js',
        },
      },
    },
    plugins: [
      wasm(),
      topLevelAwait(),
      crx({ manifest }),
      svelte({
        compilerOptions: {
          dev: !production,
        },
        preprocess: sveltePreprocess(),
      }),
    ],
  }
})
