import { resolve } from 'node:path';
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import dts from 'vite-plugin-dts';
import { createFrameworkLibEntrySourceMap } from './build/package-contract';

const peerDependencies = ['react', 'react-dom', 'react-hook-form'];
const frameworkLibEntrySourceMap = createFrameworkLibEntrySourceMap();

function isPeerDependency(id: string): boolean {
  return peerDependencies.some((dependency) => id === dependency || id.startsWith(`${dependency}/`));
}

export default defineConfig({
  plugins: [
    react(),
    tailwindcss(),
    dts({
      include: ['src/**/*'],
      exclude: ['src/**/*.test.ts', 'src/**/*.test.tsx'],
      outDir: 'dist',
      entryRoot: 'src',
    }),
  ],
  build: {
    outDir: 'dist',
    sourcemap: true,
    emptyOutDir: true,
    lib: {
      entry: Object.fromEntries(
        Object.entries(frameworkLibEntrySourceMap).map(([entryName, sourcePath]) => [entryName, resolve(__dirname, sourcePath)]),
      ),
      formats: ['es'],
      fileName: (_format, entryName) => `${entryName}.js`,
      cssFileName: 'sdkwork-ui',
    },
    rollupOptions: {
      external: isPeerDependency,
    },
  },
});
