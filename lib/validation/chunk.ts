import { z } from "zod";

export const chunkSchema = z.coerce.number().refine((val) => val > 0, {
  message: "Chunk size must be more than 1.",
});
