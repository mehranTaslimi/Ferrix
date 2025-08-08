import { z } from "zod";
import { urlSchema } from "./url";
import { chunkSchema } from "./chunk";
import { filePathSchema } from "./file-path";
import { positiveNumberSchema } from "./positive-number";
import { headersArraySchema } from "./headers";
import { cookiesArraySchema } from "./cookies";
import { proxySchema } from "./proxy";
import { authSchema } from "./auth";

export const downloadFormSchema = z
  .object({
    proxy: proxySchema,
    auth: authSchema,
    url: urlSchema,
    chunk: chunkSchema,
    filePath: filePathSchema,
    headers: headersArraySchema,
    cookies: cookiesArraySchema,
    speedLimit: positiveNumberSchema({
      message: "Speed limit must be a positive number",
    }),
    maxRetries: positiveNumberSchema({
      message: "Max retries must be a positive number",
    }),
    backoffFactor: positiveNumberSchema({
      message: "Backoff factor must be a positive number",
    }),
    timeoutSecs: positiveNumberSchema({
      message: "Timeout duration must be a positive number",
    }),
  })
  .refine((data) => {
    return true;
  });
