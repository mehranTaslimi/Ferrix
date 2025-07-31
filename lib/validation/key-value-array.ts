import { z } from "zod";

export const keyValueArraySchema = z
  .array(
    z.object({
      key: z.string().min(1, "Key is required."),
      value: z.string().min(1, "Value is required."),
    })
  )
  .optional()
  .transform((arr) =>
    arr?.reduce((acc, { key, value }) => {
      acc[key] = value;
      return acc;
    }, {} as Record<string, string>)
  );
