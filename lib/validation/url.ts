import { z } from "zod";

export const urlSchema = z
  .string()
  .refine((val) => /^https?:\/\/\S+$/.test(val), {
    message: "URL is not valid.",
  });

export const urlFormSchema = z.object({ url: urlSchema });
