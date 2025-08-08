import { z } from "zod";

export const proxySchema = z
  .object({
    enabled: z.boolean(),
    type: z.enum(["http", "https", "socks5"]),
    host: z
      .string()
      .min(1, "Proxy host is required")
      .max(255, "Host too long")
      .regex(/^[a-zA-Z0-9.-]+$/, "Invalid hostname or IP address"),
    port: z
      .number()
      .int("Port must be an integer")
      .min(1, "Port must be 1-65535")
      .max(65535, "Port must be 1-65535"),
    auth: z
      .object({
        username: z.string().min(1, "Username required").max(100),
        password: z.string().min(1, "Password required").max(100),
      })
      .optional(),
  })
  .refine(
    (data) => {
      if (data.auth?.username && !data.auth.password) return false;
      return true;
    },
    {
      message: "Password is required when username is provided",
      path: ["auth", "password"],
    }
  );
