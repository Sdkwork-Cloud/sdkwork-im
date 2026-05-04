import { defineConfig } from 'vitepress';

export default defineConfig({
  title: 'SDKWORK UI PC React',
  description: 'Shared PC React UI framework for SDKWORK desktop-class applications.',
  cleanUrls: true,
  themeConfig: {
    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Reference', link: '/reference/package' },
    ],
    sidebar: [
      {
        text: 'Guide',
        items: [{ text: 'Getting Started', link: '/guide/getting-started' }],
      },
      {
        text: 'Reference',
        items: [
          { text: 'Package', link: '/reference/package' },
          { text: 'Framework Governance', link: '/reference/framework-governance' },
        ],
      },
    ],
    search: {
      provider: 'local',
    },
  },
});
