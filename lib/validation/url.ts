import { z } from 'zod';

export const urlSchema = z.string();

export const urlFormSchema = z.object({ url: urlSchema });
