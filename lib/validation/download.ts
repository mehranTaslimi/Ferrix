import { z } from "zod";
import { urlSchema } from "./url";
import { chunkSchema } from "./chunk";

export const downloadFormSchema = z.object({
  url: urlSchema,
  chunk: chunkSchema,
});
