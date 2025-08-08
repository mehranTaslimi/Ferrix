import { z } from "zod";

export const commonKeySchema = z
  .string()
  .min(1, "Key is required")
  .max(256, "Key must be less than 256 characters");

export const commonValueSchema = z
  .string()
  .min(1, "Value is required")
  .max(2048, "Value must be less than 2048 characters");

export const baseKeyValueSchema = z.object({
  key: commonKeySchema,
  value: commonValueSchema,
});

export const keyValueArraySchema = z
  .array(baseKeyValueSchema)
  .optional()
  .refine((arr = []) => {
    const keys = arr.map((item) => item.key.toLowerCase());
    return new Set(keys).size === keys.length;
  }, "Duplicate keys are not allowed");
