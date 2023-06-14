import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
  site: 'https://keisukeyamashita.github.io',
  base: '/commitlint-rs',
  integrations: [
    starlight({
      title: 'Commitlint',
      social: {
        github: 'https://github.com/KeisukeYamashita/commitlint-rs',
      },
      sidebar: [
        {
          label: '🚀 Get Started',
          autogenerate: { directory: 'setup' },
        },
      ],
    }),
  ],

  // Process images with sharp: https://docs.astro.build/en/guides/assets/#using-sharp
  image: { service: { entrypoint: 'astro/assets/services/sharp' } },
});
