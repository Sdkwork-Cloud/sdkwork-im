import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
import { resolve } from 'path';

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, 'src/index.ts'),
      name: 'SdkworkBackend',
      formats: ['es', 'cjs'],
      fileName: (format) => `index.${format === 'es' ? 'js' : 'cjs'}`,
    },
    rollupOptions: {
      external: ['@sdkwork/sdk-common'],
      output: {
        globals: {
          '@sdkwork/sdk-common': 'SdkworkSdkCommon',
        },
      },
    },
    sourcemap: true,
  },
  plugins: [
    dts({ 
      include: ['src'],
      outDir: 'dist',
    }),
  ],
});
