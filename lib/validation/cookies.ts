import { z } from "zod";
import { baseKeyValueSchema, commonKeySchema } from "./key-value";

export const cookieSchema = baseKeyValueSchema.extend({
  key: commonKeySchema.regex(
    /^[a-zA-Z0-9_\-]+$/,
    "Cookie name can only contain alphanumeric characters, underscores and hyphens"
  ),
});
export const cookiesArraySchema = z
  .array(cookieSchema)
  .optional()
  .refine((arr = []) => {
    const keys = arr.map((item) => item.key.toLowerCase());
    return new Set(keys).size === keys.length;
  }, "Duplicate cookies are not allowed");
