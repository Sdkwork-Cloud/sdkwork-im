import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import {defineConfig, loadEnv} from 'vite';

export default defineConfig(({mode}) => {
  const env = loadEnv(mode, '.', '');
  return {
    plugins: [react(), tailwindcss()],
    define: {
// Replaced define to avoid passing server secrets to client
    },
    resolve: {
      alias: [
        { find: /^@\/(.*)/, replacement: path.resolve(__dirname, 'src/$1') },
        { find: /^@sdkwork\/clawchat-mobile-(.*)/, replacement: path.resolve(__dirname, 'packages/sdkwork-clawchat-mobile-$1/src') }
      ],
    },
    server: {
      // HMR is disabled in AI Studio via DISABLE_HMR env var.
      // Do not modifyâfile watching is disabled to prevent flickering during agent edits.
      hmr: process.env.DISABLE_HMR !== 'true',
    },
  };
});
