import eslint from '@eslint/js'
import { defineConfig } from 'eslint/config'
import tseslint from 'typescript-eslint'

export default defineConfig(eslint.configs.recommended, tseslint.configs.recommended, {
  ignores: [
    '.next/**',
    'node_modules/**',
    'target/**',
    'public/wasm/**',
    'out/**',
    '*.config.js',
    '*.config.mjs',
  ],
})
