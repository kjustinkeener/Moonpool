import { defineCollection } from 'astro:content';
import { docsLoader, i18nLoader } from '@astrojs/starlight/loaders';
import { docsSchema, i18nSchema } from '@astrojs/starlight/schema';

export const collections = {
	docs: defineCollection({ loader: docsLoader(), schema: docsSchema() }),
	// Starlight ships zh-CN/zh-TW UI strings but not under the zh-Hant tag; zh-hant.json is the zh-TW set.
	i18n: defineCollection({ loader: i18nLoader(), schema: i18nSchema() }),
};
