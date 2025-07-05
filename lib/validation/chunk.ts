import { z } from "zod";

export const chunkSchema = z
  .number()
  .refine((val) => val === undefined || val > 0, {
    message: "Chunk size must be more than 1",
  });
