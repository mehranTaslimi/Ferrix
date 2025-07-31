import { z } from "zod";

const filePathRegex = /^([a-zA-Z]:)?(\/|\\)?([^<>:"|?*\n]+(\/|\\)?)*$/;

export const filePathSchema = z
  .string()
  .regex(filePathRegex, "Invalid file path format")
  .optional();
