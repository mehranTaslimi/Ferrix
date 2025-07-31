import { z } from "zod";
import { urlSchema } from "./url";
import { chunkSchema } from "./chunk";
import { filePathSchema } from "./file-path";
import { positiveNumberSchema } from "./positive-number";
import { keyValueArraySchema } from "./key-value-array";

export const downloadFormSchema = z.object({
  url: urlSchema,
  chunk: chunkSchema,
  filePath: filePathSchema,
  // headers: keyValueArraySchema,
  // cookies: keyValueArraySchema,
  speedLimit: positiveNumberSchema({
    message: "Speed limit must be a positive number.",
  }),
  maxRetries: positiveNumberSchema({
    message: "Max entries must be a positive number.",
  }),
  backoffFactor: positiveNumberSchema({
    message: "Backoff factor must be a positive number.",
  }),
  timeoutSecs: positiveNumberSchema({
    message: "Timeout Duration be a positive number.",
  }),
});
