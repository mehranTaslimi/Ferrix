import { z } from "zod";
import { baseKeyValueSchema, commonKeySchema } from "./key-value";

export const httpHeaderSchema = baseKeyValueSchema.extend({
  key: commonKeySchema.regex(
    /^[a-zA-Z0-9-]+$/,
    "Header name can only contain alphanumeric characters and hyphens"
  ),
});

export const headersArraySchema = z
  .array(httpHeaderSchema)
  .optional()
  .refine((arr = []) => {
    const keys = arr.map((item) => item.key.toLowerCase());
    return new Set(keys).size === keys.length;
  }, "Duplicate headers are not allowed");
