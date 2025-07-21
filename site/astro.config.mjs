// @ts-check
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
  site: 'https://dotchk.org',
  base: '/',
  compressHTML: true,
  build: {
    // Inline critical CSS for faster initial paint
    inlineStylesheets: 'auto',
    // Split CSS for better caching
    assets: 'assets',
    // Minimize CSS
    minify: true
  },
  // Enable prefetching for faster navigation
  prefetch: {
    prefetchAll: true,
    defaultStrategy: 'viewport'
  },
  // Optimize images
  image: {
    // Use optimal image formats
    domains: [],
    remotePatterns: []
  }
});