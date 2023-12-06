import { defineConfig } from 'astro/config';
import react from '@astrojs/react';

// DOCS: https://astro.build/config
export default defineConfig({
  integrations: [react()],
  server: {
    port: 1420
  },
  vite: {
    server: {
      watch: {
        ignored: ['**/target/**']
      }
    }
  }
});