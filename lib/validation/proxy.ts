import { z } from "zod";

export const proxySchema = z
  .object({
    enabled: z.boolean().default(false),
    type: z.enum(["http", "https", "socks5"]),
    host: z
      .string()
      .min(1, "Proxy host is required")
      .max(255, "Host too long")
      .regex(
        /^(([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])\.)*([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9-]*[A-Za-z0-9])$|^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])$/,
        "Invalid hostname or IP address"
      ),

    port: z
      .number()
      .int("Port must be an integer")
      .min(1, "Port must be 1-65535")
      .max(65535, "Port must be 1-65535"),

    auth: z.object({
      username: z.string().max(100).optional(),
      password: z.string().max(100).optional(),
    }),
  })
  .partial()
  .superRefine((data, ctx) => {
    if (data.enabled) {
      if (!data.type) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "Proxy type is required",
          path: ["type"],
        });
      }
      if (!data.host) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "Proxy host is required",
          path: ["host"],
        });
      }
      if (data.port === undefined) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "Proxy port is required",
          path: ["port"],
        });
      }
    }

    if (data.auth?.username && !data.auth.password) {
      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        message: "Password is required when username is provided",
        path: ["auth", "password"],
      });
    }

    if (data.auth?.password && !data.auth.username) {
      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        message: "Username is required when password is provided",
        path: ["auth", "username"],
      });
    }
  })
  .optional();
