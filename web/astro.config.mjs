import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
  site: 'https://keisukeyamashita.github.io/commitlint-rs',
  integrations: [
    starlight({
      title: 'Commitlint',
      logo: {
        src: '/src/assets/checker.png',
      },
      social: {
        github: 'https://github.com/KeisukeYamashita/commitlint-rs',
      },
      sidebar: [
        {
          label: '🚀 Get Started',
          autogenerate: { directory: 'setup' },
        },
        {
          label: '🔧 Configuration',
          autogenerate: { directory: 'config' },
        },
        {
          label: '✅ Rule',
          autogenerate: { directory: 'rules' },
        },
        {
          label: '🔐 License',
          items: [
            { label: "License", link: "/license" },
          ]
        },
      ],
    }),
  ],

  // Process images with sharp: https://docs.astro.build/en/guides/assets/#using-sharp
  image: { service: { entrypoint: 'astro/assets/services/sharp' } },
});
